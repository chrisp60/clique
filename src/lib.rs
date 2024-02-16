#![allow(dead_code)]
#![feature(fs_try_exists)]

mod user;

use axum::{
    extract::{
        ws::{self, WebSocket},
        State, WebSocketUpgrade,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use tokio::{net::TcpListener, sync::broadcast};
use tower_http::services::fs::ServeDir;
use tracing::{error, info, instrument};
use user::UserSet;

const ADDR: &str = env!("ADDR");
const BROADCAST_CAP: usize = 100;
const DIST_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/client/dist");

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct Error(#[from] eyre::Report);

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
struct AppState {
    users: UserSet,
    tx: MessageTx,
}

#[derive(Clone, Debug, Default)]
struct Message {}

#[derive(Clone, Debug)]
struct MessageTx(broadcast::Sender<Message>);

impl MessageTx {
    fn new() -> Self {
        Self(broadcast::channel(BROADCAST_CAP).0)
    }
}

impl Default for MessageTx {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct Server {
    state: AppState,
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

impl Server {
    pub fn new() -> Self {
        Self {
            state: AppState {
                users: UserSet::new(),
                tx: MessageTx::new(),
            },
        }
    }

    #[instrument(skip(self))]
    pub async fn run(self) -> eyre::Result<()> {
        let tcp_listener = TcpListener::bind(ADDR).await?;
        info!("listening on {}", ADDR);
        axum::serve(
            tcp_listener,
            Self::router().with_state(self.state).into_make_service(),
        )
        .await?;
        Ok(())
    }

    fn router() -> Router<AppState> {
        let fs = ServeDir::new(DIST_DIR);
        let path = format!("{DIST_DIR}/index.html");
        let exists = std::fs::try_exists(&path).unwrap_or_default();
        assert!(exists, "{path:?} does not exist");
        Router::new()
            .nest_service("/", fs)
            .route("/ws", get(Get::ws))
    }
}

struct Get;

impl Get {
    pub async fn root() -> String {
        "Hello, World!".to_string()
    }

    #[instrument(skip_all)]
    pub async fn ws(
        ws: WebSocketUpgrade,
        State(state): State<AppState>,
    ) -> crate::Result<impl IntoResponse> {
        info!("received websocket upgrade request");
        Ok(ws.on_upgrade(|socket| handle_ws_upgrade(socket, state)))
    }
}

#[instrument(skip_all)]
async fn handle_ws_upgrade(ws: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = ws.split();
    let user = state.users.create_one().await;

    let welcome = ws::Message::Text(user.motto().to_string());
    if let Err(err) = sender.send(welcome).await {
        error!(?err);
        return;
    }

    while let Some(Ok(message)) = receiver.next().await {
        match message {
            ws::Message::Text(text) => {
                info!(text);
                println!("{}", text);
            }
            ws::Message::Binary(bytes) => {
                info!(?bytes);
            }
            ws::Message::Ping(_) => {
                info!("received ping")
            }
            ws::Message::Pong(_) => {
                info!("received pong")
            }
            ws::Message::Close(close) => {
                info!("received close: {close:?}");
                break;
            }
        }
    }
}

//! Wasm frontend using Leptos
//!
//! ```not_rust
//! trunk build --release
//! ```

use gloo_net::websocket::futures::WebSocket;
use leptos::{leptos_dom::logging::console_log, *};

/// Websocket address.
const WS_ADDR: &str = concat!("ws://", env!("ADDR"), "ws");

/// Local error wrapper.
#[derive(thiserror::Error, Debug)]
#[error(transparent)]
struct Error(#[from] eyre::Report);

/// Allows `?` to be used in [`component`] functions.
impl IntoView for Error {
    fn into_view(self) -> View {
        view! {
            <p>
                <pre>{self.to_string()}</pre>
            </p>
        }
        .into_view()
    }
}

/// Local Result type.
pub(crate) type Result<T> = std::result::Result<T, Error>;

#[component]
#[allow(unused_variables)]
fn App(ws: WebSocket) -> impl IntoView {
    todo!("the application")
}

fn main() -> Result<()> {
    match WebSocket::open(WS_ADDR) {
        Ok(ws) => {
            leptos::mount_to_body(move || view! { <App ws/> });
        }
        Err(err) => {
            console_log(&err.to_string());
            todo!("show an pretty error");
        }
    }
    Ok(())
}

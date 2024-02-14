//! Wasm frontend using Leptos
//!
//! ```not_rust
//! trunk build --release
//! ```

use gloo_net::websocket::futures::WebSocket;
use leptos::*;
use web_sys::WebSocket;

const WS_ADDR: &str = concat!("ws://", env!("ADDR"), "ws");

/// Just tricking rustc a bit.
#[allow(non_snake_case)]
fn Ok<T>(t: T) -> Result<T> {
    std::result::Result::Ok(t)
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
struct Error(#[from] eyre::Report);

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

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    leptos::mount_to_body(App);
    Ok(())
}

#[component]
fn App() -> impl IntoView {
    let (read, write) = create_signal(WebSocket::open(WS_ADDR).unwrap());
}

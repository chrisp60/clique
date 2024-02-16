//! Wasm frontend using Leptos
//!
//! ```not_rust
//! trunk build --release
//! ```

use leptos::*;
#[allow(unused_imports)]
use tracing::{debug, error, info, log, trace, warn};
use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast, UnwrapThrowExt},
    MessageEvent, WebSocket,
};

/// Websocket address.
const WS_ADDR: &str = concat!("ws://", env!("ADDR"), "/ws");

#[component]
fn App() -> impl IntoView {
    let websocket = WebSocket::new(WS_ADDR).unwrap_throw();
    let incoming_messages = RwSignal::<Option<String>>::default();
    let welcome_message = RwSignal::<Option<String>>::default();
    let client_text = RwSignal::<String>::default();

    let onmessage_signal = move |value: String| {
        if welcome_message.with(Option::is_none) {
            welcome_message.update(|buf| {
                *buf = Some(value);
            })
        } else {
            incoming_messages.update(|inc_msg| {
                *inc_msg = Some(value);
            })
        }
    };

    let onmessage = Closure::wrap(Box::new(move |msg_event: MessageEvent| {
        let Some(msg_string) = msg_event.data().as_string() else {
            error!("message data could not be cast to string");
            return;
        };
        onmessage_signal(msg_string);
    }) as Box<dyn FnMut(_)>);
    let onmessage_ref = onmessage.as_ref().unchecked_ref();
    websocket.set_onmessage(Some(onmessage_ref));
    onmessage.forget();

    let stored_ws = store_value(websocket);

    on_cleanup(move || {
        _ = stored_ws.get_value().close();
    });

    Effect::new(move |_| {
        let value = client_text.get();
        let ws = stored_ws.get_value();
        if ws.ready_state() != 1 {
            return;
        }
        if let Err(err) = ws.send_with_str(&value) {
            error!("websocket send error: {err:?}");
        }
    });

    let on_input = move |ev| {
        client_text.update(|inner| *inner = event_target_value(&ev));
    };

    view! {
        <input type="text" on:input=on_input/>
        <p>"frontend: " {client_text}</p>
        <p>"backed: " {incoming_messages}</p>
    }
    .into_view()
}

fn main() {
    tracing_wasm::set_as_global_default();
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}

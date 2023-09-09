use serde::Deserialize;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};
#[derive(Deserialize, Debug)]
struct Player {
    x: [f64; 10],
    y: [f64; 10],
    health: [f64; 10],
    team: [i32; 10],
    dormant: [i32; 10],
}
// Define macro for 'console_log' that functions like 'println!'
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
// Bindings for `console.log` manually
#[wasm_bindgen]
extern "C" {
    // 'js_namespace' used to bind 'console.log(...)' instead of
    // 'log(...)'
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[wasm_bindgen]
pub fn websocket(url: &str) -> Result<(), JsValue> {
    // Create WebSocket connection.
    let ws = WebSocket::new(url)?;

    // Listen for incoming test messages
    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            let txt_str = txt.as_string().unwrap();
            console_log!("message event, received Text");
            // Process received message
            let parsed_message: Result<Player, serde_json::Error> = serde_json::from_str(&txt_str);
            if let Ok(player) = parsed_message {
                console_log!("Received player info: {:?}", player);
            } else if let Err(err) = parsed_message {
                console_log!("Error parsing JSON: {:?}", err);
            }
        } else {
            console_log!("message event, received Unknown: {:?}", e.data());
        }
    });
    // set message event handler on WebSocket
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();

    let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
        console_log!("error event: {:?}", e);
    });
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    // Display when the WebSocket has been opened
    let cloned_ws = ws.clone();
    let onopen_callback = Closure::<dyn FnMut()>::new(move || {
        console_log!("socket opened");
        match cloned_ws.send_with_str("ping") {
            Ok(_) => console_log!("message successfully sent"),
            Err(err) => console_log!("error sending message: {:?}", err),
        }
        // send off binary message
        match cloned_ws.send_with_u8_array(&[0, 1, 2, 3]) {
            Ok(_) => console_log!("binary message successfully sent"),
            Err(err) => console_log!("error sending message: {:?}", err),
        }
    });
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    Ok(())
}

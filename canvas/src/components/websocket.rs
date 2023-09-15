use super::canvas::{clear_and_redraw, draw_players};
use super::macros::{console_log, log};
use serde::Deserialize;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

/// A player is a struct that contains the position, health, team, and dormant status of a player
#[derive(Deserialize, Debug)]
pub struct Player {
    pub x: [f64; 10],
    pub y: [f64; 10],
    pub health: [f64; 10],
    pub team: [i32; 10],
    pub dormant: [i32; 10],
    pub rotation: [f64; 10],
    pub scoped: [i32; 10],
}
/// A macro to provide `println!(..)`-style syntax for `console.log` logging.
/// # Example
/// ```
/// use crate::components::websocket::console_log;
/// console_log!("Hello {}!", "world");
/// ```
/// A web socket connection to the server
/// # Arguments
/// * `url` - The url to connect to the server
/// # Example
/// ```
/// use super::websocket::websocket;
/// let _ws = websocket("ws://localhost:27017");
/// ```
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
                clear_and_redraw();
                draw_players(player);
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

mod components;

use wasm_bindgen::prelude::*;
use crate::components::websocket::websocket;

#[wasm_bindgen(start)]
pub fn start() {
    let ws= websocket("ws://localhost:27017");
}
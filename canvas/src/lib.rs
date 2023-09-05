mod components;

use wasm_bindgen::prelude::*;
use crate::components::open_websocket::open_websocket;

#[wasm_bindgen(start)]
pub fn start() {
    open_websocket();
}
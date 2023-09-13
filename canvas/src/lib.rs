mod components;

use crate::components::canvas::clear_and_redraw;
use crate::components::websocket::websocket;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    let _ws = websocket("ws://localhost:27017");
    clear_and_redraw();
}

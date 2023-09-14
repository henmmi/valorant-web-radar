mod components;
use crate::components::canvas::{activate_rotate, clear_and_redraw};
use crate::components::websocket::websocket;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    clear_and_redraw();
    activate_rotate();
    let _ws = websocket("ws://localhost:27017");
}

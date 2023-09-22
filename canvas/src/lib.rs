mod components;
use crate::components::canvas::initialise_interface;
use crate::components::websocket::websocket;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    initialise_interface();
    let _ws = websocket("ws://localhost:27017");
}

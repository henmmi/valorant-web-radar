mod components;
use crate::components::canvas::initialise_interface;
use crate::components::game_data::Preloader;
use crate::components::websocket::websocket;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    let mut preloader = Preloader::new();
    preloader.preload_agents("agent");
    preloader.preload_maps("map");
    initialise_interface();
    let _ws = websocket("ws://localhost:27017");
}

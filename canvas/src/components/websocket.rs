use super::canvas::clear_and_refresh;
use super::game_data::GameInfo;
use super::macros::{console_log, log};
use super::player_data::{Player, Players};
use crate::components::game_data::GameScore;
use crate::components::player::draw_players;
use crate::components::player_table::create_player_info_row;
use crate::components::ui_element::{
    get_player_dropdown_length, player_dropdown, toggle_orientation,
};
use serde::Deserialize;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

#[derive(Deserialize, Debug)]
pub struct Data {
    players: Players,
    game_info: GameInfo,
}
/// A macro to provide `println!(..)`-style syntax for `console.log` logging.
/// # Example
/// ```
/// use super::macros::{console_log, log};
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
            match serde_json::from_str::<Data>(&txt_str) {
                Ok(game_data) => {
                    let mut game_info = game_data.game_info;
                    let score = GameScore {
                        t_score: game_info.t_score.pop().unwrap(),
                        ct_score: game_info.ct_score.pop().unwrap(),
                    };
                    let player_data = game_data.players;
                    let mut players: Vec<Player> = Vec::new();
                    // Push the player data into a vector of players
                    for i in 0..player_data.x.len() {
                        players.push(Player {
                            id: player_data.id[i],
                            x: player_data.x[i],
                            y: player_data.y[i],
                            health: player_data.health[i],
                            team: player_data.team[i],
                            dormant: player_data.dormant[i],
                            rotation: player_data.rotation[i],
                            scoped: player_data.scoped[i],
                            weapon: player_data.weapon[i],
                            kill: player_data.kill[i],
                            death: player_data.death[i],
                            assist: player_data.assist[i],
                            acs: player_data.acs[i],
                            shield: player_data.shield[i],
                            credits: player_data.credits[i],
                        });
                    }
                    players.reverse();
                    clear_and_refresh();
                    toggle_orientation(&players);
                    draw_players(&players);
                    create_player_info_row(&players, &score);
                    // Check if current dropdown length is equal to the number of players
                    if get_player_dropdown_length() != players.len() {
                        // If not, update the dropdown
                        player_dropdown(&players.len());
                    };
                }
                Err(err) => console_log!("Error parsing JSON: {:?}", err),
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

pub fn get_hostname() -> String {
    let window = web_sys::window().unwrap();
    let location = window.location();
    let hostname = location.hostname().unwrap();
    console_log!("Hostname: {}", hostname);
    hostname
}

pub fn get_host() -> String {
    let window = web_sys::window().unwrap();
    let location = window.location();
    let host = location.host().unwrap();
    console_log!("Host: {}", host);
    host
}

use wasm_bindgen::prelude::*;
use yew::prelude::*;
//use web_sys::*;
use yew::Properties;

#[derive(Properties, Clone, PartialEq)]
pub struct PlayerProps {
    pub player: Player,
}

#[derive(Clone, PartialEq)]
pub struct Player {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub colour: String,
}

#[function_component(PlayerComponent)]
pub fn player_component(props: &PlayerProps) -> Html {
    let player = &props.player;
    html! {
        <g>
            <circle cx={player.x.to_string()} cy={player.y.to_string()} r="5" fill={player.colour.clone()}/>
            <text x={player.x.to_string()} y={player.y.to_string()} fill="white" font-family="Arial" font-size="10">
                {player.id.to_string()}
            </text>
        </g>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let _player1 = generate_player();
    html! {
        <svg id="map" width="500" height="500">
            <image id="ascent" href="maps/Ascent.png" width="500" height="500"/>
        </svg>
    }
}

fn generate_player() -> Player {
    Player {
        id: 1,
        x: 250,
        y: 250,
        colour: "red".to_string(),
    }
}

// Here's the added code to initialize the Yew app using wasm-bindgen.
#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
use crate::components::websocket::Player;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

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
/// Getters for the canvas, context, and document
/// # Returns
/// * `canvas` - The canvas element
/// * `context` - The canvas context
/// * `document` - The document
/// # Example
/// ```
/// use crate::components::canvas::get_canvas_context_document;
/// let (canvas, context, document) = get_canvas_context_document();
/// ```
fn get_canvas_context_document() -> (
    HtmlCanvasElement,
    web_sys::CanvasRenderingContext2d,
    web_sys::Document,
) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    (canvas, context, document)
}
/// Clear the canvas and redraw the map
/// # Example
/// ```
/// use crate::components::canvas::clear_and_redraw;
/// clear_and_redraw();
/// ```
#[wasm_bindgen]
pub fn clear_and_redraw() {
    let (_, context, document) = get_canvas_context_document();

    context.clear_rect(0.0, 0.0, 1024.0, 1024.0);
    console_log!("Cleared canvas");

    let image = document
        .create_element("img")
        .unwrap()
        .dyn_into::<web_sys::HtmlImageElement>()
        .unwrap();
    image.set_id("map");
    image.set_src("http://127.0.0.1:8080/images/Ascent-391657b8f8b973aa5d90.png");
    context
        .draw_image_with_html_image_element(&image, 0.0, 0.0)
        .unwrap();
}
/// Display the player's position on the canvas
/// # Arguments
/// * `id` - The player's ID
/// * `team` - The player's team
/// * `x` - The player's X coordinate
/// * `y` - The player's Y coordinate
/// # Example
/// ```
/// use crate::components::canvas::display_player_position;
/// display_player_position(0, 0, 100.0, 100.0);
/// ```
/// # Note
/// * `team` is an integer, where 0 is red and 1 is blue
#[wasm_bindgen()]
pub fn display_player_position(id: usize, team: i32, x: f64, y: f64) {
    let (_, context, _) = get_canvas_context_document();

    // Determine team colour
    let team_id = match team {
        0 => "red",
        1 => "blue",
        _ => "black",
    };

    context.begin_path();
    context.arc(x, y, 10.0, 0.0, f64::consts::PI * 2.0).unwrap();
    context.set_fill_style(&JsValue::from_str(team_id));
    context.fill();

    // Draw the circle's outline in white
    context.set_stroke_style(&JsValue::from_str("white"));
    context.stroke();

    // Configure the text's style
    context.set_font("16px sans-serif");
    context.set_text_align("center");
    context.set_text_baseline("middle");
    context.set_fill_style(&JsValue::from_str("white"));
    context.fill_text(&id.to_string(), x, y).unwrap();
}
/// Draw the players on the canvas
/// # Arguments
/// * `player` - Input player data through the struct 'Player'
/// # Example
/// ```
/// use crate::components::canvas::draw_players;
/// use crate::components::websocket::Player;
/// let player = Player {
///    x: [0.0; 10],
///    y: [0.0; 10],
///    health: [0.0; 10],
///    team: [0; 10],
///    dormant: [0; 10],
/// };
/// draw_players(player);
/// ```
pub fn draw_players(player: Player) {
    for i in 0..10 {
        console_log!("Player {} is at ({}, {})", i, player.x[i], player.y[i]);
        display_player_position(i, player.team[i], player.x[i], player.y[i]);
    }
}

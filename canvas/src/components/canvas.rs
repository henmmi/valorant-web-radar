use super::macros::{console_log, log};
use super::websocket::Player;
use std::f64;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlCanvasElement};
/// Getters for the canvas, context, and document
/// # Returns
/// * `canvas` - The canvas element
/// * `context` - The canvas context
/// * `document` - The document
/// # Example
/// ```
/// let (canvas, context, document) = get_canvas_context_document();
/// ```
fn get_canvas_context_document() -> (
    HtmlCanvasElement,
    web_sys::CanvasRenderingContext2d,
    web_sys::Document,
) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
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
/// use super::macros::{console_log, log};
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
/// display_player_position(0, 0, 100.0, 100.0);
/// ```
/// # Note
/// * `team` is an integer, where 0 is red and 1 is blue
#[wasm_bindgen]
pub fn display_player_position(id: usize, team: i32, x: f64, y: f64) {
    let (_, context, _) = get_canvas_context_document();

    // Determine team colour
    let team_id = match team {
        0 => "red",
        1 => "blue",
        _ => "black",
    };
    context.save();
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
    context.restore();
}
/// Draw the players on the canvas
/// # Arguments
/// * `player` - Input player data through the struct 'Player'
/// # Example
/// ```
/// use super::macros::{console_log, log};
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
/// Activate the rotate button
/// # Arguments
/// * `deg` - The degree to rotate the canvas by
/// # Example
/// ```
/// activate_rotate(90f64);
/// ```
#[wasm_bindgen]
pub fn activate_rotate(deg: f64) {
    let (_, _, document) = get_canvas_context_document();
    let rotate_btn = document
        .create_element("button")
        .unwrap()
        .dyn_into::<HtmlButtonElement>()
        .unwrap();
    let deg_str = deg.to_string();
    rotate_btn.set_text_content(Some(deg_str.as_str()));
    rotate_btn.set_id(deg_str.as_str());
    document.body().unwrap().append_child(&rotate_btn).unwrap();

    let rotate_canvas = Closure::wrap(Box::new(move || {
        rotate_canvas(deg);
    }) as Box<dyn FnMut()>);

    rotate_btn.set_onclick(Some(rotate_canvas.as_ref().unchecked_ref()));
    rotate_canvas.forget();
}
/// Rotate the canvas
/// # Arguments
/// * `deg` - The degree to rotate the canvas by
/// # Example
/// ```
/// use super::macros::{console_log, log};
/// rotate_canvas(90f64);
/// ```
#[wasm_bindgen]
pub fn rotate_canvas(deg: f64) {
    let (_, context, _) = get_canvas_context_document();
    let (width, height) = get_canvas_width_height();
    context.translate(width / 2f64, height / 2f64).unwrap();
    console_log!("Translated canvas to set origin");

    let angle = get_radian_angle(deg);
    context.rotate(angle).unwrap();
    console_log!("Rotating canvas by {} degrees", deg);

    context.translate(-width / 2f64, -height / 2f64).unwrap();
    console_log!("Translated canvas to reset origin");
}
/// Convert degrees to radians
/// # Arguments
/// * `deg` - The degree to convert to radians
/// # Example
/// ```
/// let rad = get_radian_angle(90f64);
/// ```
fn get_radian_angle(deg: f64) -> f64 {
    deg * f64::consts::PI / 180.0
}

fn get_canvas_width_height() -> (f64, f64) {
    let (canvas, _, _) = get_canvas_context_document();
    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    (width, height)
}

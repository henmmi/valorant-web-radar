use super::macros::{console_log, log};
use super::websocket::Player;
use js_sys::Math::{cos, sin};
use lazy_static::lazy_static;
use std::f64;
use std::sync::RwLock;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlCanvasElement};

// Create a global variable to store the rotation angle
lazy_static! {
    static ref ROTATION_ANGLE: RwLock<f64> = RwLock::new(0.0);
}
/// Setter for the rotation angle used to remember the rotation angle
/// # Arguments
/// * `lock` - The RwLock to store the rotation angle
/// * `float` - The float to set the rotation angle to
/// # Example
/// ```
/// let lock = RwLock::new(0.0);
/// update_it(&lock, 90.0);
/// ```
fn update_it(lock: &RwLock<f64>, float: f64) {
    let mut w = lock.write().unwrap();
    console_log!("Updating rotation angle to {}", float);
    *w += float;
}
/// Getter for the rotation angle
/// # Arguments
/// * `lock` - The RwLock to store the rotation angle
/// # Example
/// ```
/// let lock = RwLock::new(0.0);
/// let angle = get_number(&lock);
/// ```
/// # Returns
/// * `f64` - The rotation angle
fn get_number(lock: &RwLock<f64>) -> f64 {
    let r1 = lock.read().unwrap();
    *r1
}
/// Getters for the canvas, context, and document
/// # Returns
/// * `canvas` - The canvas element
/// * `context` - The canvas context
/// * `document` - The document
/// # Example
/// ```
/// let (canvas, context, document) = get_canvas_context_document();
/// ```
pub fn get_canvas_context_document() -> (
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
fn identify_team(team: i32) -> &'static str {
    match team {
        0 => "red",
        1 => "blue",
        _ => "black",
    }
}
/// Display the player's position on the canvas
/// # Arguments
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
pub fn display_player_position(x: f64, y: f64, team: i32) {
    let (_, context, _) = get_canvas_context_document();
    let team_id = identify_team(team);
    context.begin_path();
    context.arc(x, y, 10.0, 0.0, f64::consts::PI * 2.0).unwrap();
    context.set_fill_style(&JsValue::from_str(team_id));
    context.fill();
    // Draw the circle's outline in white
    context.set_stroke_style(&JsValue::from_str("white"));
    context.stroke();
}
/// Draw the player's label on the canvas
/// # Arguments
/// * `id` - The player's ID
/// * `x` - The player's X coordinate
/// * `y` - The player's Y coordinate
/// * `angle` - The player's angle
/// # Example
/// ```
/// draw_player_labels(0, 100.0, 100.0, 90.0);
/// ```
#[wasm_bindgen]
pub fn draw_player_labels(id: usize, x: f64, y: f64, angle: f64) {
    let (_, context, _) = get_canvas_context_document();
    // Configure the text's style
    context.set_font("16px sans-serif");
    context.set_text_align("center");
    context.set_text_baseline("middle");
    context.set_fill_style(&JsValue::from_str("white"));
    if angle != 0.0f64 {
        context.save();
        context.translate(x, y).unwrap();
        let angle_rad = get_radian_angle(-angle);
        context.rotate(angle_rad).unwrap();
        context.fill_text(&id.to_string(), 0.0, 0.0).unwrap();
        context.restore();
    } else {
        context.fill_text(&id.to_string(), x, y).unwrap();
    }
}
/// Draw the player's label on the canvas
/// # Arguments
/// * `players` - The player's data through the struct 'Player' in a vector
/// # Example
/// ```
/// draw_players(&[Player]);
/// ```
pub fn draw_players(players: &[Player]) {
    for (i, player) in players.iter().enumerate() {
        console_log!("Player {} is at ({}, {})", i, player.x, player.y);
        draw_player_orientation(player);
        display_player_position(player.x, player.y, player.team);
        draw_player_labels(i, player.x, player.y, get_number(&ROTATION_ANGLE));
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
    rotate_btn.set_class_name("rotate-btn");
    document.body().unwrap().append_child(&rotate_btn).unwrap();

    let rotate_canvas = Closure::wrap(Box::new(move || {
        rotate_canvas(deg);
        update_it(&ROTATION_ANGLE, deg);
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
    clear_and_redraw();
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

/// Get the canvas width and height
/// # Returns
/// * `width` - The canvas width
/// * `height` - The canvas height
/// # Example
/// ```
/// let (width, height) = get_canvas_width_height();
/// ```
fn get_canvas_width_height() -> (f64, f64) {
    let (canvas, _, _) = get_canvas_context_document();
    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    (width, height)
}
/// Draw the player's orientation on the canvas via a line
/// And extend the line if the player is scoped
/// # Arguments
/// * `player` - Input player data through the struct 'Player'
/// # Example
/// ```
/// draw_player_orientation(&player);
/// ```
// create a function "draw_player_orientation" to depict the player rotation via a visible line extending from center of player icon
pub fn draw_player_orientation(player: &Player) {
    let (_, context, _) = get_canvas_context_document();

    // Determine team colour
    let team_id = match player.team {
        0 => "red",
        1 => "blue",
        _ => "black",
    };
    // Angle in radians
    let angle = get_radian_angle(player.rotation);
    let mut view_line_size = 30f64;
    // If scoped, increase the line size by 20 pixels
    if player.scoped == 1 {
        view_line_size += 20f64;
    }
    let x_line = view_line_size * cos(angle);
    let y_line = view_line_size * sin(angle);
    context.save();
    context.begin_path();
    context.translate(player.x, player.y).unwrap();
    context.move_to(0.0, 0.0);
    context.set_stroke_style(&JsValue::from_str(team_id));
    context.line_to(x_line, y_line);
    context.set_line_width(3.0);
    context.stroke();
    context.restore();
}

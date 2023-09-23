use super::macros::{console_log, log};
use crate::components::ui_element;
use lazy_static::lazy_static;
use std::f64;
use std::rc::Rc;
use std::sync::RwLock;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

/// Generates the canvas and user interface
/// # Example
/// ```
/// initialise_interface();
/// ```
pub fn initialise_interface() {
    clear_and_refresh();
    ui_element::reset_button();
    activate_rotate(90f64);
    activate_rotate(180f64);
    activate_rotate(-90f64);
    activate_rotate(-180f64);
    ui_element::create_toggle("orientation_toggle", "player_interact");
    ui_element::create_select("player_dropdown");
    ui_element::create_toggle("label_toggle", "player_label");
}
// Global variable to store the rotation angle of the canvas
lazy_static! {
    pub static ref ROTATION_ANGLE: RwLock<f64> = RwLock::new(0.0);
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
pub fn update_it(lock: &RwLock<f64>, float: f64) {
    let mut w = lock.write().unwrap();
    console_log!("Updating rotation angle to {}", float);
    *w += float;
}

/// Change the rotation angle
/// # Arguments
/// * `float` - The float to set the rotation angle to
/// # Example
/// ```
/// change_rotation_angle(90.0);
/// ```
pub fn change_it(lock: &RwLock<f64>, float: f64) {
    let mut w = lock.write().unwrap();
    console_log!("Changing rotation angle to {}", float);
    *w = float;
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
pub fn get_number(lock: &RwLock<f64>) -> f64 {
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
pub fn clear_and_refresh() {
    let (_, context, document) = get_canvas_context_document();
    context.save();
    // Reset the transform to clear the canvas
    context.reset_transform().unwrap();
    context.clear_rect(0.0, 0.0, 1024.0, 1024.0);
    context.restore();
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
#[wasm_bindgen]
pub fn reset_canvas() {
    let (_, context, document) = get_canvas_context_document();
    // Reset the transform to clear the canvas
    context.reset_transform().unwrap();
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

    change_it(&ROTATION_ANGLE, 0.0);
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
    let deg_clone = Rc::new(deg);
    ui_element::onclick_button(
        Box::new(move || {
            rotate_canvas(*deg_clone);
            update_it(&ROTATION_ANGLE, deg);
        }),
        deg.to_string().as_str(),
    );
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
    clear_and_refresh();
}

/// Convert degrees to radians
/// # Arguments
/// * `deg` - The degree to convert to radians
/// # Example
/// ```
/// let rad = get_radian_angle(90f64);
/// ```
pub fn get_radian_angle(deg: f64) -> f64 {
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
pub fn get_canvas_width_height() -> (f64, f64) {
    let (canvas, _, _) = get_canvas_context_document();
    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    (width, height)
}

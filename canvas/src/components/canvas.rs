use super::macros::{console_log, log};
use crate::components::elements::get_html_image_element_by_id;
use crate::components::{elements, ui_element};
use lazy_static::lazy_static;
use std::f64;
use std::rc::Rc;
use std::sync::RwLock;
use wasm_bindgen::prelude::*;

/// Generates the canvas and user interface
/// # Example
/// ```
/// initialise_interface();
/// ```
pub fn initialise_interface() {
    ui_element::reset_button();
    activate_rotate(90f64);
    activate_rotate(180f64);
    activate_rotate(-90f64);
    activate_rotate(-180f64);
    ui_element::create_toggle("orientation_toggle", "player_interact");
    ui_element::create_select("player_dropdown");
    ui_element::create_toggle("label_toggle", "player_label");
    clear_and_refresh();
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

/// Clear the canvas and redraw the map
/// # Example
/// ```
/// clear_and_redraw();
/// ```
#[wasm_bindgen]
pub fn clear_and_refresh() {
    let (_, context, _) = elements::get_canvas_context_document();
    context.save();
    // Reset the transform to clear the canvas
    if let Err(err) = context.reset_transform() {
        console_log!("Error resetting transform: {:?}", err)
    };
    context.clear_rect(0.0, 0.0, 1024.0, 1024.0);
    context.restore();
    console_log!("Cleared canvas");

    match get_html_image_element_by_id("Ascent") {
        Ok(image) => {
            if let Err(err) = context.draw_image_with_html_image_element(&image, 0.0, 0.0) {
                console_log!("Error drawing image: {:?}", err)
            };
        }
        Err(err) => console_log!("Error getting image: {:?}", err),
    }
}
/// Reset the canvas
/// # Example
/// ```
/// reset_canvas();
/// ```
#[wasm_bindgen]
pub fn reset_canvas() {
    let (_, context, _) = elements::get_canvas_context_document();
    // Reset the transform to clear the canvas
    if let Err(err) = context.reset_transform() {
        console_log!("Error resetting transform: {:?}", err)
    };
    context.clear_rect(0.0, 0.0, 1024.0, 1024.0);
    console_log!("Cleared canvas");

    match get_html_image_element_by_id("Ascent") {
        Ok(image) => {
            if let Err(err) = context.draw_image_with_html_image_element(&image, 0.0, 0.0) {
                console_log!("Error drawing image: {:?}", err)
            };
        }
        Err(err) => console_log!("Error getting image: {:?}", err),
    }

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
    let (_, context, _) = elements::get_canvas_context_document();
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
    let (canvas, _, _) = elements::get_canvas_context_document();
    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    (width, height)
}

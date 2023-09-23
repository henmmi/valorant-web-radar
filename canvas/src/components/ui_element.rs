use super::macros::{console_log, log};
use crate::components::canvas;
use crate::components::canvas::{get_number, ROTATION_ANGLE};
use crate::components::player::draw_player_labels;
use crate::components::websocket::Player;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlButtonElement, HtmlDivElement, HtmlInputElement, HtmlLabelElement, HtmlOptionElement,
    HtmlSelectElement, HtmlSpanElement,
};

/// Create a HTML button
/// # Arguments
/// * `name` - The name of the button
/// # Example
/// ```
/// let btn = create_button("name");
/// ```
pub fn create_button(name: &str) -> HtmlButtonElement {
    let (_, _, document) = canvas::get_canvas_context_document();
    let btn = document
        .create_element("button")
        .unwrap()
        .dyn_into::<HtmlButtonElement>()
        .unwrap();
    btn.set_text_content(Some(name));
    btn.set_id(name);
    let button_list = document
        .get_element_by_id("button_row")
        .unwrap()
        .dyn_into::<HtmlDivElement>()
        .unwrap();
    button_list.append_child(&btn).unwrap();
    btn
}

/// Create a label
/// # Arguments
/// * `name` - The name of the label
/// # Example
/// ```
/// create_label("name");
/// ```
fn create_label(name: &str) -> HtmlLabelElement {
    let (_, _, document) = canvas::get_canvas_context_document();
    let follow_label = document
        .create_element("label")
        .unwrap()
        .dyn_into::<HtmlLabelElement>()
        .unwrap();

    follow_label.set_class_name(name);

    follow_label
}

/// Create a span
/// # Arguments
/// * `name` - The name of the span
/// # Example
/// ```
/// create_span("name");
/// ```
fn create_span(name: &str) -> HtmlSpanElement {
    let (_, _, document) = canvas::get_canvas_context_document();
    let follow_span = document
        .create_element("span")
        .unwrap()
        .dyn_into::<HtmlSpanElement>()
        .unwrap();
    follow_span.set_class_name(name);

    follow_span
}

/// Create a toggle button
/// # Arguments
/// * `name` - The name of the toggle button
/// # Example
/// ```
/// create_toggle("toggle_switch");
/// ```
pub fn create_toggle(name: &str, div_name: &str) {
    let checkbox = create_checkbox(name);

    let label = create_label("switch");
    let span_round = create_span("slider round");
    let (_, _, document) = canvas::get_canvas_context_document();
    let player_interact = document
        .get_element_by_id(div_name)
        .unwrap()
        .dyn_into::<HtmlDivElement>()
        .unwrap();

    player_interact.append_child(&label).unwrap();
    label.append_child(&checkbox).unwrap();
    label.append_child(&span_round).unwrap();
}

/// Create a select
/// # Arguments
/// * `name` - The name of the select
/// # Example
/// ```
/// create_select("name");
/// ```
pub fn create_select(name: &str) -> HtmlSelectElement {
    let (_, _, document) = canvas::get_canvas_context_document();
    let select = document
        .create_element("select")
        .unwrap()
        .dyn_into::<HtmlSelectElement>()
        .unwrap();
    select.set_id(name);
    select.set_name(name);
    let player_interact = document
        .get_element_by_id("player_interact")
        .unwrap()
        .dyn_into::<HtmlDivElement>()
        .unwrap();
    player_interact.append_child(&select).unwrap();
    select
}
/// Create an option
/// # Arguments
/// * `name` - The name of the option
/// # Example
/// ```
/// create_option("name");
/// ```
pub fn create_option(name: &str) -> HtmlOptionElement {
    let (_, _, document) = canvas::get_canvas_context_document();
    let option = document
        .create_element("option")
        .unwrap()
        .dyn_into::<HtmlOptionElement>()
        .unwrap();
    option.set_value(name);
    option.set_text_content(Some(name));
    option
}

/// Create a checkbox
/// # Arguments
/// * `name` - The name of the checkbox
/// # Example
/// ```
/// create_checkbox("name");
/// ```
fn create_checkbox(name: &str) -> HtmlInputElement {
    let (_, _, document) = canvas::get_canvas_context_document();
    let follow_checkbox = document
        .create_element("input")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    follow_checkbox.set_id(name);
    follow_checkbox.set_type("checkbox");

    follow_checkbox
}

/// Create a HTML button and set the onclick event
/// # Arguments
/// * `callback` - The callback function to execute when the button is clicked
/// * `name` - The name of the button
/// # Example
/// ```
/// let deg = 90f64;
/// onclick_button(Box::new(move || {
///   rotate_canvas(*deg);
///  update_it(&ROTATION_ANGLE, deg);
/// }), "string");
/// ```
pub fn onclick_button(mut callback: Box<dyn FnMut()>, name: &str) {
    let onclick_btn = create_button(name);
    let onclick_canvas = Closure::wrap(Box::new(move || {
        callback();
    }) as Box<dyn FnMut()>);

    onclick_btn.set_onclick(Some(onclick_canvas.as_ref().unchecked_ref()));
    onclick_canvas.forget();
}

/// Reset the canvas rotation
/// # Example
/// ```
/// reset_button();
/// ```
pub fn reset_button() {
    onclick_button(
        Box::new(|| {
            canvas::reset_canvas();
        }),
        "reset",
    );
}

/// Rotates the canvas when the toggle button is checked
/// Based on the player selected in the dropdown
/// # Arguments
/// * `players` - The player's data through the struct 'Player' in a vector
/// # Example
/// ```
/// on_toggle(&[Player]);
/// ```
pub fn on_toggle(players: &[Player]) {
    let (_, context, document) = canvas::get_canvas_context_document();
    let toggle_btn = document
        .get_element_by_id("orientation_toggle")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    if toggle_btn.checked() {
        let dropdown_value = get_player_dropdown();
        console_log!("Dropdown value: {}", dropdown_value);
        let rotation_angle = &players[dropdown_value].rotation;
        context.reset_transform().unwrap();
        canvas::rotate_canvas(*rotation_angle);
        canvas::change_it(&ROTATION_ANGLE, *rotation_angle);
    }
}
/// Detect if the toggle button is checked
/// # Arguments
/// * `players` - The player's data through the struct 'Player' in a vector
/// # Example
/// ```
/// toggle_label(&[Player]);
/// ```
pub fn toggle_label(players: &[Player]) {
    let (_, _, document) = canvas::get_canvas_context_document();
    let toggle_btn = document
        .get_element_by_id("label_toggle")
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    if toggle_btn.checked() {
        draw_player_labels(players, get_number(&ROTATION_ANGLE));
    }
}
/// Create the player dropdown
/// # Arguments
/// * `players` - The number of players in the game
/// # Example
/// ```
/// player_dropdown(&usize);
/// ```
pub fn player_dropdown(players: &usize) {
    let (_, _, document) = canvas::get_canvas_context_document();
    let player_list = document
        .get_element_by_id("player_dropdown")
        .unwrap()
        .dyn_into::<HtmlSelectElement>()
        .unwrap();
    // Clear the dropdown
    player_list.set_inner_html("");
    for player in 0..*players {
        let option = create_option(player.to_string().as_str());
        player_list.append_child(&option).unwrap();
    }
}

/// Get the player's dropdown value
/// # Example
/// ```
/// get_player_dropdown();
/// ```
pub fn get_player_dropdown() -> usize {
    let (_, _, document) = canvas::get_canvas_context_document();
    let player_dropdown = document
        .get_element_by_id("player_dropdown")
        .unwrap()
        .dyn_into::<HtmlSelectElement>()
        .unwrap();
    player_dropdown.value().parse::<usize>().unwrap()
}
/// Get the player's dropdown length
/// # Example
/// ```
/// get_player_dropdown_length();
/// ```
pub fn get_player_dropdown_length() -> usize {
    let (_, _, document) = canvas::get_canvas_context_document();
    let player_dropdown = document
        .get_element_by_id("player_dropdown")
        .unwrap()
        .dyn_into::<HtmlSelectElement>()
        .unwrap();
    player_dropdown.length() as usize
}

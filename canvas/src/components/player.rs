use super::macros::{console_log, log};
use crate::components::canvas;
use crate::components::canvas::{get_radian_angle, ROTATION_ANGLE};
use crate::components::websocket::Player;
use js_sys::Math::{cos, sin};
use std::f64;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

/// Draw the player's label on the canvas and rotate it
/// # Arguments
/// * `players` - The player's data through the struct 'Player' in a vector
/// # Example
/// ```
/// draw_players(&[Player]);
/// ```
pub fn draw_players(players: &[Player]) {
    for (i, player) in players.iter().enumerate() {
        draw_player_orientation(player);
        display_player_position(player.x, player.y, player.team);
        player_health_circle(player.x, player.y, player.health);
        draw_player_labels(i, player.x, player.y, canvas::get_number(&ROTATION_ANGLE));
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
    let (_, context, _) = canvas::get_canvas_context_document();
    let team_id = identify_team(team);
    context.begin_path();
    context.arc(x, y, 10.0, 0.0, f64::consts::PI * 2.0).unwrap();
    context.set_fill_style(&JsValue::from_str(team_id));
    context.fill();
    // Draw the circle's outline in white
    context.set_stroke_style(&JsValue::from_str("white"));
    context.stroke();
}

pub fn player_health_circle(x: f64, y: f64, health: f64) {
    let (_, context, _) = canvas::get_canvas_context_document();
    // TODO: If following player, use follow angle.
    // TODO:: Fix orientation of the player health circle.
    console_log!("Starting angle: {}", canvas::get_number(&ROTATION_ANGLE));
    context.save();
    context.reset_transform().unwrap();
    context.translate(x, y).unwrap();
    context.begin_path();
    context
        .arc(
            0.0,
            0.0,
            10.0,
            calculate_starting_fill_angle(health),
            calculate_ending_fill_angle(health),
        )
        .unwrap();
    context.restore();
    context.set_fill_style(&JsValue::from_str("green"));
    context.fill();
    context.stroke();
}

pub fn calculate_starting_fill_angle(health: f64) -> f64 {
    let starting_angle =
        get_radian_angle((89f64 - health * 1.8f64) + canvas::get_number(&ROTATION_ANGLE));
    starting_angle
}

pub fn calculate_ending_fill_angle(health: f64) -> f64 {
    let ending_angle =
        get_radian_angle((90f64 + health * 1.8f64) + canvas::get_number(&ROTATION_ANGLE));
    ending_angle
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
    let (_, context, _) = canvas::get_canvas_context_document();
    // Configure the text's style
    context.set_font("16px sans-serif");
    context.set_text_align("center");
    context.set_text_baseline("middle");
    context.set_fill_style(&JsValue::from_str("white"));
    if angle != 0.0f64 {
        context.save();
        context.translate(x, y).unwrap();
        let angle_rad = canvas::get_radian_angle(-angle);
        context.rotate(angle_rad).unwrap();
        context.fill_text(&id.to_string(), 0.0, 0.0).unwrap();
        context.restore();
    } else {
        context.fill_text(&id.to_string(), x, y).unwrap();
    }
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
fn draw_player_orientation(player: &Player) {
    let (_, context, _) = canvas::get_canvas_context_document();

    // Determine team colour
    let team_id = match player.team {
        0 => "red",
        1 => "blue",
        _ => "black",
    };
    // Angle in radians
    let angle = canvas::get_radian_angle(player.rotation);
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
/// Identify the player's team
/// # Arguments
/// * `team` - The player's team
/// # Example
/// ```
/// identify_team(0);
/// ```
fn identify_team(team: i32) -> &'static str {
    match team {
        0 => "red",
        1 => "blue",
        _ => "black",
    }
}

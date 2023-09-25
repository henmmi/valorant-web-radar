use super::macros::{console_log, log};
use super::player_data::Player;
use crate::components::canvas::{get_number, get_radian_angle, ROTATION_ANGLE};
use crate::components::element;
use crate::components::elements::create_html_image_element;
use crate::components::ui_element::toggle_label;
use js_sys::Math::{cos, sin};
use std::f64;
use wasm_bindgen::JsValue;

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
pub fn display_player_position(player: &Player) {
    let (_, context, _) = element::get_canvas_context_document();
    let team_id = identify_team(player.team, true);
    context.begin_path();
    context
        .arc(player.x, player.y, 10.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();
    context.set_fill_style(&JsValue::from_str(team_id));
    context.fill();
    player_health_circle(player, get_number(&ROTATION_ANGLE));
}
/// Draw the player's health circle on the canvas
/// # Arguments
/// * `player` - Input player data through the struct 'Player'
/// * `angle` - The player's angle
/// # Example
/// ```
/// player_health_circle(&player, 90.0);
/// ```
pub fn player_health_circle(player: &Player, angle: f64) {
    let (_, context, _) = element::get_canvas_context_document();
    context.save();
    context.translate(player.x, player.y).unwrap();
    let angle_rad = get_radian_angle(-angle);
    context.rotate(angle_rad).unwrap();
    context.begin_path();
    context
        .arc(
            0.0,
            0.0,
            10.25,
            calculate_starting_fill_angle(player.health),
            calculate_ending_fill_angle(player.health),
        )
        .unwrap();
    context.set_fill_style(&JsValue::from_str(identify_team(player.team, false)));
    context.fill();
    console_log!("Drew player health circle");
    context.restore();
}
/// Finds the start draw angle for the player's health circle based on their health
/// # Arguments
/// * `health` - The player's health
/// # Example
/// ```
/// calculate_starting_fill_angle(100.0);
/// ```
pub fn calculate_starting_fill_angle(health: f64) -> f64 {
    get_radian_angle(90f64 - health / 100f64 * 180f64)
}
/// Finds the end draw angle for the player's health circle based on their health
/// # Arguments
/// * `health` - The player's health
/// # Example
/// ```
/// calculate_ending_fill_angle(100.0);
/// ```
pub fn calculate_ending_fill_angle(health: f64) -> f64 {
    get_radian_angle(90f64 + health / 100f64 * 180f64)
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
pub fn draw_player_labels(player: &[Player], angle: f64) {
    let (_, context, _) = element::get_canvas_context_document();
    // Configure the text's style
    context.set_font("16px sans-serif");
    context.set_text_align("center");
    context.set_text_baseline("middle");
    context.set_fill_style(&JsValue::from_str("white"));
    for (_i, player) in player.iter().enumerate() {
        if angle != 0.0f64 {
            context.save();
            context.translate(player.x, player.y).unwrap();
            let angle_rad = get_radian_angle(-angle);
            context.rotate(angle_rad).unwrap();
            context.fill_text(&player.id.to_string(), 0.0, 0.0).unwrap();
            context.restore();
        } else {
            context
                .fill_text(&player.id.to_string(), player.x, player.y)
                .unwrap();
        }
    }
}

/// Draw the player's label on the canvas and rotate it
/// # Arguments
/// * `players` - The player's data through the struct 'Player' in a vector
/// # Example
/// ```
/// draw_players(&[Player]);
/// ```
pub fn draw_players(players: &[Player]) {
    for (_i, player) in players.iter().enumerate() {
        draw_player_orientation(player);
        display_player_position(player);
        draw_player_icon(player, get_number(&ROTATION_ANGLE));
    }
    toggle_label(players);
}

pub fn draw_player_icon(player: &Player, angle: f64) {
    let (_, context, _) = element::get_canvas_context_document();
    let agent_icon = Player::agent_player_icon_url(player.id);
    let agent_name = Player::get_agent_name(player.id);
    let icon = create_html_image_element(agent_name, &agent_icon, "player");
    let icon_width = 16.0;
    let icon_height = 16.0;
    context.save();
    context.translate(player.x, player.y).unwrap();
    context
        .translate(-icon_width / 2.0, -icon_height / 2.0)
        .unwrap();
    let angle_rad = get_radian_angle(-angle);
    context.rotate(angle_rad).unwrap();
    context
        .draw_image_with_html_image_element_and_dw_and_dh(&icon, 0.0, 0.0, icon_width, icon_height)
        .unwrap();
    context.restore();
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
    let (_, context, _) = element::get_canvas_context_document();
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
    context.set_stroke_style(&JsValue::from_str(identify_team(player.team, false)));
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
fn identify_team(team: i32, dark: bool) -> &'static str {
    if dark {
        match team {
            0 => "#66471C",
            1 => "#212D4C",
            _ => "black",
        }
    } else {
        match team {
            0 => "#DF9B33",
            1 => "#6678A7",
            _ => "black",
        }
    }
}

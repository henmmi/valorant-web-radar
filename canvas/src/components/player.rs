use super::macros::{console_log, log};
use super::player_data::Player;
use crate::components::canvas::{get_number, get_radian_angle, ROTATION_ANGLE};
use crate::components::elements;
use crate::components::elements::{get_canvas_context_document, get_html_image_element_by_id};
use crate::components::game_data::Weapon;
use crate::components::ui_element::{toggle_label, toggle_state};
use js_sys::Math::{cos, sin};
use std::f64;
use wasm_bindgen::JsValue;
use web_sys::{HtmlImageElement, OffscreenCanvasRenderingContext2d};

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
    let (_, context, _) = get_canvas_context_document();
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
    let (_, context, _) = get_canvas_context_document();
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
    let (_, context, _) = get_canvas_context_document();
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
        draw_weapon_icons(player, get_number(&ROTATION_ANGLE));
    }
    toggle_label(players);
}
/// Draw the player's icon on the canvas
/// # Arguments
/// * `player` - Input player data through the struct 'Player'
/// * `angle` - The player's angle
/// # Example
/// ```
/// draw_player_icon(&player, 90.0);
/// ```
pub fn draw_player_icon(player: &Player, angle: f64) {
    let (_, context, _) = get_canvas_context_document();
    let agent_name = Player::get_agent_name(player.id as usize);
    match get_html_image_element_by_id(agent_name.as_str()) {
        Ok(icon) => {
            let icon_width = 16.0;
            let icon_height = 16.0;
            context.save();
            if let Err(err) = context.translate(player.x, player.y) {
                console_log!("Error translating: {:?}", err);
            }
            let angle_rad = get_radian_angle(-angle);
            if let Err(err) = context.rotate(angle_rad) {
                console_log!("Error rotating: {:?}", err);
            }

            if toggle_state("dormant_player_toggle") && player.dormant == 1 {
                if let Err(err) = context.translate(-icon_width, -icon_height) {
                    console_log!("Error translating: {:?}", err);
                }
                let dormant_icon = get_html_image_element_by_id("Dormant").unwrap();
                if let Err(err) = context.draw_image_with_html_image_element_and_dw_and_dh(
                    &dormant_icon,
                    0.0,
                    0.0,
                    icon_width * 2.0,
                    icon_height * 2.0,
                ) {
                    console_log!("Error drawing image: {:?}", err);
                }
            } else {
                context
                    .translate(-icon_width / 2.0, -icon_height / 2.0)
                    .unwrap();
                if let Err(err) = context.draw_image_with_html_image_element_and_dw_and_dh(
                    &icon,
                    0.0,
                    0.0,
                    icon_width,
                    icon_height,
                ) {
                    console_log!("Error drawing image: {:?}", err);
                }
            }
            context.restore();
        }
        Err(err) => console_log!("Error getting image element: {:?}", err),
    }
}

/// Draw the player's weapon icon on the canvas
/// # Arguments
/// * `player` - Input player data through the struct 'Player'
/// * `angle` - The player's angle
/// # Example
/// ```
/// draw_weapon_icons(&player, 90.0);
/// ```
fn draw_weapon_icons(player: &Player, angle: f64) {
    let (_, context, _) = get_canvas_context_document();
    let scaling_factor = 0.15;
    let weapon_name = Weapon::match_weapon_id(player.weapon);
    match get_html_image_element_by_id(weapon_name.as_str()) {
        Ok(elem) => {
            let icon_width = elem.natural_width() as f64 * scaling_factor;
            let icon_height = elem.natural_height() as f64 * scaling_factor;

            let (offscreen_canvas, offscreen_context) =
                elements::get_offscreen_canvas_context(elem.width(), elem.height());

            offscreen_context
                .draw_image_with_html_image_element(&elem, 0.0, 0.0)
                .unwrap();

            set_image_colour(
                offscreen_context,
                elem,
                0.0,
                0.0,
                identify_team(player.team, false),
            );
            context.save();
            context.translate(player.x, player.y).unwrap();
            let angle_rad = get_radian_angle(-angle);
            context.rotate(angle_rad).unwrap();
            let image_bitmap = offscreen_canvas.transfer_to_image_bitmap().unwrap();
            context
                .draw_image_with_image_bitmap_and_dw_and_dh(
                    &image_bitmap,
                    -icon_width / 2.0,
                    -28.0,
                    icon_width,
                    icon_height,
                )
                .unwrap();
        }
        Err(err) => console_log!("Error getting weapon icon element: {:?}", err),
    }
    context.restore();
}
/// Set the image colour of the player's icon
/// # Arguments
/// * `context` - The context of the canvas
/// * `image` - The image element
/// * `x` - The X coordinate
/// * `y` - The Y coordinate
/// * `colour` - The colour of the image
/// # Example
/// ```
/// set_image_colour(context, image, 100.0, 100.0, "red");
/// ```
pub fn set_image_colour(
    context: OffscreenCanvasRenderingContext2d,
    image: HtmlImageElement,
    x: f64,
    y: f64,
    colour: &str,
) {
    let (width, height) = (image.width(), image.height());

    context
        .draw_image_with_html_image_element(&image, x, y)
        .expect("Error drawing image");

    context
        .set_global_composite_operation("source-in")
        .expect("Error setting composite operation");

    context.set_fill_style(&JsValue::from_str(colour));
    context.fill_rect(x, y, width as f64, height as f64);
    context
        .set_global_composite_operation("source-over")
        .unwrap();
}
/// Display the players orientation as a triangle
/// # Arguments
/// * `player` - Input player data through the struct 'Player'
/// # Example
/// ```
/// draw_player_orientation(&player);
/// ```
fn draw_player_orientation(player: &Player) {
    let (_, context, _) = get_canvas_context_document();
    let angle = player.rotation;
    let view_line_size = 20f64;
    let start = get_radian_angle(angle - 15.0);
    let end = get_radian_angle(angle + 15.0);
    let x = view_line_size * cos(get_radian_angle(angle));
    let y = view_line_size * sin(get_radian_angle(angle));

    context.save();
    context.begin_path();
    context.translate(player.x, player.y).unwrap();

    // Draw the triangle for the player's orientation
    context.arc(0.0, 0.0, 13.0, start, end).unwrap();
    context.line_to(x, y);
    context.move_to(13.0 * cos(end), 13.0 * sin(end));
    context.line_to(x, y);

    context.set_stroke_style(&JsValue::from_str(identify_team(player.team, false)));
    context.set_fill_style(&JsValue::from_str(identify_team(player.team, false)));
    context.fill();

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
pub fn identify_team(team: i32, dark: bool) -> &'static str {
    if dark {
        match team {
            0 => "#66471C",
            1 => "#212D4C",
            _ => "grey",
        }
    } else {
        match team {
            0 => "#DF9B33",
            1 => "#6678A7",
            _ => "grey",
        }
    }
}

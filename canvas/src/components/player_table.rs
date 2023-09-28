use crate::components::elements;
use crate::components::elements::{
    create_html_div_element, get_div_element_by_id, get_html_image_element_by_id,
};
use crate::components::game_data::{GameScore, Weapon};
use crate::components::player::identify_team;
use crate::components::player_data::Player;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

/// Generates a info table for each player
/// # Arguments
/// * `player` - A vector of player data
/// * `team` - A GameScore struct
/// # Example
/// ```
/// create_player_info_row(&player, &team);
/// ```
pub fn create_player_info_row(player: &[Player], team: &GameScore) {
    elements::delete_collection_contents("team_score");
    elements::delete_collection_contents("players");
    create_team_info_header(team);
    create_player_info(player);
}
/// Creates the player info for the player table
/// # Arguments
/// * `player` - A vector of player data
/// # Example
/// ```
/// create_player_info(&player);
/// ```
fn create_player_info(player: &[Player]) {
    for agent in player.iter() {
        let player_row =
            create_html_div_element(format!("player_{}_info", agent.id).as_str(), "player_row")
                .unwrap();
        get_div_element_by_id(format!("team_{}_players", agent.team).as_str())
            .unwrap()
            .append_child(&player_row)
            .unwrap();
        let player_name = Player::get_agent_name(agent.id as usize);

        let (canvas, context) = new_player_info_block();
        // Set player row layout as three components
        let health_bar_size = canvas.width() as f64 * 0.84;
        player_row.append_child(&canvas).unwrap();
        add_health_text_and_bar(&agent, &context, &canvas, health_bar_size);
        add_weapon_icon_to_player_block(agent, &canvas, &context);
        add_player_name_icon_to_block(&player_name, &canvas, &context, health_bar_size);
        add_credits_text(
            agent,
            &canvas,
            &context,
            canvas.width() as f64 - health_bar_size,
        );
        add_kda_acs_text(agent, &canvas, &context, health_bar_size);
        add_shield_info(agent, &canvas, &context);
    }
}
/// Creates the header info for the each team
/// # Arguments
/// * `team` - A GameScore struct
/// # Example
/// ```
/// create_header_info(&team);
/// ```
fn create_team_info_header(team: &GameScore) {
    for i in 0..2 {
        let header =
            create_html_div_element(format!("team_{}_header", i).as_str(), "team_header").unwrap();
        get_div_element_by_id(format!("team_{}_score", i).as_str())
            .unwrap()
            .append_child(&header)
            .unwrap();
        let team_name = match i {
            0 => "Attackers",
            1 => "Defenders",
            _ => "Unknown",
        };
        let score = match i {
            0 => team.t_score,
            1 => team.ct_score,
            _ => 0,
        };
        let (canvas, context) = new_player_info_block();
        header.append_child(&canvas).unwrap();
        context.set_font("bold 18px sans-serif");
        context.set_text_align("left");
        context.set_text_baseline("middle");
        context.set_fill_style(&JsValue::from_str("white"));
        context
            .fill_text(team_name, 20.0, canvas.height() as f64 / 2.0)
            .unwrap();
        context.set_fill_style(&JsValue::from_str(identify_team(i, false)));
        context
            .fill_text(
                score.to_string().as_str(),
                canvas.width() as f64 * 0.9,
                canvas.height() as f64 / 2.0,
            )
            .unwrap();
    }
}

/// Adds shield info to player info block
/// # Arguments
/// * `agent` - A player data struct
/// * `canvas` - A canvas element
/// * `context` - A canvas rendering context
/// # Example
/// ```
/// add_shield_info(&agent, &canvas, &context);
/// ```
fn add_shield_info(agent: &Player, canvas: &HtmlCanvasElement, context: &CanvasRenderingContext2d) {
    let image_size = 20.0;
    context.set_font("14px sans-serif");
    context.set_text_align("right");
    context.set_text_baseline("middle");
    context.set_fill_style(&JsValue::from_str("#01FFFE"));

    context
        .fill_text(
            format!("{}", agent.shield).as_str(),
            canvas.width() as f64 - image_size * 1.25,
            canvas.height() as f64 * 0.75,
        )
        .unwrap();

    if agent.shield > 25 {
        let shield_max = 50.0;
        let shield_name = "HeavyShield";
        draw_light_heavy_shield(context, canvas, agent, image_size, shield_max, shield_name);
    } else {
        let shield_max = 25.0;
        let shield_name = "LightShield";
        draw_light_heavy_shield(context, canvas, agent, image_size, shield_max, shield_name);
    }
}
/// Draws the light or heavy shield
/// # Arguments
/// * `context` - A canvas rendering context
/// * `canvas` - A canvas element
/// * `agent` - A player data struct
/// * `image_size` - A f64 value of the image size
/// * `shield_max` - A f64 value of the shield max
/// * `shield_name` - A string slice of the shield name
/// # Example
/// ```
/// draw_light_heavy_shield(&context, &canvas, &agent, image_size, shield_max, shield_name);
/// ```
fn draw_light_heavy_shield(
    context: &CanvasRenderingContext2d,
    canvas: &HtmlCanvasElement,
    agent: &Player,
    image_size: f64,
    shield_max: f64,
    shield_name: &str,
) {
    context.set_fill_style(&JsValue::from_str("#47FABC"));
    context.set_global_alpha(0.5);
    context.fill_rect(
        canvas.width() as f64 - image_size,
        canvas.height() as f64,
        image_size,
        -(canvas.height() as f64 * 0.5) * agent.shield as f64 / shield_max,
    );
    context
        .draw_image_with_html_image_element_and_dw_and_dh(
            &get_html_image_element_by_id(shield_name).unwrap(),
            canvas.width() as f64 - image_size,
            canvas.height() as f64 * 0.75 - image_size / 2.0,
            image_size,
            image_size,
        )
        .unwrap();
}

/// Adds KDA and ACS text to player info block
/// # Arguments
/// * `agent` - A player data struct
/// * `canvas` - A canvas element
/// * `context` - A canvas rendering context
/// * `health_bar_size` - A f64 value of the health bar size
/// # Example
/// ```
/// add_kda_acs_text(&agent, &canvas, &context, health_bar_size);
/// ```
fn add_kda_acs_text(
    agent: &Player,
    canvas: &HtmlCanvasElement,
    context: &CanvasRenderingContext2d,
    health_bar_size: f64,
) {
    context.set_font("14px sans-serif");
    context.set_text_align("left");
    context.set_text_baseline("middle");
    context.set_fill_style(&JsValue::from_str("white"));
    context
        .fill_text(
            format!("{} / {} / {}", agent.kill, agent.death, agent.assist).as_str(),
            health_bar_size / 4.0 + (canvas.width() as f64 - health_bar_size),
            canvas.height() as f64 * 0.75,
        )
        .unwrap();
    context.set_text_align("right");
    context
        .fill_text(
            format!("{} ACS", agent.acs).as_str(),
            health_bar_size * 0.75 + (canvas.width() as f64 - health_bar_size),
            canvas.height() as f64 * 0.75,
        )
        .unwrap();
}

/// Adds credits text to player info block
/// # Arguments
/// * `agent` - A player data struct
/// * `canvas` - A canvas element
/// * `context` - A canvas rendering context
/// * `start_x` - A f64 value of the starting x position
/// # Example
/// ```
/// add_credits_text(&agent, &canvas, &context, start_x);
/// ```
fn add_credits_text(
    agent: &Player,
    canvas: &HtmlCanvasElement,
    context: &CanvasRenderingContext2d,
    start_x: f64,
) {
    context.set_font("14px sans-serif");
    context.set_text_align("left");
    context.set_text_baseline("middle");
    context.set_fill_style(&JsValue::from_str("#25B14E"));
    context
        .fill_text(
            format!("$ {}", agent.credits.to_string().as_str()).as_str(),
            20.0 + start_x,
            canvas.height() as f64 * 0.75,
        )
        .expect("TODO: panic message");
}

/// Adds player name and icon to player info block
/// # Arguments
/// * `player_name` - A string slice of the player name
/// * `canvas` - A canvas element
/// * `context` - A canvas rendering context
/// * `health_bar_size` - A f64 value of the health bar size
/// # Example
/// ```
/// add_player_name_icon_to_block(&player_name, canvas, context, health_bar_size);
/// ```
fn add_player_name_icon_to_block(
    player_name: &str,
    canvas: &HtmlCanvasElement,
    context: &CanvasRenderingContext2d,
    health_bar_size: f64,
) {
    context
        .draw_image_with_html_image_element_and_dw_and_dh(
            &get_html_image_element_by_id(player_name).unwrap(),
            0.0,
            0.0,
            64.0,
            64.0,
        )
        .unwrap();

    context.set_font("14px sans-serif");
    context.set_text_align("left");
    context.set_text_baseline("middle");
    context.set_fill_style(&JsValue::from_str("white"));
    context
        .fill_text(
            player_name,
            health_bar_size / 4.0 + (canvas.width() as f64 - health_bar_size),
            canvas.height() as f64 / 4.0,
        )
        .expect("TODO: panic message");
}
/// Adds weapon icon to player info block
/// # Arguments
/// * `agent` - A player data struct
/// * `canvas` - A canvas element
/// * `context` - A canvas rendering context
/// * `health_bar_size` - A f64 value of the health bar size
/// # Example
/// ```
/// add_weapon_icon_to_player_block(&agent, &canvas, &context, health_bar_size);
/// ```
fn add_weapon_icon_to_player_block(
    agent: &Player,
    canvas: &HtmlCanvasElement,
    context: &CanvasRenderingContext2d,
) {
    let weapon_icon =
        get_html_image_element_by_id(Weapon::match_weapon_id(agent.weapon).as_str()).unwrap();
    let weapon_icon_width = weapon_icon.width() as f64 * 0.15;
    let weapon_icon_height = weapon_icon.height() as f64 * 0.15;
    context
        .draw_image_with_html_image_element_and_dw_and_dh(
            &weapon_icon,
            canvas.width() as f64 - weapon_icon_width - 10.0,
            canvas.height() as f64 / 4.0 - weapon_icon_height / 2.0,
            weapon_icon_width,
            weapon_icon_height,
        )
        .unwrap();
}
/// Adds health text and bar to player info block
/// # Arguments
/// * `agent` - A player data struct
/// * `context` - A canvas rendering context
/// * `canvas` - A canvas element
/// * `health_bar_width` - A f64 value of the health bar width
/// # Example
/// ```
/// add_health_text_and_bar(&agent, &context, &canvas, health_bar_width);
/// ```
fn add_health_text_and_bar(
    agent: &&Player,
    context: &CanvasRenderingContext2d,
    canvas: &HtmlCanvasElement,
    health_bar_size: f64,
) {
    let bar_height = canvas.height() as f64 / 2.0;
    // Health Bar Background
    context.set_fill_style(&JsValue::from_str(identify_team(agent.team, true)));
    context.fill_rect(
        canvas.width() as f64 - health_bar_size,
        0.0,
        health_bar_size,
        bar_height,
    );
    // Health Bar
    context.set_fill_style(&JsValue::from_str(identify_team(agent.team, false)));
    context.fill_rect(
        canvas.width() as f64 - health_bar_size,
        0.0,
        health_bar_size * agent.health / 100.0,
        bar_height,
    );
    // Health Text
    context.set_font("14px sans-serif");
    context.set_text_align("left");
    context.set_text_baseline("middle");
    context.set_fill_style(&JsValue::from_str("white"));
    context
        .fill_text(
            (agent.health).round().to_string().as_str(),
            20.0 + canvas.width() as f64 - health_bar_size,
            bar_height / 2.0,
        )
        .expect("TODO: panic message");
}
/// Creates a new player info block
/// # Example
/// ```
/// let (canvas, context) = new_player_info_block();
/// ```
/// # Returns
/// * `canvas` - A canvas element
/// * `context` - A canvas rendering context
fn new_player_info_block() -> (HtmlCanvasElement, CanvasRenderingContext2d) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    canvas.set_width(425);
    canvas.set_height(60);
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    (canvas, context)
}

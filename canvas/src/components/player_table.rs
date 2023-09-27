use crate::components::elements::{
    create_html_div_element, get_canvas_context_document, get_div_element_by_id,
    get_elements_by_class, get_html_image_element_by_id,
};
use crate::components::game_data::Weapon;
use crate::components::player::identify_team;
use crate::components::player_data::Player;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlCanvasElement;

pub fn create_player_info_row(player: &[Player]) {
    let (_, _, document) = get_canvas_context_document();

    // Clear player info every instance
    if let Some(player_row) = get_elements_by_class("players") {
        for i in 0..player_row.length() as usize {
            let range = document.create_range().unwrap();
            range
                .select_node_contents(&player_row.item(i as u32).unwrap())
                .unwrap();
            range.delete_contents().unwrap();
        }
    }

    // Populate player info
    for (_i, agent) in player.iter().enumerate() {
        let player_row =
            create_html_div_element(format!("player_{}_info", agent.id).as_str(), "player_row")
                .unwrap();
        get_div_element_by_id(format!("team_{}_players", agent.team).as_str())
            .unwrap()
            .append_child(&player_row)
            .unwrap();
        let player_name = Player::get_agent_name(agent.id as usize);
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        canvas.set_width(280);
        canvas.set_height(30);

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        player_row.append_child(&canvas).unwrap();
        context.set_fill_style(&JsValue::from_str(identify_team(agent.team, true)));
        context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        context.set_fill_style(&JsValue::from_str(identify_team(agent.team, false)));
        context.fill_rect(
            0.0,
            0.0,
            canvas.width() as f64 * agent.health / 100.0,
            canvas.height() as f64,
        );
        context.set_font("14px sans-serif");
        context.set_text_align("center");
        context.set_text_baseline("middle");
        context.set_fill_style(&JsValue::from_str("white"));
        context
            .fill_text(
                (agent.health).round().to_string().as_str(),
                20.0,
                canvas.height() as f64 / 2.0,
            )
            .expect("TODO: panic message");
        let weapon_icon =
            get_html_image_element_by_id(Weapon::match_weapon_id(agent.weapon).as_str()).unwrap();
        let weapon_icon_width = weapon_icon.width() as f64 * 0.15;
        let weapon_icon_height = weapon_icon.height() as f64 * 0.15;
        context
            .draw_image_with_html_image_element_and_dw_and_dh(
                &weapon_icon,
                canvas.width() as f64 - weapon_icon_width - 10.0,
                canvas.height() as f64 / 2.0 - weapon_icon_height / 2.0,
                weapon_icon_width,
                weapon_icon_height,
            )
            .unwrap();
        context.set_font("14px sans-serif");
        context.set_text_align("left");
        context.set_text_baseline("middle");
        context.set_fill_style(&JsValue::from_str("white"));
        context
            .fill_text(
                player_name.as_str(),
                canvas.width() as f64 / 4.0,
                canvas.height() as f64 / 2.0,
            )
            .expect("TODO: panic message");
    }
}

// pub fn add_player_health_bar(health: i32) -> Result<(), JsValue> {
//     let (_, _, document) = get_canvas_context_document();
//     let (canvas, context) = get_player_table_canvas_context();
// }

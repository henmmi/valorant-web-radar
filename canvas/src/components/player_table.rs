use crate::components::elements::{
    create_html_div_element, get_canvas_context_document, get_div_element_by_id,
    get_elements_by_class,
};
use crate::components::player_data::Player;
pub fn create_player_info_row(player: &[Player]) {
    let (_, _, document) = get_canvas_context_document();
    if let Some(player_row) = get_elements_by_class("players") {
        for i in 0..player_row.length() as usize {
            let range = document.create_range().unwrap();
            range
                .select_node_contents(&player_row.item(i as u32).unwrap())
                .unwrap();
            range.delete_contents().unwrap();
        }
    }
    for (i, agent) in player.iter().enumerate() {
        let _player_name = Player::get_agent_name(i);
        let player_row =
            create_html_div_element(format!("Player_{}_info", agent.id).as_str(), "player_row")
                .unwrap();
        get_div_element_by_id(format!("team_{}_players", agent.team).as_str())
            .unwrap()
            .append_child(&player_row)
            .unwrap();
    }
}

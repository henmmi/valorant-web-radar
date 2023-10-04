use crate::components::canvas::{get_number, get_radian_angle, ROTATION_ANGLE};
use crate::components::elements::{get_canvas_context_document, get_html_image_element_by_id};
use crate::components::websocket::get_host;
use serde::Deserialize;
use strum_macros::EnumIter;

#[derive(Deserialize, Debug, EnumIter)]
pub enum Agent {
    Brimstone,
    Viper,
    Omen,
    Killjoy,
    Cypher,
    Sova,
    Sage,
    Phoenix,
    Jett,
    Reyna,
    Raze,
    Breach,
    Skye,
    Yoru,
    Astra,
    Kayo,
    Chamber,
    Neon,
    Fade,
    Harbor,
    Gekko,
    Deadlock,
}

/// Data container for a single player
#[derive(Deserialize, Debug)]
pub struct Player {
    pub id: i32,
    pub x: f64,
    pub y: f64,
    pub health: i32,
    pub team: i32,
    pub dormant: i32,
    pub rotation: f64,
    pub scoped: i32,
    pub weapon: i32,
    pub kill: i32,
    pub death: i32,
    pub assist: i32,
    pub acs: i32,
    pub shield: i32,
    pub credits: i32,
}
/// Data container for all players
#[derive(Deserialize, Debug)]
pub struct Players {
    pub id: Vec<i32>,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub health: Vec<i32>,
    pub team: Vec<i32>,
    pub dormant: Vec<i32>,
    pub rotation: Vec<f64>,
    pub scoped: Vec<i32>,
    pub weapon: Vec<i32>,
    pub kill: Vec<i32>,
    pub death: Vec<i32>,
    pub assist: Vec<i32>,
    pub acs: Vec<i32>,
    pub shield: Vec<i32>,
    pub credits: Vec<i32>,
}
impl Player {
    /// Get the agent name from the id
    /// # Arguments
    /// * `id` - The id of the agent
    /// # Example
    /// ```
    /// assert_eq!(Player::get_agent_name(0), "Brimstone");
    /// ```
    pub fn get_agent_name(id: usize) -> String {
        match id {
            0 => "Brimstone".to_string(),
            1 => "Viper".to_string(),
            2 => "Omen".to_string(),
            3 => "Killjoy".to_string(),
            4 => "Cypher".to_string(),
            5 => "Sova".to_string(),
            6 => "Sage".to_string(),
            7 => "Phoenix".to_string(),
            8 => "Jett".to_string(),
            9 => "Reyna".to_string(),
            10 => "Raze".to_string(),
            11 => "Breach".to_string(),
            12 => "Skye".to_string(),
            13 => "Yoru".to_string(),
            14 => "Astra".to_string(),
            15 => "Kayo".to_string(),
            16 => "Chamber".to_string(),
            17 => "Neon".to_string(),
            18 => "Fade".to_string(),
            19 => "Harbor".to_string(),
            20 => "Gekko".to_string(),
            21 => "Deadlock".to_string(),
            _ => "Unknown".to_string(),
        }
    }
    /// Get the agent icon url from the id
    /// # Arguments
    /// * `id` - The id of the agent
    /// # Example
    /// ```
    /// assert_eq!(Player::agent_player_icon_url(0), "http://url:8080/images/Brimstone.png");
    /// ```
    pub fn agent_player_icon_url(id: usize) -> String {
        format!(
            "http://{}/images/{}.png",
            get_host(),
            Player::get_agent_name(id)
        )
    }
}
#[derive(Debug)]
pub struct DeadPlayers {
    pub x: f64,
    pub y: f64,
    pub prevail_count: i32,
}
/// Data container for all dead players
/// # Example
/// ```
/// let mut dead_players = DeadPlayers::new(0.0, 0.0);
/// ```
impl DeadPlayers {
    /// Create a new DeadPlayers object
    /// # Arguments
    /// * `x` - The x coordinate of the dead player
    /// * `y` - The y coordinate of the dead player
    /// # Example
    /// ```
    /// let mut dead_players = DeadPlayers::new(0.0, 0.0);
    /// ```
    pub fn new(x: f64, y: f64) -> Self {
        DeadPlayers {
            x,
            y,
            prevail_count: 5,
        }
    }
    /// Display dead players as Killed icon on the map for a few seconds
    /// # Arguments
    /// * `killed` - The vector of killed players
    /// # Example
    /// ```
    /// DeadPlayers::draw_dead_players(&mut Vec<DeadPlayers>);
    /// ```
    pub fn draw_dead_players(killed: &mut Vec<DeadPlayers>) {
        const ALPHA_FACTOR: f64 = 0.2;
        const DEATH_ICON_SIZE: f64 = 32.0;
        let (_, context, _) = get_canvas_context_document();
        let angle = get_number(&ROTATION_ANGLE);
        for player in killed.iter_mut() {
            context.save();
            context.translate(player.x, player.y).unwrap();
            let angle_rad = get_radian_angle(-angle);
            let death_icon = get_html_image_element_by_id("Killed").unwrap();
            context.rotate(angle_rad).unwrap();
            context.set_global_alpha(ALPHA_FACTOR * player.prevail_count as f64);
            context
                .draw_image_with_html_image_element_and_dw_and_dh(
                    &death_icon,
                    -DEATH_ICON_SIZE / 2.0,
                    -DEATH_ICON_SIZE / 2.0,
                    DEATH_ICON_SIZE,
                    DEATH_ICON_SIZE,
                )
                .unwrap();
            context.restore();
            player.prevail_count -= 1;
        }
        // Retain the killed players that still have a prevail count
        killed.retain(|x| x.prevail_count > 0);
    }
}

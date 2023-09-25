use serde::Deserialize;

/// Data container for a single player
#[derive(Deserialize, Debug)]
pub struct Player {
    pub id: i32,
    pub x: f64,
    pub y: f64,
    pub health: f64,
    pub team: i32,
    pub dormant: i32,
    pub rotation: f64,
    pub scoped: i32,
}
/// Data container for all players
#[derive(Deserialize, Debug)]
pub struct Players {
    pub id: Vec<i32>,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub health: Vec<f64>,
    pub team: Vec<i32>,
    pub dormant: Vec<i32>,
    pub rotation: Vec<f64>,
    pub scoped: Vec<i32>,
}
impl Player {
    pub fn get_agent_name(id: i32) -> &'static str {
        match id {
            0 => "Brimstone",
            1 => "Viper",
            2 => "Omen",
            3 => "Killjoy",
            4 => "Cypher",
            5 => "Sova",
            6 => "Sage",
            7 => "Phoenix",
            8 => "Jett",
            9 => "Reyna",
            10 => "Raze",
            11 => "Breach",
            12 => "Skye",
            13 => "Yoru",
            14 => "Astra",
            15 => "Kayo",
            16 => "Chamber",
            17 => "Neon",
            18 => "Fade",
            19 => "Harbor",
            20 => "Gekko",
            21 => "Deadlock",
            _ => "Unknown",
        }
    }
    pub fn agent_player_icon_url(id: i32) -> String {
        "http://127.0.0.1:8080/images/".to_owned() + Player::get_agent_name(id) + ".png"
    }
}

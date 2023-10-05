use super::macros::{console_log, log};
use crate::components::elements::{create_html_image_element, get_div_element_by_id};
use crate::components::player_data::{Agent, Player};
use crate::components::websocket::get_host;
use serde::Deserialize;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlImageElement;

#[derive(Deserialize, Debug, EnumIter)]
pub enum Map {
    Ascent,
    Bind,
    Breeze,
    Haven,
    Icebox,
    Split,
}
impl Map {
    pub fn get_string(&self) -> String {
        match self {
            Map::Ascent => "Ascent".to_string(),
            Map::Bind => "Bind".to_string(),
            Map::Breeze => "Breeze".to_string(),
            Map::Haven => "Haven".to_string(),
            Map::Icebox => "Icebox".to_string(),
            Map::Split => "Split".to_string(),
        }
    }
}

#[derive(Deserialize, Debug, EnumIter)]
enum Icon {
    Dormant,
    Killed,
    HeavyShield,
    LightShield,
    Switch,
    Spike,
    Defuse,
}

impl Icon {
    pub fn get_string(&self) -> String {
        match self {
            Icon::Dormant => "Dormant".to_string(),
            Icon::Killed => "Killed".to_string(),
            Icon::HeavyShield => "HeavyShield".to_string(),
            Icon::LightShield => "LightShield".to_string(),
            Icon::Switch => "Switch".to_string(),
            Icon::Spike => "Spike".to_string(),
            Icon::Defuse => "Defuse".to_string(),
        }
    }
}
#[derive(Deserialize, Debug, EnumIter)]
pub enum Weapon {
    Ares,
    Bucky,
    Bulldog,
    ChamberOp,
    ChamberQ,
    Classic,
    Frenzy,
    Ghost,
    Guardian,
    Judge,
    Knife,
    Marshal,
    Odin,
    Operator,
    Phantom,
    Sheriff,
    Shorty,
    Spectre,
    Stinger,
    Vandal,
}

impl Weapon {
    pub fn get_string(&self) -> String {
        match self {
            Weapon::Ares => "Ares".to_string(),
            Weapon::Bucky => "Bucky".to_string(),
            Weapon::Bulldog => "Bulldog".to_string(),
            Weapon::ChamberOp => "ChamberOp".to_string(),
            Weapon::ChamberQ => "ChamberQ".to_string(),
            Weapon::Classic => "Classic".to_string(),
            Weapon::Frenzy => "Frenzy".to_string(),
            Weapon::Ghost => "Ghost".to_string(),
            Weapon::Guardian => "Guardian".to_string(),
            Weapon::Judge => "Judge".to_string(),
            Weapon::Knife => "Knife".to_string(),
            Weapon::Marshal => "Marshal".to_string(),
            Weapon::Odin => "Odin".to_string(),
            Weapon::Operator => "Operator".to_string(),
            Weapon::Phantom => "Phantom".to_string(),
            Weapon::Sheriff => "Sheriff".to_string(),
            Weapon::Shorty => "Shorty".to_string(),
            Weapon::Spectre => "Spectre".to_string(),
            Weapon::Stinger => "Stinger".to_string(),
            Weapon::Vandal => "Vandal".to_string(),
        }
    }

    pub fn match_weapon_id(id: i32) -> String {
        match id {
            0 => Weapon::Ares.get_string(),
            1 => Weapon::Bucky.get_string(),
            2 => Weapon::Bulldog.get_string(),
            3 => Weapon::ChamberOp.get_string(),
            4 => Weapon::ChamberQ.get_string(),
            5 => Weapon::Classic.get_string(),
            6 => Weapon::Frenzy.get_string(),
            7 => Weapon::Ghost.get_string(),
            8 => Weapon::Guardian.get_string(),
            9 => Weapon::Judge.get_string(),
            10 => Weapon::Marshal.get_string(),
            11 => Weapon::Odin.get_string(),
            12 => Weapon::Operator.get_string(),
            13 => Weapon::Phantom.get_string(),
            14 => Weapon::Sheriff.get_string(),
            15 => Weapon::Shorty.get_string(),
            16 => Weapon::Spectre.get_string(),
            17 => Weapon::Stinger.get_string(),
            18 => Weapon::Vandal.get_string(),
            _ => Weapon::Knife.get_string(),
        }
    }
}
#[derive(Deserialize, Debug, Clone)]
pub struct GameScore {
    pub round_win_status: i32,
}
pub fn get_score(score: &[GameScore]) -> (i32, i32) {
    let mut t_score = 0;
    let mut ct_score = 0;
    for (_, val) in score.iter().enumerate() {
        if val.round_win_status == 0 {
            t_score += 1;
        } else if val.round_win_status == 1 {
            ct_score += 1;
        }
    }
    (t_score, ct_score)
}
#[derive(Deserialize, Debug)]
pub struct GameInfo {
    pub round_win_status: Vec<i32>,
    pub max_rounds: i32,
    pub round_time: Vec<f64>,
    pub spike_planted: i32,
    pub spike_x: Vec<f64>,
    pub spike_y: Vec<f64>,
    pub spike_time: Vec<f64>,
}

impl GameInfo {
    /// Get the map url
    /// # Arguments
    /// * `name` - The name of the map
    /// # Example
    /// ```
    /// assert_eq!(GameInfo::get_map_url("Ascent"), "http://url:8080/images/Ascent.png");
    /// ```
    pub fn get_map_url(name: &str) -> String {
        format!("http://{}/images/{}.png", get_host(), name)
    }
}
#[wasm_bindgen]
pub struct Preloader {
    agents: HashMap<String, HtmlImageElement>,
    maps: HashMap<String, HtmlImageElement>,
    weapons: HashMap<String, HtmlImageElement>,
    icons: HashMap<String, HtmlImageElement>,
}
#[wasm_bindgen]
impl Preloader {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Preloader {
            agents: HashMap::new(),
            maps: HashMap::new(),
            weapons: HashMap::new(),
            icons: HashMap::new(),
        }
    }
    /// Preload all the assets
    /// # Example
    /// ```
    /// let preloader = Preloader::new();
    /// preloader.preload_assets();
    /// ```
    pub fn preload_assets(&mut self) {
        Preloader::preload_agents(self, "agent");
        Preloader::preload_maps(self, "map");
        Preloader::preload_icons(self, "icon");
        Preloader::preload_weapons(self, "weapon");
    }
    /// Preload the agents icons
    /// # Arguments
    /// * `class` - The class of the image element
    /// # Example
    /// ```
    /// let preloader = Preloader::new();
    /// preloader.preload_agents("agent");
    /// ```
    pub fn preload_agents(&mut self, class: &str) {
        if let Ok(div) = get_div_element_by_id("player_storage") {
            for (id, _player) in Agent::iter().enumerate() {
                match create_html_image_element(
                    Player::get_agent_name(id).as_str(),
                    Player::agent_player_icon_url(id).as_str(),
                    class,
                ) {
                    Ok(element) => {
                        element.style().set_property("display", "none").unwrap();
                        div.append_child(&element).unwrap();
                        self.agents.insert(Player::get_agent_name(id), element);
                    }
                    Err(err) => console_log!("Error creating image element: {:?}", err),
                }
            }
        }
    }
    /// Preload the maps icons
    /// # Arguments
    /// * `class` - The class of the image element
    /// # Example
    /// ```
    /// let preloader = Preloader::new();
    /// preloader.preload_maps("map");
    /// ```
    pub fn preload_maps(&mut self, class: &str) {
        if let Ok(div) = get_div_element_by_id("map_storage") {
            for map_name in Map::iter() {
                match create_html_image_element(
                    &map_name.get_string(),
                    &GameInfo::get_map_url(&map_name.get_string()),
                    class,
                ) {
                    Ok(element) => {
                        element.style().set_property("display", "none").unwrap();
                        div.append_child(&element).unwrap();
                        self.maps.insert(map_name.get_string(), element);
                    }
                    Err(err) => console_log!("Error creating image element: {:?}", err),
                }
            }
        }
    }
    /// Preload the icons
    /// # Arguments
    /// * `name` - The name of the icon
    /// # Example
    /// ```
    /// let preloader = Preloader::new();
    /// preloader.preload_icons("Dormant");
    /// ```
    pub fn preload_icons(&mut self, class: &str) {
        if let Ok(div) = get_div_element_by_id("icon_storage") {
            for icon in Icon::iter() {
                match create_html_image_element(
                    &icon.get_string(),
                    get_url(&icon.get_string()).as_str(),
                    class,
                ) {
                    Ok(element) => {
                        element.style().set_property("display", "none").unwrap();
                        div.append_child(&element).unwrap();
                        self.icons.insert(icon.get_string(), element);
                    }
                    Err(err) => console_log!("Error creating image element: {:?}", err),
                }
            }
        }
    }
    /// Preload the weapons icons
    /// # Arguments
    /// * `name` - The name of the weapon
    /// # Example
    /// ```
    /// let preloader = Preloader::new();
    /// preloader.preload_weapons("weapon");
    /// ```
    pub fn preload_weapons(&mut self, name: &str) {
        if let Ok(div) = get_div_element_by_id("weapon_storage") {
            for weapons in Weapon::iter() {
                match create_html_image_element(
                    &weapons.get_string(),
                    get_url(&weapons.get_string()).as_str(),
                    name,
                ) {
                    Ok(element) => {
                        element.style().set_property("display", "none").unwrap();
                        div.append_child(&element).unwrap();
                        self.weapons.insert(weapons.get_string(), element);
                    }
                    Err(err) => console_log!("Error creating image element: {:?}", err),
                }
            }
        }
    }
}
impl Default for Preloader {
    fn default() -> Self {
        Preloader::new()
    }
}

pub fn get_url(name: &str) -> String {
    format!("http://{}/images/{}.png", get_host(), name)
}

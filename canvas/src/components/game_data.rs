use super::macros::{console_log, log};
use crate::components::elements::{
    create_html_image_element, get_div_element_by_id, get_html_image_element_by_id,
    get_offscreen_canvas_context,
};
use crate::components::player::{identify_team, set_image_colour};
use crate::components::player_data::{Agent, Player};
use crate::components::websocket::get_host;
use serde::Deserialize;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

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
}

impl Icon {
    pub fn get_string(&self) -> String {
        match self {
            Icon::Dormant => "Dormant".to_string(),
            Icon::Killed => "Killed".to_string(),
            Icon::HeavyShield => "HeavyShield".to_string(),
            Icon::LightShield => "LightShield".to_string(),
            Icon::Switch => "Switch".to_string(),
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
}
#[wasm_bindgen]
impl Preloader {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Preloader {
            agents: HashMap::new(),
            maps: HashMap::new(),
            weapons: HashMap::new(),
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
                        console_log!("Preloaded agent {}", Player::get_agent_name(id));
                        console_log!("Agent URL: {}", Player::agent_player_icon_url(id));
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
                        console_log!("Preloaded map {}", map_name.get_string());
                        console_log!("Map URL: {}", GameInfo::get_map_url(&map_name.get_string()));
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
                        self.maps.insert(icon.get_string(), element);
                        console_log!("Preloaded icon {}", icon.get_string());
                        console_log!("Icon URL: {}", get_url(&icon.get_string()));
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
                        console_log!("Preloaded weapon {}", weapons.get_string());
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
#[derive(Debug, Copy, Clone)]
pub struct RoundDisplayConfig {
    text_size: f64,
    rect_size: f64,
    gap_size: f64,
    initial_canvas_width: u32,
}
/// Implement the RoundDisplayConfig struct
/// # Example
/// ```
/// let round_display_config = RoundDisplayConfig::new();
/// ```
impl RoundDisplayConfig {
    const TEXT_SIZE: f64 = 20.0;
    const RECT_SIZE: f64 = 30.0;
    const GAP_SIZE: f64 = 50.0;
    const INITIAL_CANVAS_WIDTH: u32 = 1000;
    pub fn new() -> Self {
        RoundDisplayConfig {
            text_size: Self::TEXT_SIZE,
            rect_size: Self::RECT_SIZE,
            gap_size: Self::GAP_SIZE,
            initial_canvas_width: Self::INITIAL_CANVAS_WIDTH,
        }
    }
    /// Get the canvas context for the rounds display
    /// # Example
    /// ```
    /// let (canvas, context) = self.get_rounds_display_canvas_context();
    /// ```
    fn get_rounds_display_canvas_context(&self) -> (HtmlCanvasElement, CanvasRenderingContext2d) {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("rounds_display").unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        (canvas, context)
    }
    /// Create the rounds played row
    /// # Arguments
    /// * `game_score` - The game score
    /// * `info` - The game info
    /// # Example
    /// ```
    /// self.create_rounds_played_row(&game_score, &info);
    /// ```
    pub fn create_rounds_played_row(&self, game_score: &[GameScore], info: &GameInfo) {
        let (canvas, context) = self.get_rounds_display_canvas_context();
        if let Ok(div) = get_div_element_by_id("rounds_played") {
            self.generate_rounds(game_score, info, &canvas, context);
            div.append_child(&canvas).unwrap();
        }
    }
    /// Generate the rounds
    /// # Arguments
    /// * `game_score` - The game score
    /// * `info` - The game info
    /// * `canvas` - The canvas element
    /// * `context` - The canvas context
    /// # Example
    /// ```
    /// self.generate_rounds(&game_score, &info, &canvas, context);
    /// ```
    fn generate_rounds(
        self,
        game_score: &[GameScore],
        info: &GameInfo,
        canvas: &HtmlCanvasElement,
        context: CanvasRenderingContext2d,
    ) {
        let switch_icon = get_html_image_element_by_id("Switch").unwrap();
        let mut draw_switch = false;
        let mut overtime = 23;
        let scaling_factor = 0.8;
        const INIT_TRANSLATE_X: f64 = 20.0;
        const INIT_TRANSLATE_Y: f64 = 2.0;
        const INIT_TRANSLATE_OT: f64 = 21.0;

        canvas.set_width(self.initial_canvas_width);
        let mut present_round = false;
        self.calculate_canvas_width(info, canvas, scaling_factor);
        let mut overtime_count = 0;
        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        for (i, val) in game_score.iter().enumerate() {
            context.save();
            context
                .translate(INIT_TRANSLATE_X, INIT_TRANSLATE_Y)
                .unwrap();
            if i >= 12 {
                if !draw_switch {
                    self.draw_switch_icon(&switch_icon, scaling_factor, i as f64);
                    draw_switch = true;
                }
                context.translate(INIT_TRANSLATE_X, 0.0).unwrap();
            }
            if i >= overtime {
                if overtime % 2 == 0 {
                    self.draw_overtime_label(&context, scaling_factor, overtime_count, i as f64);
                    overtime_count += 1;
                    console_log!("Overtime: {}", overtime_count);
                }
                overtime += 1;
                context
                    .translate(INIT_TRANSLATE_OT * overtime_count as f64, 0.0)
                    .unwrap();
            }
            self.draw_round_info(&mut present_round, scaling_factor, &i, val);
            context.restore();
        }
    }
    /// Draw the round info
    /// # Arguments
    /// * `context` - The canvas context
    /// * `scaling_factor` - The scaling factor
    /// * `i` - The index
    /// * `val` - The game score
    /// # Example
    /// ```
    /// self.draw_round_info(&context, scaling_factor, &i, &val);
    /// ```
    fn draw_round_info(
        self,
        present_round: &mut bool,
        scaling_factor: f64,
        i: &usize,
        val: &GameScore,
    ) {
        let (_, context) = self.get_rounds_display_canvas_context();
        let mut text_colour = identify_team(val.round_win_status, false);
        if val.round_win_status == 2 && !*present_round {
            text_colour = "#BC544B";
            *present_round = true;
        }
        context.set_text_align("center");
        context.set_fill_style(&JsValue::from_str(text_colour));
        context.set_font(format!("{}px sans-serif", self.text_size * scaling_factor).as_str());
        context
            .fill_text(
                format!("{}", i + 1).as_str(),
                *i as f64 * self.gap_size * scaling_factor,
                self.text_size * scaling_factor * 1.125,
            )
            .unwrap();
        context.begin_path();
        context.set_stroke_style(&JsValue::from_str(identify_team(
            val.round_win_status,
            false,
        )));
        context.rect(
            (*i as f64 * self.gap_size * scaling_factor) - self.rect_size * scaling_factor / 2.0,
            0.0,
            self.rect_size * scaling_factor,
            self.rect_size * scaling_factor,
        );
        context.stroke();
    }
    /// Draw the overtime label
    /// # Arguments
    /// * `context` - The canvas context
    /// * `scaling_factor` - The scaling factor
    /// * `overtime_count` - The overtime count
    /// * `i` - The index
    /// # Example
    /// ```
    /// self.draw_overtime_label(&context, scaling_factor, overtime_count, i as f64);
    /// ```
    fn draw_overtime_label(
        self,
        context: &CanvasRenderingContext2d,
        scaling_factor: f64,
        overtime_count: i32,
        i: f64,
    ) {
        context.set_font(format!("bold {}px sans-serif", self.text_size * scaling_factor).as_str());
        context.set_fill_style(&JsValue::from_str("white"));
        context
            .fill_text(
                format!("OT{}", overtime_count + 1).as_str(),
                (i * self.gap_size * scaling_factor) + (20.0 * overtime_count as f64)
                    - (self.rect_size * scaling_factor),
                self.text_size * scaling_factor * 1.125,
            )
            .unwrap();
    }
    /// Draw the switch icon
    /// # Arguments
    /// * `switch_icon` - The switch icon
    /// * `scaling_factor` - The scaling factor
    /// * `i` - The index
    /// # Example
    /// ```
    /// self.draw_switch_icon(&switch_icon, scaling_factor, i as f64);
    /// ```
    fn draw_switch_icon(&self, switch_icon: &HtmlImageElement, scaling_factor: f64, i: f64) {
        let (_, context) = self.get_rounds_display_canvas_context();

        let (off_canvas, off_context) =
            get_offscreen_canvas_context(switch_icon.width(), switch_icon.height());
        off_context
            .draw_image_with_html_image_element(switch_icon, 0.0, 0.0)
            .unwrap();
        set_image_colour(off_context, switch_icon.clone(), 0.0, 0.0, "white");
        let image_bitmap = off_canvas.transfer_to_image_bitmap().unwrap();
        context
            .draw_image_with_image_bitmap_and_dw_and_dh(
                &image_bitmap,
                (i * self.gap_size * scaling_factor) + 3.0 - self.rect_size * scaling_factor,
                0.0,
                self.rect_size * scaling_factor,
                self.rect_size * scaling_factor,
            )
            .unwrap();
    }
    /// Calculate the canvas width
    /// # Arguments
    /// * `info` - The game info
    /// * `canvas` - The canvas element
    /// * `scaling_factor` - The scaling factor
    /// # Example
    /// ```
    /// self.calculate_canvas_width(&info, &canvas, scaling_factor);
    /// ```
    fn calculate_canvas_width(
        &self,
        info: &GameInfo,
        canvas: &HtmlCanvasElement,
        scaling_factor: f64,
    ) {
        if info.max_rounds > 24 {
            canvas
                .set_width(1000 + ((info.max_rounds - 24) * (60.0 * scaling_factor) as i32) as u32);
        };
    }
}

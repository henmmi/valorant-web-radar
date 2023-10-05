use crate::components::game_data;
use crate::components::game_data::{GameInfo, GameScore};
use crate::components::player::identify_team;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct GameStatus {
    text_size: f64,
    text_colour: String,
    text_font: String,
    t_colour: String,
    ct_colour: String,
}

impl GameStatus {
    pub fn new() -> Self {
        GameStatus {
            text_size: 20.0,
            text_colour: "#FFFFFF".to_string(),
            text_font: "sans-serif".to_string(),
            t_colour: identify_team(0, false).to_string(),
            ct_colour: identify_team(1, false).to_string(),
        }
    }
    /// Get the canvas context for the game state
    /// # Example
    /// ```
    /// let (canvas, context) = self.get_game_state_canvas_context();
    /// ```
    fn get_game_state_canvas_context(&self) -> (HtmlCanvasElement, CanvasRenderingContext2d) {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("game_state").unwrap();
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
    /// Creates a display for information on the current game state
    /// # Arguments
    /// * `info` - The game info
    /// # Example
    /// ```
    /// self.create_game_state_row(&info);
    /// ```
    pub fn create_game_state_row(&self, info: &GameInfo, spike_status: &i32) {
        let (canvas, _) = self.get_game_state_canvas_context();
        canvas.set_width(300);
        self.add_game_timer(info, spike_status);
    }

    fn add_game_timer(&self, info: &GameInfo, spike_status: &i32) {
        let (canvas, context) = self.get_game_state_canvas_context();
        if *spike_status == 1 {
            context.set_fill_style(&JsValue::from_str("#BC544B"));
        } else {
            context.set_fill_style(&JsValue::from_str(self.text_colour.as_str()));
        }
        context.set_font(format!("{}px {}", self.text_size, self.text_font).as_str());
        context.set_text_align("left");
        context
            .fill_text(
                self.convert_time(info.round_time[0]).as_str(),
                canvas.width() as f64 / 2.0 - self.text_size * 1.5,
                canvas.height() as f64 / 2.0,
            )
            .unwrap();
    }
    /// Convert the time to a string
    /// # Arguments
    /// * `time` - The time
    /// # Example
    /// ```
    /// self.convert_time(time);
    /// ```
    fn convert_time(&self, time: f64) -> String {
        let total_seconds = time.round() as i64;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{:}:{:02}", minutes, seconds)
    }
    /// Add the score and round number to game_state canvas
    /// # Arguments
    /// * `info` - The game score
    /// # Example
    /// ```
    /// self.add_score_and_round_number(&info);
    /// ```
    pub fn add_score_and_round_number(&self, info: &[GameScore]) {
        let (canvas, context) = self.get_game_state_canvas_context();
        let (t_score, ct_score) = game_data::get_score(info);
        context.set_text_align("center");
        context.set_fill_style(&JsValue::from_str(self.text_colour.as_str()));
        context.set_font(format!("{}px {}", self.text_size / 2.0, self.text_font).as_str());
        context
            .fill_text(
                format!("Round {}", t_score + ct_score + 1).as_str(),
                canvas.width() as f64 / 2.0 - self.text_size / 2.0,
                canvas.height() as f64 * 0.9,
            )
            .unwrap();
        context.set_text_align("center");
        context.set_font(format!("{}px {}", self.text_size * 2.0, self.text_font).as_str());
        context.set_fill_style(&JsValue::from_str(self.t_colour.as_str()));
        context
            .fill_text(
                format!("{}", t_score).as_str(),
                canvas.width() as f64 * 0.5 - 60.0,
                canvas.height() as f64 * 0.9,
            )
            .unwrap();
        context.set_text_align("center");
        context.set_fill_style(&JsValue::from_str(self.ct_colour.as_str()));
        context
            .fill_text(
                format!("{}", ct_score).as_str(),
                canvas.width() as f64 * 0.5 + 57.5 - self.text_size,
                canvas.height() as f64 * 0.9,
            )
            .unwrap();
    }
}

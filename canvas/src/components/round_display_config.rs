use crate::components::elements::{get_html_image_element_by_id, get_offscreen_canvas_context};
use crate::components::game_data::{GameInfo, GameScore};
use crate::components::player::{identify_team, set_image_colour};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

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
        self.generate_rounds(game_score, info, &canvas, context);
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

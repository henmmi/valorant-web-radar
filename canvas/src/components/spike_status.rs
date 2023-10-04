use crate::components::canvas::{get_number, get_radian_angle, ROTATION_ANGLE};
use crate::components::elements::{
    get_canvas_context_document, get_html_image_element_by_id, get_offscreen_canvas_context,
};
use crate::components::player::set_image_colour;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub struct SpikeStatus {
    x: f64,
    y: f64,
    time: f64,
}

impl SpikeStatus {
    pub fn new(x: f64, y: f64, time: f64) -> Self {
        Self { x, y, time }
    }

    pub fn draw_spike(&self) {
        const SPIKE_SIZE: f64 = 32.0;
        const FONT_SIZE: f64 = 12.0;
        let mut image_colour = "white";
        if self.time < 8.0 {
            image_colour = "#BC544B";
        }
        let (_, context, _) = get_canvas_context_document();
        let spike_icon = get_html_image_element_by_id("Spike").unwrap();
        context.save();
        context.translate(self.x, self.y).unwrap();
        let angle_rad = get_radian_angle(-get_number(&ROTATION_ANGLE));
        context.rotate(angle_rad).unwrap();
        let (off_canvas, off_context) =
            get_offscreen_canvas_context(spike_icon.width(), spike_icon.height());
        off_context
            .draw_image_with_html_image_element(&spike_icon, 0.0, 0.0)
            .unwrap();
        set_image_colour(off_context, spike_icon, 0.0, 0.0, image_colour);
        let image_bitmap = off_canvas.transfer_to_image_bitmap().unwrap();
        context
            .draw_image_with_image_bitmap_and_dw_and_dh(
                &image_bitmap,
                -SPIKE_SIZE / 2.0,
                SPIKE_SIZE / 2.0,
                SPIKE_SIZE,
                SPIKE_SIZE,
            )
            .unwrap();
        context.set_font(format!("{}px Arial", FONT_SIZE).as_str());
        context.set_fill_style(&JsValue::from_str(image_colour));
        context
            .fill_text(&format!("{:.1}", self.time), -FONT_SIZE, SPIKE_SIZE / 2.0)
            .unwrap();
        context.restore();
    }
}

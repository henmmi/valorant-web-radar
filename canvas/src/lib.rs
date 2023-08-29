use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::HtmlCanvasElement;

fn get_canvas_context_document() -> (HtmlCanvasElement, web_sys::CanvasRenderingContext2d, web_sys::Document) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    (canvas, context, document)
}
#[wasm_bindgen()]
pub fn display_image(imagePath: &str) {
    let (canvas, context, document) = get_canvas_context_document();
    
    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    
    let image = document.create_element("img")
        .unwrap()
        .dyn_into::<web_sys::HtmlImageElement>()
        .unwrap();
    
    image.set_src(imagePath);
    
    let image_clone = image.clone();
    let context_clone = context.clone();
    let canvas_clone = canvas.clone();
    let closure = Closure::wrap(Box::new(move || {
        // Get original image dimensions
        let original_width = image_clone.width() as f64;
        let original_height = image_clone.height() as f64;
        
        // Calculate scaling factors
        let scale_width = canvas_clone.width() as f64 / original_width;
        let scale_height = canvas_clone.height() as f64 / original_height;
        
        // Use the smallest scaling factor
        let scale = scale_width.min(scale_height);
        
        // Calculate new image dimensions
        let new_width = original_width * scale;
        let new_height = original_height * scale;
        context_clone.draw_image_with_html_image_element_and_dw_and_dh(&image_clone, 0.0, 0.0, new_width, new_height)
            .expect("Failed to draw the image");
    }) as Box<dyn FnMut()>);
        
        image.set_onload(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
}
#[wasm_bindgen()]
pub fn display_player_position(x: f64, y: f64){
    let (canvas, context, document) = get_canvas_context_document();
    context.begin_path();
    context.arc(x, y, 10.0, 0.0, f64::consts::PI * 2.0).unwrap();
    context.set_fill_style(&JsValue::from_str("red"));
    context.fill();
    context.stroke();
}
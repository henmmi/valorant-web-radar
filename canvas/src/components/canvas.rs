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
pub fn change_map(image_path: &str) -> String {
    let (canvas, context, document) = get_canvas_context_document();
    
    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    
    let image = document.create_element("img")
        .unwrap()
        .dyn_into::<web_sys::HtmlImageElement>()
        .unwrap();
    
    image.set_src(image_path);
    let image_clone = image.clone();
    let closure = Closure::wrap(Box::new(move || {
        // Get original image dimensions
        context.draw_image_with_html_image_element_and_dw_and_dh(&image_clone, 0.0, 0.0, 1024.0, 1024.0)
            .expect("Failed to draw the image");
    }) as Box<dyn FnMut()>);
    
    image.set_onload(Some(closure.as_ref().unchecked_ref()));
    closure.forget();
    return image_path.to_string();
}

#[wasm_bindgen()]
pub fn display_player_position(id: u32, x: f64, y: f64){
    let (_, context, _) = get_canvas_context_document();
    
    // Determine team colour
    let team = match id {
        1..=5 => "red",
        6..=10 => "blue",
        _ => "black",
    };
    
    context.begin_path();
    context.arc(x, y, 10.0, 0.0, f64::consts::PI * 2.0).unwrap();
    context.set_fill_style(&JsValue::from_str(&team));
    context.fill();
    
    // Draw the circle's outline in white
    context.set_stroke_style(&JsValue::from_str("white"));
    context.stroke();
    
    // Configure the text's style
    context.set_font("16px sans-serif");
    context.set_text_align("center");
    context.set_text_baseline("middle");
    context.set_fill_style(&JsValue::from_str("white"));
    context.fill_text(&id.to_string(), x, y).unwrap();
}
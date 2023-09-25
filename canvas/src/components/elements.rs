use super::macros::{console_log, log};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlCanvasElement, HtmlDivElement, HtmlImageElement};

/// Getters for the canvas, context, and document
/// # Returns
/// * `canvas` - The canvas element
/// * `context` - The canvas context
/// * `document` - The document
/// # Example
/// ```
/// let (canvas, context, document) = get_canvas_context_document();
/// ```
pub fn get_canvas_context_document() -> (
    HtmlCanvasElement,
    web_sys::CanvasRenderingContext2d,
    web_sys::Document,
) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
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
/// Create a HTML image element
/// # Arguments
/// * `id` - The id of the image element
/// * `url` - The url of the image element
/// * `class` - The class of the image element
/// # Example
/// ```
/// let img_elem = create_html_image_element("id", "url", "class");
/// ```
pub fn create_html_image_element(
    id: &str,
    url: &str,
    class: &str,
) -> Result<HtmlImageElement, JsValue> {
    let (_, _, document) = get_canvas_context_document();
    let element = document.create_element("img")?;
    let img_elem = element.dyn_into::<HtmlImageElement>()?;
    img_elem.set_id(id);
    img_elem.set_class_name(class);
    img_elem.set_src(url);
    Ok(img_elem)
}
/// Get the HTML image element by id
/// # Arguments
/// * `id` - The id of the image element
/// # Example
/// ```
/// let img_elem = get_html_image_element_by_id("id");
/// ```
pub fn get_html_image_element_by_id(id: &str) -> Result<HtmlImageElement, ()> {
    let (_, _, document) = get_canvas_context_document();
    let element = match document.get_element_by_id(id) {
        Some(element) => element,
        None => panic!("No img element found with id: {}", id),
    };
    match element.dyn_into::<HtmlImageElement>() {
        Ok(img_elem) => Ok(img_elem),
        Err(_) => Err(console_log!(
            "Element with id: {} is not an HtmlImageElement",
            id
        )),
    }
}
/// Get the HTML div element by id
/// # Arguments
/// * `id` - The id of the div element
/// # Example
/// ```
/// let div_elem = get_div_element_by_id("id");
/// ```
pub fn get_div_element_by_id(id: &str) -> Result<HtmlDivElement, ()> {
    let (_, _, document) = get_canvas_context_document();
    let element = match document.get_element_by_id(id) {
        Some(element) => element,
        None => panic!("No div element found with id: {}", id),
    };
    match element.dyn_into::<HtmlDivElement>() {
        Ok(div_elem) => Ok(div_elem),
        Err(_) => Err(console_log!(
            "Element with id: {} is not an HtmlDivElement",
            id
        )),
    }
}

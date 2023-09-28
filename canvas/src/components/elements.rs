use super::macros::{console_log, log};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    HtmlCanvasElement, HtmlCollection, HtmlDivElement, HtmlImageElement, HtmlInputElement,
    OffscreenCanvas, OffscreenCanvasRenderingContext2d,
};

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
pub fn create_html_div_element(id: &str, class: &str) -> Result<HtmlDivElement, JsValue> {
    let (_, _, document) = get_canvas_context_document();
    let element = document.create_element("div")?;
    let div_elem = element.dyn_into::<HtmlDivElement>()?;
    div_elem.set_id(id);
    div_elem.set_class_name(class);
    Ok(div_elem)
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
        Some(element) => {
            console_log!("Found element with id: {:?}", element.dyn_ref::<JsValue>());
            element
        }
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
/// Get the HTML input element by id
/// # Arguments
/// * `id` - The id of the input element
/// # Example
/// ```
/// let input_elem = get_input_element_by_id("id");
/// ```
pub fn get_input_element_by_id(id: &str) -> Result<HtmlInputElement, ()> {
    let (_, _, document) = get_canvas_context_document();
    let element = match document.get_element_by_id(id) {
        Some(element) => element,
        None => panic!("No input element found with id: {}", id),
    };
    match element.dyn_into::<HtmlInputElement>() {
        Ok(input_elem) => Ok(input_elem),
        Err(_) => Err(console_log!(
            "Element with id: {} is not an HtmlInputElement",
            id,
        )),
    }
}
/// Returns a DivElement by class if it exists
pub fn get_elements_by_class(class: &str) -> Option<HtmlCollection> {
    let (_, _, document) = get_canvas_context_document();
    Some(document.get_elements_by_class_name(class))
}
/// Create OffscreenCanvas and OffscreenCanvasRenderingContext2d objects
/// # Arguments
/// * `width` - The width of the canvas
/// * `height` - The height of the canvas
/// # Example
/// ```
/// let (offscreen_canvas, offscreen_context) = get_offscreen_canvas_context(1920, 1080);
/// ```
pub fn get_offscreen_canvas_context(
    width: u32,
    height: u32,
) -> (OffscreenCanvas, OffscreenCanvasRenderingContext2d) {
    let offscreen_canvas: OffscreenCanvas = OffscreenCanvas::new(width, height).unwrap();
    offscreen_canvas.set_width(width);
    offscreen_canvas.set_height(height);

    let offscreen_context = offscreen_canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<OffscreenCanvasRenderingContext2d>()
        .unwrap();

    (offscreen_canvas, offscreen_context)
}

/// Deletes all content in a HTMLCollection
pub fn delete_collection_contents(class: &str) {
    let (_, _, document) = get_canvas_context_document();
    // Clear player info every instance
    if let Some(collection) = get_elements_by_class(class) {
        for i in 0..collection.length() as usize {
            let range = document.create_range().unwrap();
            range
                .select_node_contents(&collection.item(i as u32).unwrap())
                .unwrap();
            range.delete_contents().unwrap();
        }
    }
}

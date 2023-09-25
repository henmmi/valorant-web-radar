use super::macros::{console_log, log};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlDivElement, HtmlImageElement};

pub fn create_html_image_element(
    id: &str,
    url: &str,
    class: &str,
) -> Result<HtmlImageElement, JsValue> {
    let (_, _, document) = super::element::get_canvas_context_document();
    let element = document.create_element("img")?;
    let img_elem = element.dyn_into::<HtmlImageElement>()?;
    img_elem.set_id(id);
    img_elem.set_class_name(class);
    img_elem.set_src(url);
    Ok(img_elem)
}

pub fn get_html_image_element_by_id(id: &str) -> Result<HtmlImageElement, ()> {
    let (_, _, document) = super::element::get_canvas_context_document();
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

pub fn get_div_element_by_id(id: &str) -> Result<HtmlDivElement, ()> {
    let (_, _, document) = super::element::get_canvas_context_document();
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

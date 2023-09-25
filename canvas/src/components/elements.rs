use wasm_bindgen::JsCast;

pub fn create_html_image_element(id: &str, url: &str, class: &str) -> web_sys::HtmlImageElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let image = document
        .create_element("img")
        .unwrap()
        .dyn_into::<web_sys::HtmlImageElement>()
        .unwrap();
    image.set_id(id);
    image.set_class_name(class);
    image.set_src(url);
    image
}

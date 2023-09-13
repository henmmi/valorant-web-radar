pub mod canvas;
pub mod websocket;

pub mod macros {
    /// A macro to provide `println!(..)`-style syntax for `console.log` logging.
    /// # Example
    /// ```
    /// use super::macros::{console_log, log};
    /// console_log!("Hello {}!", "world");
    /// ```
    #[macro_export]
    macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
    pub(crate) use console_log;
    /// 'js_namespace' used to bind 'console.log(...)' instead of
    /// 'log(...)'
    /// Invoked as console.log in JS
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: &str);
    }
}
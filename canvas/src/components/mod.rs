pub mod canvas;
mod dead_players;
pub mod elements;
pub mod game_data;
mod game_status;
pub mod player;
pub mod player_data;
pub mod player_table;
mod preloader;
mod round_display_config;
mod spike_status;
pub mod ui_element;
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

mod app;
mod utils;

use wasm_bindgen::prelude::*;
#[wasm_bindgen(start)]
pub fn main() {
    yew::start_app::<app::App>();
}

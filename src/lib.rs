use leptos::*;
use wasm_bindgen::prelude::*;

mod app;
mod product;
use app::App;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once(); // Покажет ошибки в консоли браузера
    mount_to_body(App);
}

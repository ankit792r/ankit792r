pub mod app;

pub mod pages;
pub mod comps;
// pub mod models;

pub mod database;
pub mod model;
pub mod models;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

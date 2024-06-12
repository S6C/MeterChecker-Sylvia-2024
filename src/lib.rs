// Allow non snake case to avoid complaints about component names
#![allow(non_snake_case)]

pub mod app;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod octopus;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}

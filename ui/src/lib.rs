use wasm_bindgen::prelude::*;

mod components;

use components::app::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<App>();
    Ok(())
}

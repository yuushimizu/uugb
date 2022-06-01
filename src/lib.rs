#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start_wasm(canvas_id: &str) -> Result<(), wasm_bindgen::JsValue> {
    gui::start_wasm(canvas_id)
}

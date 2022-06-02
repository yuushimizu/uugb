mod utils;

use gui::command;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Handle(command::Sender);

#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<Handle, wasm_bindgen::JsValue> {
    utils::set_panic_hook();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Warn));
    Ok(Handle(gui::start_wasm(canvas_id)?))
}

#[wasm_bindgen]
pub fn set_rom(handle: &mut Handle, bytes: &[u8]) {
    handle.0.send(command::Command::Rom(bytes.to_vec()));
}

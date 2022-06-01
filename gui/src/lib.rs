mod app;
mod audio;
mod renderer;

use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
pub fn start_native(rom_filepath: Option<PathBuf>) {
    eframe::run_native(
        "u_u GB",
        eframe::NativeOptions {
            initial_window_size: Some(eframe::egui::Vec2::new(
                core::display_size().x as f32 * 2.0,
                core::display_size().y as f32 * 2.0,
            )),
            drag_and_drop_support: true,
            ..Default::default()
        },
        Box::new(|_| {
            let mut app = Box::new(app::App::default());
            if let Some(rom_filepath) = rom_filepath {
                app.boot(&rom_filepath);
            }
            app
        }),
    );
}

#[cfg(target_arch = "wasm32")]
pub fn start_wasm(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    eframe::start_web(canvas_id, Box::new(|_| Box::new(app::App::default())))
}

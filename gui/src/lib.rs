mod app;
mod audio;
mod renderer;

#[cfg(not(target_arch = "wasm32"))]
pub fn start_native() {
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
        Box::new(|_| Box::new(app::App::default())),
    );
}

#[cfg(target_arch = "wasm32")]
pub fn start_wasm(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    eframe::start_web(canvas_id, Box::new(|_| Box::new(app::App::default())))
}

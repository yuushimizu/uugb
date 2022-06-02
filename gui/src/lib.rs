mod app;
mod audio;
mod renderer;

pub mod command;

#[cfg(not(target_arch = "wasm32"))]
pub fn start_native() {
    let (_, receiver) = command::channels();
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
        Box::new(|_| Box::new(app::App::new(receiver))),
    );
}

#[cfg(target_arch = "wasm32")]
pub fn start_wasm(canvas_id: &str) -> Result<command::Sender, eframe::wasm_bindgen::JsValue> {
    let (sender, receiver) = command::channels();
    eframe::start_web(canvas_id, Box::new(|_| Box::new(app::App::new(receiver))))?;
    Ok(sender)
}

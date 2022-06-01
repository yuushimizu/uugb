mod audio;
mod renderer;

use audio::AudioOutput;
use renderer::Renderer;

use core::{Cartridge, GameBoy};
use eframe::{egui, epi};
use std::{
    fs::File,
    io::Read,
    path::Path,
    time::{Duration, SystemTime},
};

const NANOS_PER_FRAME: u128 = (1_000_000_000f64 / core::FRAME_RATE) as u128;

pub struct App {
    game_boy: Option<GameBoy>,
    renderer: Renderer,
    texture: Option<egui::TextureHandle>,
    audio_output: AudioOutput,
    last_frame_time: SystemTime,
}

impl Default for App {
    fn default() -> Self {
        Self {
            game_boy: None,
            renderer: Default::default(),
            texture: None,
            audio_output: AudioOutput::new().unwrap(),
            last_frame_time: SystemTime::now(),
        }
    }
}

impl App {
    pub fn boot(&mut self, rom_filepath: &Path) {
        let mut rom = Vec::new();
        File::open(rom_filepath)
            .unwrap()
            .read_to_end(&mut rom)
            .unwrap();
        self.game_boy = Some(GameBoy::new(Cartridge::new(rom.into()).unwrap()))
    }

    pub fn run(rom_filepath: &Option<&Path>) {
        let mut app = Self::default();
        if let Some(rom_filepath) = rom_filepath {
            app.boot(rom_filepath);
        }
        let native_options = eframe::NativeOptions {
            initial_window_size: Some(eframe::egui::Vec2::new(
                core::display_size().x as f32 * 2.0,
                core::display_size().y as f32 * 2.0,
            )),
            drag_and_drop_support: true,
            ..Default::default()
        };
        eframe::run_native(Box::new(app), native_options);
    }

    fn advance_frame(&mut self, button_state: core::ButtonState) {
        if let Some(ref mut game_boy) = self.game_boy {
            game_boy.set_button_state(button_state);
            let current_time = SystemTime::now();
            let duration = current_time
                .duration_since(self.last_frame_time)
                .map(|duration| duration.as_nanos())
                .unwrap_or(0);
            if duration >= NANOS_PER_FRAME {
                for _ in 0..core::M_CYCLES_PER_FRAME {
                    game_boy.tick(
                        &mut self.renderer,
                        &mut self.audio_output,
                        &mut core::NoSerialConnection,
                    );
                }
                self.last_frame_time += Duration::from_nanos(NANOS_PER_FRAME as u64);
            }
        }
    }
}

fn button_state(context: &egui::Context) -> core::ButtonState {
    use egui::Key::*;
    let keys = &context.input().keys_down;
    core::ButtonState {
        up: keys.contains(&ArrowUp),
        down: keys.contains(&ArrowDown),
        left: keys.contains(&ArrowLeft),
        right: keys.contains(&ArrowRight),
        a: keys.contains(&X),
        b: keys.contains(&Z),
        start: keys.contains(&Enter),
        select: keys.contains(&Space),
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "GB Emulator"
    }

    fn setup(
        &mut self,
        _context: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    fn update(&mut self, context: &egui::Context, _frame: &epi::Frame) {
        self.advance_frame(button_state(context));
        egui::CentralPanel::default()
            .frame(egui::Frame {
                margin: egui::style::Margin::symmetric(24f32, 16f32),
                fill: egui::Color32::LIGHT_GRAY,
                ..Default::default()
            })
            .show(context, |ui| {
                egui::Frame {
                    stroke: egui::Stroke::new(4f32, egui::Color32::DARK_GRAY),
                    ..Default::default()
                }
                .show(ui, |ui| {
                    let texture = self.texture.get_or_insert_with(|| {
                        ui.ctx().load_texture("game-frame", self.renderer.image())
                    });
                    texture.set(self.renderer.image());
                    ui.image(texture, ui.max_rect().max - ui.max_rect().min)
                        .request_focus();
                });
            });
        context.request_repaint();
    }
}

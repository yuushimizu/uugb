use core::{Cartridge, GameBoy};
use eframe::{egui, epi};
use std::{
    fs::File,
    io::Read,
    path::Path,
    time::{Duration, SystemTime},
};

struct Renderer {
    rendered_image: egui::ColorImage,
    rendering_image: egui::ColorImage,
}

impl Renderer {
    fn image(&self) -> egui::ColorImage {
        self.rendered_image.clone()
    }
}

impl Default for Renderer {
    fn default() -> Self {
        let size = [
            core::display_size().x as usize,
            core::display_size().y as usize,
        ];
        Self {
            rendered_image: egui::ColorImage::new(size, egui::Color32::BLACK),
            rendering_image: egui::ColorImage::new(size, egui::Color32::BLACK),
        }
    }
}

impl core::Renderer for Renderer {
    fn render(&mut self, position: core::Vec2, color: core::Color) {
        use core::Color::*;
        self.rendering_image.pixels
            [position.y as usize * core::display_size().x as usize + position.x as usize] =
            match color {
                White => egui::Color32::from_rgb(134, 163, 90),
                LightGray => egui::Color32::from_rgb(111, 137, 79),
                DarkGray => egui::Color32::from_rgb(88, 117, 79),
                Black => egui::Color32::from_rgb(50, 84, 79),
            };
        if position.x == core::display_size().x - 1 && position.y == core::display_size().y - 1 {
            std::mem::swap(&mut self.rendered_image, &mut self.rendering_image);
        }
    }
}

const NANOS_PER_FRAME: u128 = (1_000_000_000f64 / 59.7f64) as u128;

pub struct App {
    game_boy: Option<GameBoy>,
    renderer: Renderer,
    texture: Option<egui::TextureHandle>,
    last_frame_time: SystemTime,
}

impl App {
    pub fn new(filepath: &Path) -> Self {
        let mut rom = Vec::new();
        File::open(filepath).unwrap().read_to_end(&mut rom).unwrap();
        Self {
            game_boy: Some(GameBoy::new(Cartridge::new(rom.into()).unwrap())),
            renderer: Default::default(),
            texture: None,
            last_frame_time: SystemTime::now(),
        }
    }

    fn advance_frame(&mut self) {
        let current_time = SystemTime::now();
        let duration = current_time
            .duration_since(self.last_frame_time)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        if duration >= NANOS_PER_FRAME {
            if let Some(ref mut game_boy) = self.game_boy {
                for _ in 0..core::M_CYCLES_PER_FRAME {
                    game_boy.tick(&mut self.renderer, &mut core::serial::NoSerialConnection);
                }
            }
            self.last_frame_time += Duration::from_nanos(NANOS_PER_FRAME as u64);
        }
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
        self.advance_frame();
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
                    ui.image(texture, ui.max_rect().max - ui.max_rect().min);
                });
            });
        context.request_repaint();
    }
}

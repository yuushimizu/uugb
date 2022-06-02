use crate::{audio::AudioOutput, command, renderer::Renderer};
use core::{Cartridge, GameBoy};
use eframe::egui;
use std::rc::Rc;

struct State {
    game_boy: GameBoy,
    renderer: Renderer,
    audio_output: AudioOutput,
    processed_m_cycles: u64,
    period_start_time_ms: f64,
}

impl State {
    pub fn new(rom: Rc<Vec<u8>>) -> Option<Self> {
        Some(Self {
            game_boy: GameBoy::new(
                Cartridge::new(rom)
                    .map_err(|error| {
                        log::warn!("Could not load the rom: {:?}", error);
                        error
                    })
                    .ok()?,
            ),
            renderer: Default::default(),
            audio_output: Default::default(),
            processed_m_cycles: 0,
            period_start_time_ms: instant::now(),
        })
    }

    fn advance_cycles(&mut self, button_state: core::ButtonState) {
        self.game_boy.set_button_state(button_state);
        let current_time = instant::now();
        let target_m_cycles = ((core::M_CYCLES as f64) * (current_time - self.period_start_time_ms)
            / 1000f64)
            .floor() as u64;
        while self.processed_m_cycles < target_m_cycles {
            self.game_boy.tick(
                &mut self.renderer,
                &mut self.audio_output,
                &mut core::NoSerialConnection,
            );
            self.processed_m_cycles += 1;
        }
        while self.processed_m_cycles > core::M_CYCLES {
            self.processed_m_cycles -= core::M_CYCLES;
            self.period_start_time_ms += 1000f64;
        }
    }
}

pub struct App {
    state: Option<State>,
    texture: Option<egui::TextureHandle>,
    receiver: command::Receiver,
}

#[cfg(not(target_arch = "wasm32"))]
fn dropped_file_bytes(file: &egui::DroppedFile) -> Option<Rc<Vec<u8>>> {
    use std::{fs::File, io::Read};
    let mut bytes = Vec::new();
    file.path
        .as_ref()
        .and_then(|path| File::open(path).ok())?
        .read_to_end(&mut bytes)
        .ok()?;
    Some(Rc::new(bytes))
}

#[cfg(target_arch = "wasm32")]
fn dropped_file_bytes(file: &egui::DroppedFile) -> Option<Rc<Vec<u8>>> {
    file.bytes.as_ref().map(|bytes| Rc::new(bytes.to_vec()))
}

impl App {
    pub fn new(receiver: command::Receiver) -> Self {
        Self {
            state: None,
            texture: None,
            receiver,
        }
    }

    pub fn boot(&mut self, rom: Rc<Vec<u8>>) {
        self.state = State::new(rom)
    }

    fn advance_cycles(&mut self, button_state: core::ButtonState) {
        if let Some(state) = &mut self.state {
            state.advance_cycles(button_state);
        }
    }

    fn process_dropped_file(&mut self, context: &egui::Context) {
        if let Some(bytes) = context
            .input()
            .raw
            .dropped_files
            .first()
            .and_then(|file| dropped_file_bytes(file))
        {
            self.boot(bytes);
        }
    }

    fn process_command(&mut self) {
        if let Ok(command) = self.receiver.try_recv() {
            use command::Command::*;
            match command {
                Rom(bytes) => self.boot(Rc::new(bytes)),
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

impl eframe::App for App {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        self.advance_cycles(button_state(context));
        egui::CentralPanel::default().show(context, |ui| {
            egui::Frame::default().show(ui, |ui| {
                let texture = self.texture.get_or_insert_with(|| {
                    ui.ctx()
                        .load_texture("game-frame", Renderer::default_image())
                });
                if let Some(state) = &self.state {
                    texture.set(state.renderer.image());
                }
                ui.image(texture, ui.max_rect().max - ui.max_rect().min)
                    .request_focus();
            });
        });
        self.process_dropped_file(context);
        self.process_command();
        context.request_repaint();
    }

    #[cfg(target_arch = "wasm32")]
    fn max_size_points(&self) -> egui::Vec2 {
        eframe::egui::Vec2::new(core::display_size().x as f32, core::display_size().y as f32)
    }
}

mod control;
mod coordinate;
mod interrupt_source;
mod palette;
mod renderer;

pub mod oam;
pub mod vram;

pub use coordinate::Coordinate;
pub use palette::Palette;
pub use renderer::Renderer;

use control::Control;
use interrupt_source::InterruptSource;
use vram::Vram;

use crate::interrupt::{Interrupt, InterruptController};

const WIDTH: u8 = 160;

const HEIGHT: u8 = 144;

const LINES_PER_FRAME: u8 = 154;

const CYCLES_PER_LINE: u64 = 456;

const OAM_SEARCH_CYCLES: u64 = 80;

const TRANSFER_CYCLES: u64 = 168;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Mode {
    HBlank = 0b00,
    VBlank = 0b01,
    OamSearch = 0b10,
    Transfer = 0b11,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Ppu {
    vram: Vram,
    control: Control,
    interrupt_source: InterruptSource,
    current_position: Coordinate,
    y_compare: u8,
    background_palette: Palette,
    object_palette0: Palette,
    object_palette1: Palette,
    scroll_position: Coordinate,
    window_position: Coordinate,
    cycles_in_line: u64,
}

impl Ppu {
    fn mode(&self) -> Mode {
        use Mode::*;
        if self.current_position.y >= HEIGHT {
            VBlank
        } else if self.cycles_in_line < OAM_SEARCH_CYCLES {
            OamSearch
        } else if self.cycles_in_line < OAM_SEARCH_CYCLES + TRANSFER_CYCLES {
            Transfer
        } else {
            HBlank
        }
    }

    fn background_pixel(&self, position: Coordinate) -> u8 {
        if !self.control.background_and_window_enabled() {
            0b00
        } else {
            self.vram
                .tile_map(
                    self.control.background_tile_map_area(),
                    self.control.background_tile_data_area(),
                )
                .pixel(position)
        }
    }

    fn render_pixel(&self, renderer: &mut impl Renderer) {
        let pixel = if self.control.background_and_window_enabled() {
            self.background_pixel(self.current_position)
        } else {
            0b00
        };
        renderer.render(self.current_position, pixel)
    }

    fn advance_cycle(&mut self) {
        self.cycles_in_line += 1;
        if self.cycles_in_line >= CYCLES_PER_LINE {
            self.current_position.y += 1;
            self.current_position.x = 0;
            self.cycles_in_line = 0;
            if self.current_position.y >= LINES_PER_FRAME {
                self.current_position.y = 0;
            }
        }
    }

    fn request_interrupt(
        &self,
        previous_mode: Mode,
        interrupt_controller: &mut InterruptController,
    ) {
        use Mode::*;
        if previous_mode != self.mode() && self.mode() == VBlank {
            interrupt_controller.request(Interrupt::VBlank);
        }
        if self.current_position.y == self.y_compare && self.interrupt_source.ly()
            || (previous_mode != self.mode()
                && match self.mode() {
                    HBlank => self.interrupt_source.hblank(),
                    VBlank => self.interrupt_source.vblank(),
                    OamSearch => self.interrupt_source.oam(),
                    _ => false,
                })
        {
            interrupt_controller.request(Interrupt::LcdStat);
        }
    }

    pub fn tick(
        &mut self,
        interrupt_controller: &mut InterruptController,
        renderer: &mut impl Renderer,
    ) {
        if !self.control.is_enabled() {
            return;
        }
        let current_mode = self.mode();
        if current_mode == Mode::Transfer {
            if self.current_position.x < WIDTH {
                self.render_pixel(renderer);
            }
            self.current_position.x += 1;
        }
        self.advance_cycle();
        self.request_interrupt(current_mode, interrupt_controller);
    }

    pub fn vram(&self) -> &Vram {
        &self.vram
    }

    pub fn vram_mut(&mut self) -> &mut Vram {
        &mut self.vram
    }

    pub fn control_bits(&self) -> u8 {
        self.control.bits()
    }

    pub fn set_control_bits(&mut self, value: u8) {
        self.control.set_bits(value);
        if !self.control.is_enabled() {
            self.current_position.y = 0;
        }
    }

    pub fn status_bits(&self) -> u8 {
        self.interrupt_source.bits() << 3
            | ((self.current_position.y == self.y_compare) as u8) << 2
            | (self.mode() as u8)
    }

    pub fn set_status_bits(&mut self, value: u8) {
        self.interrupt_source.set_bits(value >> 3);
    }

    pub fn current_y(&self) -> u8 {
        self.current_position.y
    }

    pub fn y_compare(&self) -> u8 {
        self.y_compare
    }

    pub fn background_palette(&self) -> &Palette {
        &self.background_palette
    }

    pub fn background_palette_mut(&mut self) -> &mut Palette {
        &mut self.background_palette
    }

    pub fn object_palette0(&self) -> &Palette {
        &self.object_palette0
    }

    pub fn object_palette0_mut(&mut self) -> &mut Palette {
        &mut self.object_palette0
    }

    pub fn object_palette1(&self) -> &Palette {
        &self.object_palette1
    }

    pub fn object_palette1_mut(&mut self) -> &mut Palette {
        &mut self.object_palette1
    }

    pub fn scroll_position(&self) -> Coordinate {
        self.scroll_position
    }

    pub fn scroll_position_mut(&mut self) -> &mut Coordinate {
        &mut self.scroll_position
    }

    pub fn set_y_compare(&mut self, value: u8) {
        self.y_compare = value;
    }

    pub fn window_position(&self) -> Coordinate {
        self.window_position
    }

    pub fn window_position_mut(&mut self) -> &mut Coordinate {
        &mut self.window_position
    }
}

mod control;
mod coordinate;
mod renderer;

pub mod oam;
pub mod vram;

pub use coordinate::Coordinate;
pub use renderer::Renderer;

use control::Control;
use vram::Vram;

use crate::{interrupt::InterruptController, util::bits::Bits};

const WIDTH: u8 = 160;

const HEIGHT: u8 = 144;

const CYCLES_PER_LINE: u64 = 456;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct InterruptSource {
    pub ly: bool,
    pub oam: bool,
    pub vblank: bool,
    pub hblank: bool,
}

impl InterruptSource {
    pub fn bits(&self) -> u8 {
        u8::from_bits(&[self.ly, self.oam, self.vblank, self.hblank])
    }

    pub fn set_bits(&mut self, value: u8) {
        self.ly = value.bit(3);
        self.oam = value.bit(2);
        self.vblank = value.bit(1);
        self.hblank = value.bit(0);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Mode {
    HBlank = 0,
    VBlank = 1,
    Oam = 2,
    Transfer = 3,
}

impl Default for Mode {
    fn default() -> Self {
        Self::HBlank
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Ppu {
    vram: Vram,
    control: Control,
    interrupt_source: InterruptSource,
    mode: Mode,
    current_y: u8,
    y_compare: u8,
    scroll_position: Coordinate,
    window_position: Coordinate,
    cycles: u64,
}

impl Ppu {
    pub fn tick(&mut self, interrupt_controller: &InterruptController, renderer: impl Renderer) {}

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
        self.control.set_bits(value)
    }

    pub fn status_bits(&self) -> u8 {
        self.interrupt_source.bits() << 3
            | ((self.current_y == self.y_compare) as u8) << 2
            | (self.mode as u8)
    }

    pub fn set_status_bits(&mut self, value: u8) {
        self.interrupt_source.set_bits(value >> 3);
    }

    pub fn scroll_position(&self) -> Coordinate {
        self.scroll_position
    }

    pub fn scroll_position_mut(&mut self) -> &mut Coordinate {
        &mut self.scroll_position
    }

    pub fn current_y(&self) -> u8 {
        self.current_y
    }

    pub fn y_compare(&self) -> u8 {
        self.y_compare
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

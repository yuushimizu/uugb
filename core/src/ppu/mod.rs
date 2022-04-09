pub mod oam;
pub mod vram;

mod control;

use crate::{interrupt::InterruptController, util::bits::Bits};
use control::Control;
use vram::Vram;

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
    ly: u8,
    lyc: u8,
    cycles: u64,
}

impl Ppu {
    pub fn tick(&mut self, interrupt_controller: &InterruptController) {}

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
        self.interrupt_source.bits() << 3 | ((self.ly == self.lyc) as u8) << 2 | (self.mode as u8)
    }

    pub fn set_status_bits(&mut self, value: u8) {
        self.interrupt_source.set_bits(value >> 3);
    }
}

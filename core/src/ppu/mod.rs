mod control;
pub mod oam;
pub mod vram;

use crate::interrupt::InterruptController;
use control::Control;
use vram::Vram;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Ppu {
    vram: Vram,
    control: Control,
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
}

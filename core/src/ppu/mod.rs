pub mod vram;

use vram::Vram;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Ppu {
    vram: Vram,
}

impl Ppu {
    pub fn read_vram(&self, address: u16) -> u8 {
        self.vram.read(address)
    }

    pub fn write_vram(&mut self, address: u16, value: u8) {
        self.vram.write(address, value);
    }
}

mod wram;

use crate::cartridge::Cartridge;
use wram::Wram;

#[derive(Debug)]
pub struct Memory {
    cartridge: Cartridge,
    wram: Wram,
    hram: Vec<u8>,
}

impl Memory {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            wram: Default::default(),
            hram: vec![0x00u8; 0x7F],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.read(address),
            0x8000..=0x9FFF => panic!("VRAM"),
            0xA000..=0xBFFF => self.cartridge.read(address),
            0xC000..=0xDFFF => self.wram.read(address - 0xC000),
            0xE000..=0xFDFF => self.wram.read(address - 0xE000), // mirror
            0xFEA0..=0xFEFF => 0,                                // unusable
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            _ => panic!("Read from the address: {:04X}", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.cartridge.write(address, value),
            0x8000..=0x9FFF => panic!("VRAM"),
            0xA000..=0xBFFF => self.cartridge.write(address, value),
            0xC000..=0xDFFF => self.wram.write(address - 0xC000, value),
            0xE000..=0xFDFF => self.wram.write(address - 0xE000, value), // mirror
            0xFEA0..=0xFEFF => {}                                        // unusable
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
            _ => panic!("Write {:02X} to the address: {:04X}", value, address),
        }
    }
}

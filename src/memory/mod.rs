use crate::cartridge::Cartridge;

#[derive(Debug, Clone)]
pub struct Memory {
    cartridge: Cartridge,
}

impl Memory {
    pub fn new(cartridge: Cartridge) -> Self {
        Self { cartridge }
    }

    pub fn read(self: &Self, address: u16) -> u8 {
        println!("Memory: read from {:#06x}", address);
        0
    }
}

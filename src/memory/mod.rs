use crate::cartridge::Cartridge;

#[derive(Debug, Clone)]
pub struct Memory {
    cartridge: Cartridge,
}

impl Memory {
    pub fn new(cartridge: Cartridge) -> Self {
        Self { cartridge }
    }

    pub fn read(&self, address: u16) -> u8 {
        println!("Memory: read from {:04X}", address);
        0
    }
}

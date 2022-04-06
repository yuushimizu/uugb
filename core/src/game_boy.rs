use crate::{
    cartridge::Cartridge,
    cpu::Cpu,
    memory::{Hram, MappedMemory, Wram},
};

pub struct GameBoy {
    cartridge: Cartridge,
    cpu: Cpu,
    wram: Wram,
    hram: Hram,
}

impl GameBoy {
    pub fn boot(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            cpu: Default::default(),
            wram: Wram::default(),
            hram: Hram::default(),
        }
    }

    pub fn step(&mut self) -> crate::cpu::Instruction {
        self.cpu.step(&mut MappedMemory {
            cartridge: &mut self.cartridge,
            wram: &mut self.wram,
            hram: &mut self.hram,
        })
    }
}

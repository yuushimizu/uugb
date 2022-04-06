use crate::{
    cartridge::Cartridge,
    cpu::Cpu,
    io::Joypad,
    memory::{Hram, MappedMemory, Wram},
};

pub struct GameBoy {
    cartridge: Cartridge,
    cpu: Cpu,
    wram: Wram,
    hram: Hram,
    joypad: Joypad,
}

impl GameBoy {
    pub fn boot(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            cpu: Default::default(),
            wram: Default::default(),
            hram: Default::default(),
            joypad: Default::default(),
        }
    }

    pub fn step(&mut self) -> crate::cpu::Instruction {
        self.cpu.step(&mut MappedMemory {
            cartridge: &mut self.cartridge,
            wram: &mut self.wram,
            hram: &mut self.hram,
            joypad: &mut self.joypad,
        })
    }
}

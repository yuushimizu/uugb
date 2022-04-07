use crate::{
    cartridge::Cartridge,
    cpu::Cpu,
    interrupt::InterruptController,
    io::Joypad,
    memory::{mapped_memory, Hram, MappedMemory, Wram},
};

pub struct GameBoy {
    cartridge: Cartridge,
    cpu: Cpu,
    wram: Wram,
    hram: Hram,
    joypad: Joypad,
    interrupt_controller: InterruptController,
}

impl GameBoy {
    pub fn boot(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            cpu: Default::default(),
            wram: Default::default(),
            hram: Default::default(),
            joypad: Default::default(),
            interrupt_controller: Default::default(),
        }
    }

    pub fn step(&mut self) -> crate::cpu::Instruction {
        self.cpu
            .step(&mut MappedMemory::new(mapped_memory::Components {
                cartridge: &mut self.cartridge,
                wram: &mut self.wram,
                hram: &mut self.hram,
                joypad: &mut self.joypad,
                interrupt_controller: &mut self.interrupt_controller,
            }))
    }
}

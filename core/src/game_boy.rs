use crate::{
    cartridge::Cartridge,
    cpu::Cpu,
    interrupt::InterruptController,
    joypad::Joypad,
    memory::{self, Hram, Memory, Wram},
    ppu::Ppu,
    serial::Serial,
    timer::Timer,
};

#[derive(Debug)]
struct MemoryComponents {
    cartridge: Cartridge,
    wram: Wram,
    ppu: Ppu,
    hram: Hram,
    interrupt_controller: InterruptController,
    joypad: Joypad,
    timer: Timer,
    serial: Serial,
}

impl memory::Components for MemoryComponents {
    fn refs(&self) -> memory::ComponentsRefs {
        memory::ComponentsRefs {
            cartridge: &self.cartridge,
            wram: &self.wram,
            ppu: &self.ppu,
            hram: &self.hram,
            interrupt_controller: &self.interrupt_controller,
            joypad: &self.joypad,
            timer: &self.timer,
            serial: &self.serial,
        }
    }

    fn refs_mut(&mut self) -> memory::ComponentsRefsMut {
        memory::ComponentsRefsMut {
            cartridge: &mut self.cartridge,
            wram: &mut self.wram,
            ppu: &mut self.ppu,
            hram: &mut self.hram,
            interrupt_controller: &mut self.interrupt_controller,
            joypad: &mut self.joypad,
            timer: &mut self.timer,
            serial: &mut self.serial,
        }
    }
}

#[derive(Debug)]
pub struct GameBoy {
    cpu: Cpu,
    memory_components: MemoryComponents,
    serial_connection: DummySerialConnection,
}

impl GameBoy {
    pub fn boot(cartridge: Cartridge) -> Self {
        Self {
            cpu: Default::default(),
            memory_components: MemoryComponents {
                cartridge,
                wram: Default::default(),
                ppu: Default::default(),
                hram: Default::default(),
                interrupt_controller: Default::default(),
                joypad: Default::default(),
                timer: Default::default(),
                serial: Default::default(),
            },
            serial_connection: DummySerialConnection::new(),
        }
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut Memory::new(&mut self.memory_components));
        self.memory_components
            .timer
            .tick(&mut self.memory_components.interrupt_controller);
        self.memory_components.serial.tick(
            &mut self.memory_components.interrupt_controller,
            &mut self.serial_connection,
        );
    }
}

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct DummySerialConnection {
    file: File,
    bits: Vec<bool>,
}

impl DummySerialConnection {
    fn new() -> Self {
        Self {
            file: File::create("./log/dummy-serial").unwrap(),
            bits: vec![],
        }
    }
}

impl crate::serial::SerialConnection for DummySerialConnection {
    fn send(&mut self, bit: bool) {
        use crate::util::bits::Bits;
        self.bits.push(bit);
        if self.bits.len() >= 8 {
            match self.file.write(&[u8::from_bits(&self.bits)]) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Serial output error: {:?}", err);
                }
            }
            self.bits.clear();
        }
    }

    fn receive(&self) -> bool {
        true
    }
}

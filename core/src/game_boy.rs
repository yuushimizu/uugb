use crate::{
    cartridge::Cartridge,
    cpu::Cpu,
    interrupt::InterruptController,
    io::Joypad,
    memory::{mapped_memory, Hram, MappedMemory, Wram},
    serial::Serial,
    timer::Timer,
};

pub struct GameBoy {
    cartridge: Cartridge,
    cpu: Cpu,
    wram: Wram,
    hram: Hram,
    interrupt_controller: InterruptController,
    joypad: Joypad,
    timer: Timer,
    serial: Serial,
    serial_connection: DummySerialConnection,
}

impl GameBoy {
    pub fn boot(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            cpu: Default::default(),
            wram: Default::default(),
            hram: Default::default(),
            interrupt_controller: Default::default(),
            joypad: Default::default(),
            timer: Default::default(),
            serial: Default::default(),
            serial_connection: DummySerialConnection::new(),
        }
    }

    pub fn tick(&mut self) {
        self.cpu
            .tick(&mut MappedMemory::new(mapped_memory::Components {
                cartridge: &mut self.cartridge,
                wram: &mut self.wram,
                hram: &mut self.hram,
                interrupt_controller: &mut self.interrupt_controller,
                joypad: &mut self.joypad,
                timer: &mut self.timer,
                serial: &mut self.serial,
            }));
        self.timer.tick(&mut self.interrupt_controller);
        self.serial
            .tick(&mut self.interrupt_controller, &mut self.serial_connection);
    }
}

use std::fs::File;
use std::io::prelude::*;

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

impl crate::serial::Connection for DummySerialConnection {
    fn send(&mut self, bit: bool) {
        self.bits.push(bit);
        if self.bits.len() >= 8 {
            match self.file.write(&[self
                .bits
                .iter()
                .fold(0x00u8, |acc, &bit| acc << 1 | (bit as u8))])
            {
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

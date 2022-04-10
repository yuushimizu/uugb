use crate::{
    cartridge::Cartridge,
    cpu::{self, Cpu},
    interrupt::InterruptController,
    joypad::Joypad,
    memory::{self, Hram, Wram},
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

impl memory::Context for MemoryComponents {
    fn components(&self) -> memory::ComponentsRefs {
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

    fn components_mut(&mut self) -> memory::ComponentsRefsMut {
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

impl cpu::Context for MemoryComponents {
    fn interrupt_controller(&self) -> &InterruptController {
        &self.interrupt_controller
    }

    fn interrupt_controller_mut(&mut self) -> &mut InterruptController {
        &mut self.interrupt_controller
    }
}

#[derive(Debug)]
pub struct GameBoy {
    cpu: Cpu,
    memory_components: MemoryComponents,
    dummy_renderer: DummyRenderer,
    dummy_serial_connection: DummySerialConnection,
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
            dummy_renderer: DummyRenderer {},
            dummy_serial_connection: Default::default(),
        }
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.memory_components);
        self.memory_components.ppu.tick(
            &mut self.memory_components.interrupt_controller,
            &mut self.dummy_renderer,
        );
        self.memory_components
            .timer
            .tick(&mut self.memory_components.interrupt_controller);
        self.memory_components.serial.tick(
            &mut self.memory_components.interrupt_controller,
            &mut self.dummy_serial_connection,
        );
    }
}

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct DummyRenderer {}

impl crate::ppu::Renderer for DummyRenderer {
    fn render(&mut self, position: crate::ppu::Coordinate, color: u8) {}
}

#[derive(Debug)]
struct DummySerialConnection {
    file: File,
    bits: Vec<bool>,
}

impl Default for DummySerialConnection {
    fn default() -> Self {
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

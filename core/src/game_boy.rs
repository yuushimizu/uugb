use crate::{
    cartridge::Cartridge,
    cpu::{self, Cpu},
    interrupt::InterruptController,
    joypad::Joypad,
    memory::{self, Hram, Memory, Wram},
    ppu::{Ppu, Renderer},
    serial::{Serial, SerialConnection},
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
        }
    }

    pub fn tick(
        &mut self,
        renderer: &mut impl Renderer,
        serial_connection: &mut impl SerialConnection,
    ) {
        self.cpu.tick(&mut self.memory_components);
        self.memory_components
            .ppu
            .tick(&mut self.memory_components.interrupt_controller, renderer);
        self.memory_components
            .timer
            .tick(&mut self.memory_components.interrupt_controller);
        self.memory_components.serial.tick(
            &mut self.memory_components.interrupt_controller,
            serial_connection,
        );
    }

    pub fn dump(&mut self) -> Vec<u8> {
        let memory = Memory::new(&mut self.memory_components);
        let mut buffer = vec![];
        for address in 0x0000..=0xFFFF {
            buffer.push(memory.read(address));
        }
        buffer
    }
}

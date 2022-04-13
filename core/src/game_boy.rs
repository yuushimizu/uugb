use crate::{
    cartridge::Cartridge,
    cpu::{self, Cpu},
    interrupt::InterruptController,
    joypad::Joypad,
    memory::{self, Dma, Hram, Memory, Wram},
    ppu::{Ppu, Renderer},
    serial::{Serial, SerialConnection},
    timer::{Divider, Timer},
};

#[derive(Debug)]
struct MemoryComponents {
    cartridge: Cartridge,
    wram: Wram,
    ppu: Ppu,
    hram: Hram,
    interrupt_controller: InterruptController,
    joypad: Joypad,
    divider: Divider,
    timer: Timer,
    serial: Serial,
    dma: Dma,
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
            divider: &self.divider,
            timer: &self.timer,
            serial: &self.serial,
            dma: &self.dma,
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
            divider: &mut self.divider,
            timer: &mut self.timer,
            serial: &mut self.serial,
            dma: &mut self.dma,
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
                divider: Default::default(),
                timer: Default::default(),
                serial: Default::default(),
                dma: Default::default(),
            },
        }
    }

    pub fn tick(
        &mut self,
        renderer: &mut impl Renderer,
        serial_connection: &mut impl SerialConnection,
    ) {
        for _ in 0..4 {
            self.memory_components.divider.tick();
            self.memory_components
                .ppu
                .tick(&mut self.memory_components.interrupt_controller, renderer);
        }
        self.cpu.tick(&mut self.memory_components);
        self.memory_components.timer.tick(
            &self.memory_components.divider,
            &mut self.memory_components.interrupt_controller,
        );
        self.memory_components.serial.tick(
            &mut self.memory_components.interrupt_controller,
            serial_connection,
        );
        Memory::new(&mut self.memory_components).tick();
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

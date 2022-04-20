use crate::{
    cartridge::Cartridge,
    cpu::Cpu,
    interrupt::InterruptController,
    joypad::{ButtonState, Joypad},
    memory::{self, Dma, Hram, Memory, Wram},
    ppu::{Ppu, Renderer},
    serial::{Serial, SerialConnection},
    timer::{Divider, Timer},
};

#[derive(Debug)]
pub struct GameBoy {
    cartridge: Cartridge,
    cpu: Cpu,
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

impl GameBoy {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            cpu: Default::default(),
            wram: Default::default(),
            ppu: Default::default(),
            hram: Default::default(),
            interrupt_controller: Default::default(),
            joypad: Default::default(),
            divider: Default::default(),
            timer: Default::default(),
            serial: Default::default(),
            dma: Default::default(),
        }
    }

    fn separate_components(&mut self) -> (&mut Cpu, Memory) {
        (
            &mut self.cpu,
            Memory::new(memory::Components {
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
            }),
        )
    }

    pub fn tick(
        &mut self,
        renderer: &mut impl Renderer,
        serial_connection: &mut impl SerialConnection,
    ) {
        for _ in 0..4 {
            self.divider.tick();
            self.ppu.tick(&mut self.interrupt_controller, renderer);
        }
        let (cpu, mut memory) = self.separate_components();
        cpu.tick(&mut memory);
        self.timer
            .tick(&self.divider, &mut self.interrupt_controller);
        self.serial
            .tick(&mut self.interrupt_controller, serial_connection);
        let (_, mut memory) = self.separate_components();
        memory.tick();
    }

    pub fn set_button_state(&mut self, button_state: ButtonState) {
        self.joypad
            .set_button_state(button_state, &mut self.interrupt_controller);
    }

    pub fn dump(&mut self) -> Vec<u8> {
        let (_, memory) = self.separate_components();
        let mut buffer = vec![];
        for address in 0x0000..=0xFFFF {
            buffer.push(memory.read(address));
        }
        buffer
    }
}

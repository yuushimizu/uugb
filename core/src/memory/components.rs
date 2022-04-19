use crate::{
    cartridge::Cartridge,
    interrupt::InterruptController,
    joypad::Joypad,
    memory::{Dma, Hram, Wram},
    ppu::Ppu,
    serial::Serial,
    timer::{Divider, Timer},
};

#[derive(Debug)]
pub struct Components<'a> {
    pub cartridge: &'a mut Cartridge,
    pub wram: &'a mut Wram,
    pub ppu: &'a mut Ppu,
    pub hram: &'a mut Hram,
    pub interrupt_controller: &'a mut InterruptController,
    pub joypad: &'a mut Joypad,
    pub divider: &'a mut Divider,
    pub timer: &'a mut Timer,
    pub serial: &'a mut Serial,
    pub dma: &'a mut Dma,
}

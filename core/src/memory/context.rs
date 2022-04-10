use crate::{
    cartridge::Cartridge,
    interrupt::InterruptController,
    joypad::Joypad,
    memory::{Hram, Wram},
    ppu::Ppu,
    serial::Serial,
    timer::Timer,
};

#[derive(Debug)]
pub struct ComponentsRefs<'a> {
    pub cartridge: &'a Cartridge,
    pub wram: &'a Wram,
    pub ppu: &'a Ppu,
    pub hram: &'a Hram,
    pub interrupt_controller: &'a InterruptController,
    pub joypad: &'a Joypad,
    pub timer: &'a Timer,
    pub serial: &'a Serial,
}

#[derive(Debug)]
pub struct ComponentsRefsMut<'a> {
    pub cartridge: &'a mut Cartridge,
    pub wram: &'a mut Wram,
    pub ppu: &'a mut Ppu,
    pub hram: &'a mut Hram,
    pub interrupt_controller: &'a mut InterruptController,
    pub joypad: &'a mut Joypad,
    pub timer: &'a mut Timer,
    pub serial: &'a mut Serial,
}

pub trait Context {
    fn components(&self) -> ComponentsRefs;

    fn components_mut(&mut self) -> ComponentsRefsMut;
}

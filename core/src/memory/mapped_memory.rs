use super::{Hram, Memory, Wram};
use crate::{cartridge::Cartridge, io::Joypad};

#[derive(Debug)]
pub struct Components<'a> {
    pub cartridge: &'a mut Cartridge,
    pub wram: &'a mut Wram,
    pub hram: &'a mut Hram,
    pub joypad: &'a mut Joypad,
}

#[derive(Debug)]
pub struct MappedMemory<'a>(Components<'a>);

struct Segment {
    read: fn(&Components, u16) -> u8,
    write: fn(&mut Components, u16, u8),
}

impl Segment {
    fn cartridge() -> Self {
        Self {
            read: |component, address| component.cartridge.read(address),
            write: |component, address, value| component.cartridge.write(address, value),
        }
    }

    fn wram() -> Self {
        const BASE_ADDRESS: u16 = 0xC000;
        Self {
            read: |component, address| component.wram.read(address - BASE_ADDRESS),
            write: |component, address, value| component.wram.write(address - BASE_ADDRESS, value),
        }
    }

    fn wram_mirror() -> Self {
        const BASE_ADDRESS: u16 = 0xE000;
        Self {
            read: |component, address| component.wram.read(address - BASE_ADDRESS),
            write: |component, address, value| component.wram.write(address - BASE_ADDRESS, value),
        }
    }

    fn unusable() -> Self {
        Self {
            read: |_, _| 0xFF,
            write: |_, _, _| {},
        }
    }

    fn hram() -> Self {
        const BASE_ADDRESS: u16 = 0xFF80;
        Self {
            read: |component, address| component.hram.read(address - BASE_ADDRESS),
            write: |component, address, value| {
                component.hram.write(address - BASE_ADDRESS, value);
            },
        }
    }

    fn joypad() -> Self {
        Self {
            read: |component, _address| component.joypad.bits(),
            write: |component, _address, value| component.joypad.set_bits(value),
        }
    }
}

impl<'a> MappedMemory<'a> {
    pub fn new(components: Components<'a>) -> Self {
        Self(components)
    }

    fn segment(&self, address: u16) -> Segment {
        match address {
            0x0000..=0x7FFF => Segment::cartridge(),
            0x8000..=0x9FFF => panic!("VRAM"),
            0xA000..=0xBFFF => Segment::cartridge(),
            0xC000..=0xDFFF => Segment::wram(),
            0xE000..=0xFDFF => Segment::wram_mirror(),
            0xFEA0..=0xFEFF => Segment::unusable(),
            0xFF00 => Segment::joypad(),
            0xFF80..=0xFFFE => Segment::hram(),
            _ => panic!("Read from the address: {:04X}", address),
        }
    }
}

impl<'a> Memory for MappedMemory<'a> {
    fn read(&self, address: u16) -> u8 {
        (self.segment(address).read)(&self.0, address)
    }

    fn write(&mut self, address: u16, value: u8) {
        (self.segment(address).write)(&mut self.0, address, value)
    }
}

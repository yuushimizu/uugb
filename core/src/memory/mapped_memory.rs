use super::{Hram, Memory, Wram};
use crate::cartridge::Cartridge;

#[derive(Debug)]
pub struct MappedMemory<'a> {
    pub cartridge: &'a mut Cartridge,
    pub wram: &'a mut Wram,
    pub hram: &'a mut Hram,
}

struct Segment {
    read: Box<dyn Fn(&MappedMemory, u16) -> u8>,
    write: Box<dyn FnMut(&mut MappedMemory, u16, u8)>,
}

impl<'a> MappedMemory<'a> {
    fn cartridge_segment(&self) -> Segment {
        Segment {
            read: Box::new(|memory, address| memory.cartridge.read(address)),
            write: Box::new(|memory, address, value| memory.cartridge.write(address, value)),
        }
    }

    fn wram_segment(&self, base_address: u16) -> Segment {
        Segment {
            read: Box::new(move |memory, address| memory.wram.read(address - base_address)),
            write: Box::new(move |memory, address, value| {
                memory.wram.write(address - base_address, value)
            }),
        }
    }

    fn unusable_segment(&self) -> Segment {
        Segment {
            read: Box::new(|_, _| 0xFF),
            write: Box::new(|_, _, _| {}),
        }
    }

    fn hram_segment(&self) -> Segment {
        Segment {
            read: Box::new(|memory, address| memory.hram.read(address - 0xFF80)),
            write: Box::new(|memory, address, value| {
                memory.hram.write(address - 0xFF80, value);
            }),
        }
    }

    fn segment(&self, address: u16) -> Segment {
        match address {
            0x0000..=0x7FFF => self.cartridge_segment(),
            0x8000..=0x9FFF => panic!("VRAM"),
            0xA000..=0xBFFF => self.cartridge_segment(),
            0xC000..=0xDFFF => self.wram_segment(0xC000),
            0xE000..=0xFDFF => self.wram_segment(0xE000), // mirror
            0xFEA0..=0xFEFF => self.unusable_segment(),
            0xFF80..=0xFFFE => self.hram_segment(),
            _ => panic!("Read from the address: {:04X}", address),
        }
    }
}

impl<'a> Memory for MappedMemory<'a> {
    fn read(&self, address: u16) -> u8 {
        (self.segment(address).read)(self, address)
    }

    fn write(&mut self, address: u16, value: u8) {
        (self.segment(address).write)(self, address, value)
    }
}

use super::{Hram, Memory, Wram};
use crate::{cartridge::Cartridge, io::Joypad};

#[derive(Debug)]
pub struct MappedMemory<'a> {
    pub cartridge: &'a mut Cartridge,
    pub wram: &'a mut Wram,
    pub hram: &'a mut Hram,
    pub joypad: &'a mut Joypad,
}

struct Segment {
    read: Box<dyn Fn(&MappedMemory, u16) -> u8>,
    write: Box<dyn FnMut(&mut MappedMemory, u16, u8)>,
}

impl Segment {
    fn new<
        R: Fn(&MappedMemory, u16) -> u8 + 'static,
        W: FnMut(&mut MappedMemory, u16, u8) + 'static,
    >(
        read: R,
        write: W,
    ) -> Self {
        Self {
            read: Box::new(read),
            write: Box::new(write),
        }
    }
}

impl<'a> MappedMemory<'a> {
    fn cartridge_segment(&self) -> Segment {
        Segment::new(
            |this, address| this.cartridge.read(address),
            |this, address, value| this.cartridge.write(address, value),
        )
    }

    fn wram_segment(&self, base_address: u16) -> Segment {
        Segment::new(
            move |this, address| this.wram.read(address - base_address),
            move |this, address, value| this.wram.write(address - base_address, value),
        )
    }

    fn unusable_segment(&self) -> Segment {
        Segment::new(|_, _| 0xFF, |_, _, _| {})
    }

    fn hram_segment(&self) -> Segment {
        Segment::new(
            |this, address| this.hram.read(address - 0xFF80),
            |this, address, value| {
                this.hram.write(address - 0xFF80, value);
            },
        )
    }

    fn joypad_segment(&self) -> Segment {
        Segment::new(
            |this, _address| this.joypad.bits(),
            |this, _address, value| this.joypad.set_bits(value),
        )
    }

    fn segment(&self, address: u16) -> Segment {
        match address {
            0x0000..=0x7FFF => self.cartridge_segment(),
            0x8000..=0x9FFF => panic!("VRAM"),
            0xA000..=0xBFFF => self.cartridge_segment(),
            0xC000..=0xDFFF => self.wram_segment(0xC000),
            0xE000..=0xFDFF => self.wram_segment(0xE000), // mirror
            0xFEA0..=0xFEFF => self.unusable_segment(),
            0xFF00 => self.joypad_segment(),
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

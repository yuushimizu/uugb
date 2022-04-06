mod wram;

use crate::cartridge::Cartridge;
use wram::Wram;

#[derive(Debug)]
pub struct Memory {
    cartridge: Cartridge,
    wram: Wram,
    hram: Vec<u8>,
}

struct Segment {
    read: Box<dyn Fn(&Memory, u16) -> u8>,
    write: Box<dyn FnMut(&mut Memory, u16, u8)>,
}

impl Memory {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            wram: Default::default(),
            hram: vec![0x00u8; 0x7F],
        }
    }

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

    fn segment(&self, address: u16) -> Segment {
        match address {
            0x0000..=0x7FFF => self.cartridge_segment(),
            0x8000..=0x9FFF => panic!("VRAM"),
            0xA000..=0xBFFF => self.cartridge_segment(),
            0xC000..=0xDFFF => self.wram_segment(0xC000),
            0xE000..=0xFDFF => self.wram_segment(0xE000), // mirror
            0xFEA0..=0xFEFF => Segment {
                read: Box::new(|_, _| 0xFF),
                write: Box::new(|_, _, _| {}),
            }, // unusable
            0xFF80..=0xFFFE => Segment {
                read: Box::new(|memory, address| memory.hram[(address - 0xFF80) as usize]),
                write: Box::new(|memory, address, value| {
                    memory.hram[(address - 0xFF80) as usize] = value
                }),
            },
            _ => panic!("Read from the address: {:04X}", address),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        (self.segment(address).read)(self, address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        (self.segment(address).write)(self, address, value)
    }
}

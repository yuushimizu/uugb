use super::{Hram, Memory, Wram};
use crate::{cartridge::Cartridge, interrupt::InterruptController, io::Joypad};

#[derive(Debug)]
pub struct Components<'a> {
    pub cartridge: &'a mut Cartridge,
    pub wram: &'a mut Wram,
    pub hram: &'a mut Hram,
    pub joypad: &'a mut Joypad,
    pub interrupt_controller: &'a mut InterruptController,
}

mod segment {
    use super::Components;

    pub struct Segment {
        pub read: fn(&Components, u16) -> u8,
        pub write: fn(&mut Components, u16, u8),
    }

    pub const CARTRIDGE: Segment = Segment {
        read: |component, address| component.cartridge.read(address),
        write: |component, address, value| component.cartridge.write(address, value),
    };

    pub const WRAM: Segment = {
        const BASE_ADDRESS: u16 = 0xC000;
        Segment {
            read: |component, address| component.wram.read(address - BASE_ADDRESS),
            write: |component, address, value| component.wram.write(address - BASE_ADDRESS, value),
        }
    };

    pub const WRAM_MIRROR: Segment = {
        const BASE_ADDRESS: u16 = 0xE000;
        Segment {
            read: |component, address| component.wram.read(address - BASE_ADDRESS),
            write: |component, address, value| component.wram.write(address - BASE_ADDRESS, value),
        }
    };

    pub const UNUSABLE: Segment = Segment {
        read: |_, _| 0xFF,
        write: |_, _, _| {},
    };

    pub const HRAM: Segment = {
        const BASE_ADDRESS: u16 = 0xFF80;
        Segment {
            read: |component, address| component.hram.read(address - BASE_ADDRESS),
            write: |component, address, value| {
                component.hram.write(address - BASE_ADDRESS, value);
            },
        }
    };

    pub const JOYPAD: Segment = Segment {
        read: |component, _address| component.joypad.bits(),
        write: |component, _address, value| component.joypad.set_bits(value),
    };

    pub const INTERRUPT_REQUESTED: Segment = Segment {
        read: |component, _address| component.interrupt_controller.requested_bits(),
        write: |component, _address, value| {
            component.interrupt_controller.set_requested_bits(value)
        },
    };

    pub const INTERRUPT_ENABLED: Segment = Segment {
        read: |component, _address| component.interrupt_controller.enabled_bits(),
        write: |component, _address, value| component.interrupt_controller.set_enabled_bits(value),
    };
}

use segment::Segment;

#[derive(Debug)]
pub struct MappedMemory<'a>(Components<'a>);

impl<'a> MappedMemory<'a> {
    pub fn new(components: Components<'a>) -> Self {
        Self(components)
    }

    fn segment(&self, address: u16) -> Segment {
        use segment::*;
        match address {
            0x0000..=0x7FFF => CARTRIDGE,
            0x8000..=0x9FFF => panic!("VRAM"),
            0xA000..=0xBFFF => CARTRIDGE,
            0xC000..=0xDFFF => WRAM,
            0xE000..=0xFDFF => WRAM_MIRROR,
            0xFE00..=0xFE9F => panic!("OAM"),
            0xFEA0..=0xFEFF => UNUSABLE,
            0xFF00 => JOYPAD,
            0xFF0F => INTERRUPT_REQUESTED,
            0xFF80..=0xFFFE => HRAM,
            0xFFFF => INTERRUPT_ENABLED,
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

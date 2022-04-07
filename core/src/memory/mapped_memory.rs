use super::{Hram, Memory, Wram};
use crate::{cartridge::Cartridge, interrupt::InterruptController, io::Joypad, timer::Timer};

#[derive(Debug)]
pub struct Components<'a> {
    pub cartridge: &'a mut Cartridge,
    pub wram: &'a mut Wram,
    pub hram: &'a mut Hram,
    pub joypad: &'a mut Joypad,
    pub timer: &'a mut Timer,
    pub interrupt_controller: &'a mut InterruptController,
}

mod segment {
    use super::Components;

    pub struct Segment {
        pub read: fn(&Components, u16) -> u8,
        pub write: fn(&mut Components, u16, u8),
    }

    impl Segment {
        pub fn read(&self, components: &Components, address: u16) -> u8 {
            (self.read)(components, address)
        }

        pub fn write(&self, components: &mut Components, address: u16, value: u8) {
            (self.write)(components, address, value);
        }
    }

    pub const CARTRIDGE: Segment = Segment {
        read: |components, address| components.cartridge.read(address),
        write: |components, address, value| components.cartridge.write(address, value),
    };

    pub const WRAM: Segment = {
        const BASE_ADDRESS: u16 = 0xC000;
        Segment {
            read: |components, address| components.wram.read(address - BASE_ADDRESS),
            write: |components, address, value| {
                components.wram.write(address - BASE_ADDRESS, value)
            },
        }
    };

    pub const WRAM_MIRROR: Segment = {
        const BASE_ADDRESS: u16 = 0xE000;
        Segment {
            read: |components, address| components.wram.read(address - BASE_ADDRESS),
            write: |components, address, value| {
                components.wram.write(address - BASE_ADDRESS, value)
            },
        }
    };

    pub const UNUSABLE: Segment = Segment {
        read: |_, _| 0xFF,
        write: |_, _, _| {},
    };

    pub const HRAM: Segment = {
        const BASE_ADDRESS: u16 = 0xFF80;
        Segment {
            read: |components, address| components.hram.read(address - BASE_ADDRESS),
            write: |components, address, value| {
                components.hram.write(address - BASE_ADDRESS, value);
            },
        }
    };

    pub const JOYPAD: Segment = Segment {
        read: |components, _| components.joypad.bits(),
        write: |components, _, value| {
            components
                .joypad
                .set_bits(value, components.interrupt_controller)
        },
    };

    pub const TIMER: Segment = {
        fn inner_segment(address: u16) -> Segment {
            match address {
                0xFF04 => Segment {
                    read: |components, _| components.timer.divider_register(),
                    write: |components, _, _| components.timer.reset_divider(),
                },
                0xFF05 => Segment {
                    read: |components, _| components.timer.counter(),
                    write: |components, _, value| components.timer.set_counter(value),
                },
                0xFF06 => Segment {
                    read: |components, _| components.timer.modulo(),
                    write: |components, _, value| components.timer.set_modulo(value),
                },
                0xFF07 => Segment {
                    read: |components, _| components.timer.control_bits(),
                    write: |components, _, value| components.timer.set_control_bits(value),
                },
                _ => unreachable!(),
            }
        }
        Segment {
            read: |components, address| inner_segment(address).read(components, address),
            write: |components, address, value| {
                inner_segment(address).write(components, address, value)
            },
        }
    };

    pub const INTERRUPT_REQUESTED: Segment = Segment {
        read: |components, _address| components.interrupt_controller.requested_bits(),
        write: |components, _address, value| {
            components.interrupt_controller.set_requested_bits(value)
        },
    };

    pub const INTERRUPT_ENABLED: Segment = Segment {
        read: |components, _address| components.interrupt_controller.enabled_bits(),
        write: |components, _address, value| {
            components.interrupt_controller.set_enabled_bits(value)
        },
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
            0xFF04..=0xFF07 => TIMER,
            0xFF0F => INTERRUPT_REQUESTED,
            0xFF80..=0xFFFE => HRAM,
            0xFFFF => INTERRUPT_ENABLED,
            _ => panic!("Read from the address: {:04X}", address),
        }
    }
}

impl<'a> Memory for MappedMemory<'a> {
    fn read(&self, address: u16) -> u8 {
        self.segment(address).read(&self.0, address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.segment(address).write(&mut self.0, address, value)
    }
}

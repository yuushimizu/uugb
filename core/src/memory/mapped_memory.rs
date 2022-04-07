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
        pub reader: Box<dyn Fn(&Components, u16) -> u8>,
        pub writer: Box<dyn Fn(&mut Components, u16, u8)>,
    }

    impl Segment {
        pub fn new<
            R: Fn(&Components, u16) -> u8 + 'static,
            W: Fn(&mut Components, u16, u8) + 'static,
        >(
            reader: R,
            writer: W,
        ) -> Self {
            Self {
                reader: Box::new(reader),
                writer: Box::new(writer),
            }
        }

        pub fn read(&self, components: &Components, address: u16) -> u8 {
            (self.reader)(components, address)
        }

        pub fn write(&self, components: &mut Components, address: u16, value: u8) {
            (self.writer)(components, address, value);
        }
    }

    pub fn cartridge() -> Segment {
        Segment::new(
            |components, address| components.cartridge.read(address),
            |components, address, value| components.cartridge.write(address, value),
        )
    }

    pub fn wram(base_address: u16) -> Segment {
        Segment::new(
            move |components, address| components.wram.read(address - base_address),
            move |components, address, value| components.wram.write(address - base_address, value),
        )
    }

    pub fn unusable() -> Segment {
        Segment::new(|_, _| 0xFF, |_, _, _| {})
    }

    pub fn hram(base_address: u16) -> Segment {
        Segment::new(
            move |components, address| components.hram.read(address - base_address),
            move |components, address, value| {
                components.hram.write(address - base_address, value);
            },
        )
    }

    pub fn joypad() -> Segment {
        Segment::new(
            |components, _| components.joypad.bits(),
            |components, _, value| {
                components
                    .joypad
                    .set_bits(value, components.interrupt_controller)
            },
        )
    }

    pub fn timer() -> Segment {
        fn inner(address: u16) -> Segment {
            match address {
                0xFF04 => Segment::new(
                    |components, _| components.timer.divider_register(),
                    |components, _, _| components.timer.reset_divider(),
                ),
                0xFF05 => Segment::new(
                    |components, _| components.timer.counter(),
                    |components, _, value| components.timer.set_counter(value),
                ),
                0xFF06 => Segment::new(
                    |components, _| components.timer.modulo(),
                    |components, _, value| components.timer.set_modulo(value),
                ),
                0xFF07 => Segment::new(
                    |components, _| components.timer.control_bits(),
                    |components, _, value| components.timer.set_control_bits(value),
                ),
                _ => unreachable!(),
            }
        }
        Segment::new(
            |components, address| inner(address).read(components, address),
            |components, address, value| inner(address).write(components, address, value),
        )
    }

    pub fn interrupt_requested() -> Segment {
        Segment::new(
            |components, _address| components.interrupt_controller.requested_bits(),
            |components, _address, value| components.interrupt_controller.set_requested_bits(value),
        )
    }

    pub fn interrupt_enabled() -> Segment {
        Segment::new(
            |components, _address| components.interrupt_controller.enabled_bits(),
            |components, _address, value| components.interrupt_controller.set_enabled_bits(value),
        )
    }
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
            0x0000..=0x7FFF => cartridge(),
            0x8000..=0x9FFF => panic!("VRAM"),
            0xA000..=0xBFFF => cartridge(),
            0xC000..=0xDFFF => wram(0xC000),
            0xE000..=0xFDFF => wram(0xE000), // mirror
            0xFE00..=0xFE9F => panic!("OAM"),
            0xFEA0..=0xFEFF => unusable(),
            0xFF00 => joypad(),
            0xFF04..=0xFF07 => timer(),
            0xFF0F => interrupt_requested(),
            0xFF80..=0xFFFE => hram(0xFF80),
            0xFFFF => interrupt_enabled(),
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

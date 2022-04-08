use super::{Hram, Memory, Wram};
use crate::{cartridge::Cartridge, interrupt::InterruptController, io::Joypad, timer::Timer};
use log;

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

    pub fn wram() -> Segment {
        Segment::new(
            |components, address| match address {
                0xC000..=0xCFFF => components.wram.read(address - 0xC000),
                0xD000..=0xDFFF => components.wram.read(address - 0xD000),
                _ => unreachable!(),
            },
            |components, address, value| match address {
                0xC000..=0xCFFF => components.wram.write(address - 0xC000, value),
                0xD000..=0xDFFF => components.wram.write(address - 0xD000, value),
                _ => unreachable!(),
            },
        )
    }

    pub fn unusable() -> Segment {
        Segment::new(|_, _| 0xFF, |_, _, _| {})
    }

    pub fn hram() -> Segment {
        const BASE_ADDRESS: u16 = 0xFF80;
        Segment::new(
            move |components, address| components.hram.read(address - BASE_ADDRESS),
            move |components, address, value| {
                components.hram.write(address - BASE_ADDRESS, value);
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

    pub fn serial() -> Segment {
        Segment::new(
            |components, address| {
                log::info!("Serial Port is not implemented yet u_u");
                0
            },
            |components, address, value| {
                log::info!("serial Port is not implemented yet x_x");
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

    pub fn apu() -> Segment {
        Segment::new(
            |components, address| {
                log::info!("APU is not implemented yet >_<");
                0
            },
            |components, address, value| {
                log::info!("APU is not implemented yet x_x");
            },
        )
    }

    pub fn ppu() -> Segment {
        Segment::new(
            |components, address| {
                log::info!("APU is not implemented yet >_<");
                0
            },
            |components, address, value| {
                log::info!("APU is not implemented yet x_x");
            },
        )
    }

    pub fn ir() -> Segment {
        Segment::new(
            |components, address| {
                log::info!("IR is not implemented yet >_<");
                0
            },
            |components, address, value| {
                log::info!("IR is not implemented yet x_x");
            },
        )
    }

    pub fn interrupt_enabled() -> Segment {
        Segment::new(
            |components, _address| components.interrupt_controller.enabled_bits(),
            |components, _address, value| components.interrupt_controller.set_enabled_bits(value),
        )
    }

    pub fn unknown() -> Segment {
        Segment::new(
            |components, address| {
                log::warn!("Attempt to read from the unknown segment: {:04X}", address);
                0xFF
            },
            |components, address, value| {
                log::warn!("Attempt to write to the unknown segment: {:04X}", address);
            },
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
            0xC000..=0xFDFF => wram(),
            0xFE00..=0xFE9F => panic!("OAM"),
            0xFEA0..=0xFEFF => unusable(),
            0xFF00 => joypad(),
            0xFF01..=0xFF02 => serial(),
            0xFF03 => unknown(),
            0xFF04..=0xFF07 => timer(),
            0xFF08..=0xFF0E => unknown(),
            0xFF0F => interrupt_requested(),
            0xFF10..=0xFF3F => apu(),
            0xFF40..=0xFF4F => ppu(),
            0xFF50 => unknown(),
            0xFF51..=0xFF55 => ppu(),
            0xFF56 => ir(),
            0xFF57..=0xFF67 => unknown(),
            0xFF68..=0xFF6C => ppu(),
            0xFF6D..=0xFF6F => unknown(),
            0xFF70 => wram(),
            0xFF71 => unknown(),
            0xFF72..=0xFF75 => unknown(), // undocumented registers?
            0xFF76..=0xFF77 => apu(),
            0xFF78..=0xFF7F => unknown(),
            0xFF80..=0xFFFE => hram(),
            0xFFFF => interrupt_enabled(),
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

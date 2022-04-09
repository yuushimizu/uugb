use super::{Hram, Memory, Wram};
use crate::{
    cartridge::Cartridge, interrupt::InterruptController, joypad::Joypad, ppu::Ppu, serial::Serial,
    timer::Timer,
};

#[derive(Debug)]
pub struct Components<'a> {
    pub cartridge: &'a mut Cartridge,
    pub wram: &'a mut Wram,
    pub ppu: &'a mut Ppu,
    pub hram: &'a mut Hram,
    pub interrupt_controller: &'a mut InterruptController,
    pub joypad: &'a mut Joypad,
    pub timer: &'a mut Timer,
    pub serial: &'a mut Serial,
}

mod segment {
    use super::Components;

    pub enum Segment<'a> {
        Leaf(fn(&Components, u16) -> u8, fn(&mut Components, u16, u8)),
        Nested(fn(address: u16) -> &'a Segment<'a>),
        Offset(u16, &'a Segment<'a>),
    }

    impl<'a> Segment<'a> {
        pub fn read(&self, components: &Components, address: u16) -> u8 {
            use Segment::*;
            match self {
                Leaf(reader, _) => reader(components, address),
                Nested(inner) => inner(address).read(components, address),
                Offset(offset, inner) => inner.read(components, address - offset),
            }
        }

        pub fn write(&self, components: &mut Components, address: u16, value: u8) {
            use Segment::*;
            match self {
                Leaf(_, writer) => writer(components, address, value),
                Nested(inner) => inner(address).write(components, address, value),
                Offset(offset, inner) => inner.write(components, address - offset, value),
            }
        }
    }

    pub const UNKNOWN: Segment = Segment::Leaf(
        |_, address| {
            log::warn!("Attempt to read from the unknown segment: {:04X}", address);
            0xFF
        },
        |_, address, _| {
            log::warn!("Attempt to write to the unknown segment: {:04X}", address);
        },
    );

    pub const CARTRIDGE: Segment = Segment::Leaf(
        |components, address| components.cartridge.read(address),
        |components, address, value| components.cartridge.write(address, value),
    );

    pub const WRAM: Segment = {
        Segment::Nested(|address| match address {
            0xC000..=0xCFFF => &Segment::Offset(
                0xC000,
                &Segment::Leaf(
                    |components, address| components.wram.read(address),
                    |components, address, value| components.wram.write(address, value),
                ),
            ),
            0xD000..=0xDFFF => &Segment::Offset(
                0xD000,
                &Segment::Leaf(
                    |components, address| components.wram.read_bank(address),
                    |components, address, value| components.wram.write_bank(address, value),
                ),
            ),
            0xFF70 => &Segment::Leaf(
                |components, _| components.wram.bank_switch(),
                |components, _, value| components.wram.set_bank_switch(value),
            ),
            _ => unreachable!(),
        })
    };

    pub const UNUSABLE: Segment = Segment::Leaf(|_, _| 0xFF, |_, _, _| {});

    pub const HRAM: Segment = Segment::Offset(
        0xFF80,
        &Segment::Leaf(
            |components, address| components.hram.read(address),
            |components, address, value| {
                components.hram.write(address, value);
            },
        ),
    );

    pub const JOYPAD: Segment = Segment::Leaf(
        |components, _| components.joypad.bits(),
        |components, _, value| {
            components
                .joypad
                .set_bits(value, components.interrupt_controller)
        },
    );

    pub const SERIAL: Segment = Segment::Nested(|address| match address {
        0xFF01 => &Segment::Leaf(
            |components, _| components.serial.data(),
            |components, _, value| components.serial.set_data(value),
        ),
        0xFF02 => &Segment::Leaf(
            |components, _| components.serial.control_bits(),
            |components, _, value| components.serial.set_control_bits(value),
        ),
        _ => unreachable!(),
    });

    pub const TIMER: Segment = Segment::Nested(|address| match address {
        0xFF04 => &Segment::Leaf(
            |components, _| components.timer.divider_register(),
            |components, _, _| components.timer.reset_divider(),
        ),
        0xFF05 => &Segment::Leaf(
            |components, _| components.timer.counter(),
            |components, _, value| components.timer.set_counter(value),
        ),
        0xFF06 => &Segment::Leaf(
            |components, _| components.timer.modulo(),
            |components, _, value| components.timer.set_modulo(value),
        ),
        0xFF07 => &Segment::Leaf(
            |components, _| components.timer.control_bits(),
            |components, _, value| components.timer.set_control_bits(value),
        ),
        _ => unreachable!(),
    });

    pub const INTERRUPT_REQUESTED: Segment = Segment::Leaf(
        |components, _| components.interrupt_controller.requested_bits(),
        |components, _, value| components.interrupt_controller.set_requested_bits(value),
    );

    pub const APU: Segment = Segment::Leaf(|_, _| 0, |_, _, _| {});

    pub const PPU: Segment = Segment::Nested(|address| match address {
        0x8000..=0x9FFF => &Segment::Offset(
            0x8000,
            &Segment::Leaf(
                |components, address| components.ppu.read_vram(address),
                |components, address, value| components.ppu.write_vram(address, value),
            ),
        ),
        0xFE00..=0xFE9F => &Segment::Leaf(|_, _| 0, |_, _, _| {}),
        0xFF40..=0xFF4F => &Segment::Leaf(|_, _| 0, |_, _, _| {}),
        0xFF51..=0xFF55 => &Segment::Leaf(|_, _| 0, |_, _, _| {}),
        0xFF68..=0xFF6C => &Segment::Leaf(|_, _| 0, |_, _, _| {}),
        _ => unreachable!(),
    });

    pub const IR: Segment = Segment::Leaf(|_, _| 0, |_, _, _| {});

    pub const INTERRUPT_ENABLED: Segment = Segment::Leaf(
        |components, _| components.interrupt_controller.enabled_bits(),
        |components, _, value| components.interrupt_controller.set_enabled_bits(value),
    );

    pub const ROOT: Segment = Segment::Nested(|address| {
        match address {
            0x0000..=0x7FFF => &CARTRIDGE,
            0x8000..=0x9FFF => &PPU,
            0xA000..=0xBFFF => &CARTRIDGE,
            0xC000..=0xFDFF => &WRAM,
            0xFE00..=0xFE9F => &PPU,
            0xFEA0..=0xFEFF => &UNUSABLE,
            0xFF00 => &JOYPAD,
            0xFF01..=0xFF02 => &SERIAL,
            0xFF03 => &UNKNOWN,
            0xFF04..=0xFF07 => &TIMER,
            0xFF08..=0xFF0E => &UNKNOWN,
            0xFF0F => &INTERRUPT_REQUESTED,
            0xFF10..=0xFF3F => &APU,
            0xFF40..=0xFF4F => &PPU,
            0xFF50 => &UNKNOWN,
            0xFF51..=0xFF55 => &PPU,
            0xFF56 => &IR,
            0xFF57..=0xFF67 => &UNKNOWN,
            0xFF68..=0xFF6C => &PPU,
            0xFF6D..=0xFF6F => &UNKNOWN,
            0xFF70 => &WRAM,
            0xFF71 => &UNKNOWN,
            0xFF72..=0xFF75 => &UNKNOWN, // undocumented registers?
            0xFF76..=0xFF77 => &APU,
            0xFF78..=0xFF7F => &UNKNOWN,
            0xFF80..=0xFFFE => &HRAM,
            0xFFFF => &INTERRUPT_ENABLED,
        }
    });
}

#[derive(Debug)]
pub struct MappedMemory<'a>(Components<'a>);

impl<'a> MappedMemory<'a> {
    pub fn new(components: Components<'a>) -> Self {
        Self(components)
    }
}

impl<'a> Memory for MappedMemory<'a> {
    fn read(&self, address: u16) -> u8 {
        segment::ROOT.read(&self.0, address)
    }

    fn write(&mut self, address: u16, value: u8) {
        segment::ROOT.write(&mut self.0, address, value)
    }
}

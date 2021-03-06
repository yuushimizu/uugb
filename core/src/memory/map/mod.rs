mod apu;
mod cartridge;
mod cgb;
mod hram;
mod interrupt;
mod ir;
mod joypad;
mod ppu;
mod segment;
mod serial;
mod timer;
mod unknown;
mod unusable;
mod wram;

use apu::APU;
use cartridge::CARTRIDGE;
use cgb::CGB_REGISTERS;
use hram::HRAM;
use interrupt::{INTERRUPT_ENABLED, INTERRUPT_REQUESTED};
use ir::IR;
use joypad::JOYPAD;
use ppu::{OAM, PPU, VRAM};
use segment::Segment;
use serial::SERIAL;
use timer::TIMER;
use unknown::UNKNOWN;
use unusable::UNUSABLE;
use wram::WRAM;

pub const ROOT: Segment = Segment::Nested(|address| {
    match address {
        0x0000..=0x7FFF => &CARTRIDGE,
        0x8000..=0x9FFF => &VRAM,
        0xA000..=0xBFFF => &CARTRIDGE,
        0xC000..=0xFDFF => &WRAM,
        0xFE00..=0xFE9F => &OAM,
        0xFEA0..=0xFEFF => &UNUSABLE,
        0xFF00 => &JOYPAD,
        0xFF01..=0xFF02 => &SERIAL,
        0xFF03 => &UNKNOWN,
        0xFF04..=0xFF07 => &TIMER,
        0xFF08..=0xFF0E => &UNKNOWN,
        0xFF0F => &INTERRUPT_REQUESTED,
        0xFF10..=0xFF3F => &APU,
        0xFF40..=0xFF4C => &PPU,
        0xFF4D => &CGB_REGISTERS,
        0xFF4E..=0xFF55 => &PPU,
        0xFF56 => &IR,
        0xFF57..=0xFF67 => &UNKNOWN,
        0xFF68..=0xFF6C => &PPU,
        0xFF6D..=0xFF6F => &UNKNOWN,
        0xFF70 => &WRAM, // CGB: WRAM Bank
        0xFF71 => &UNKNOWN,
        0xFF72..=0xFF75 => &UNKNOWN, // undocumented registers?
        0xFF76..=0xFF77 => &APU,
        0xFF78..=0xFF7F => &UNKNOWN,
        0xFF80..=0xFFFE => &HRAM,
        0xFFFF => &INTERRUPT_ENABLED,
    }
});

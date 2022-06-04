use super::{Mbc, MbcContext};
use std::cmp::max;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BankingMode {
    Simple,
    Advanced,
}

impl From<u8> for BankingMode {
    fn from(value: u8) -> Self {
        if value & 0b1 == 0b0 {
            Self::Simple
        } else {
            Self::Advanced
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mbc1 {
    rom_bank_number_lower: usize,
    ram_bank_number_or_rom_bank_number_upper: usize,
    ram_enabled: bool,
    banking_mode: BankingMode,
}

impl Default for Mbc1 {
    fn default() -> Self {
        Self {
            rom_bank_number_lower: 1,
            ram_bank_number_or_rom_bank_number_upper: 0,
            ram_enabled: false,
            banking_mode: BankingMode::Simple,
        }
    }
}

impl Mbc1 {
    fn first_rom_bank_number(&self) -> usize {
        use BankingMode::*;
        match self.banking_mode {
            Simple => 0,
            Advanced => self.ram_bank_number_or_rom_bank_number_upper << 5,
        }
    }

    fn rom_bank_number(&self) -> usize {
        self.ram_bank_number_or_rom_bank_number_upper << 5 | self.rom_bank_number_lower
    }

    fn ram_bank_number(&self) -> usize {
        use BankingMode::*;
        match self.banking_mode {
            Simple => 0,
            Advanced => self.ram_bank_number_or_rom_bank_number_upper,
        }
    }
}

impl Mbc for Mbc1 {
    fn read_rom(&self, context: &dyn MbcContext, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => context.read_from_rom_bank(self.first_rom_bank_number(), address),
            0x4000..=0x7FFF => context.read_from_rom_bank(self.rom_bank_number(), address - 0x4000),
            _ => unreachable!(),
        }
    }

    fn write_rom(&mut self, _: &mut dyn MbcContext, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                self.ram_enabled = value & 0xF == 0xA;
            }
            0x2000..=0x3FFF => {
                self.rom_bank_number_lower = max(1, value & 0b0001_1111) as usize;
            }
            0x4000..=0x5FFF => {
                self.ram_bank_number_or_rom_bank_number_upper = (value & 0b11) as usize;
            }
            0x6000..=0x7FFF => {
                self.banking_mode = value.into();
            }
            _ => unreachable!(),
        }
    }

    fn read_ram(&self, context: &dyn MbcContext, address: u16) -> u8 {
        if self.ram_enabled {
            context.read_from_ram_bank(self.ram_bank_number(), address)
        } else {
            0xFF
        }
    }

    fn write_ram(&mut self, context: &mut dyn MbcContext, address: u16, value: u8) {
        if self.ram_enabled {
            context.write_to_ram_bank(self.ram_bank_number(), address, value);
        }
    }
}

// TODO MBC1m?

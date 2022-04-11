use super::{Mbc, MbcContext};
use std::{cmp::max, ops::Range};

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

#[derive(Debug)]
pub struct Mbc1 {
    rom_bank_number_lower: u8,
    ram_bank_number_or_rom_bank_number_upper: u8,
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
    fn first_rom_bank_number(&self) -> u8 {
        use BankingMode::*;
        match self.banking_mode {
            Simple => 0,
            Advanced => self.ram_bank_number_or_rom_bank_number_upper << 5,
        }
    }

    fn rom_bank_number(&self) -> u8 {
        self.ram_bank_number_or_rom_bank_number_upper << 5 | self.rom_bank_number_lower
    }

    fn ram_bank_number(&self) -> u8 {
        use BankingMode::*;
        match self.banking_mode {
            Simple => 0,
            Advanced => self.ram_bank_number_or_rom_bank_number_upper,
        }
    }

    fn set_rom_bank_number_lower(&mut self, value: u8) {
        self.rom_bank_number_lower = max(1, value & 0b0001_1111);
    }

    fn set_ram_bank_number_or_rom_bank_number_upper(&mut self, value: u8) {
        self.ram_bank_number_or_rom_bank_number_upper = value & 0b11
    }

    fn set_ram_enabled(&mut self, value: u8) {
        self.ram_enabled = value & 0xF == 0xA
    }

    fn set_banking_mode(&mut self, value: u8) {
        self.banking_mode = value.into()
    }
}

const ROM_BANK_SIZE: usize = 0x4000;

const RAM_BANK_SIZE: usize = 0x2000;

fn bin_digits(n: u8) -> u8 {
    let mut rest = n | 0b1;
    let mut result = 0;
    while rest != 0 {
        result += 1;
        rest >>= 1;
    }
    result
}

fn bit_mask(n: u8) -> u8 {
    0b1 << (bin_digits(n) - 0b1)
}

trait MbcContextHelpers: MbcContext {
    fn rom_bank_mask(&self) -> u8 {
        bit_mask(max(
            1,
            ((self.rom().len() as f64 / ROM_BANK_SIZE as f64).ceil() as u8).saturating_sub(1),
        ))
    }

    fn rom_bank(&self, number: u8) -> &[u8] {
        let start = (number & self.rom_bank_mask()) as usize * ROM_BANK_SIZE;
        &self.rom()[start..start + ROM_BANK_SIZE]
    }

    fn ram_bank_mask(&self) -> u8 {
        bit_mask(((self.ram().len() as f64 / RAM_BANK_SIZE as f64).ceil() as u8).saturating_sub(1))
    }

    fn ram_bank_range(&self, number: u8) -> Range<usize> {
        let start = (number & self.ram_bank_mask()) as usize * RAM_BANK_SIZE;
        let end = start + RAM_BANK_SIZE;
        if end >= self.ram().len() {
            0..0
        } else {
            start..end
        }
    }

    fn ram_bank(&self, number: u8) -> &[u8] {
        &self.ram()[self.ram_bank_range(number)]
    }

    fn ram_bank_mut(&mut self, number: u8) -> &mut [u8] {
        let range = self.ram_bank_range(number);
        &mut self.ram_mut()[range]
    }
}

impl<T: MbcContext + ?Sized> MbcContextHelpers for T {}

impl Mbc for Mbc1 {
    fn read_rom(&self, context: &dyn MbcContext, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => *context
                .rom_bank(self.first_rom_bank_number())
                .get(address as usize)
                .unwrap_or(&0x00),
            0x4000..=0x7FFF => *context
                .rom_bank(self.rom_bank_number())
                .get(address as usize - 0x4000)
                .unwrap_or(&0x00),
            _ => unreachable!(),
        }
    }

    fn write_rom(&mut self, _: &mut dyn MbcContext, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.set_ram_enabled(value),
            0x2000..=0x3FFF => self.set_rom_bank_number_lower(value),
            0x4000..=0x5FFF => self.set_ram_bank_number_or_rom_bank_number_upper(value),
            0x6000..=0x7FFF => self.set_banking_mode(value),
            _ => unreachable!(),
        }
    }

    fn read_ram(&self, context: &dyn MbcContext, address: u16) -> u8 {
        *context
            .ram_bank(self.ram_bank_number())
            .get(address as usize)
            .unwrap_or(&0x00)
    }

    fn write_ram(&mut self, context: &mut dyn MbcContext, address: u16, value: u8) {
        if self.ram_enabled {
            if let Some(e) = context
                .ram_bank_mut(self.ram_bank_number())
                .get_mut(address as usize)
            {
                *e = value;
            }
        }
    }
}

// TODO MBC1m?

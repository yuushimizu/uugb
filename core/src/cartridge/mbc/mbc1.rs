use super::{Context, Mbc};
use log;
use std::cmp::{max, min};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BankingMode {
    Simple,
    Advanced,
}

#[derive(Debug)]
pub struct Mbc1 {
    rom_bank: u8,
    ram_bank_or_rom_bank_upper: u8,
    ram_enabled: bool,
    banking_mode: BankingMode,
}

impl Mbc1 {
    pub fn new() -> Self {
        Self {
            rom_bank: 1,
            ram_bank_or_rom_bank_upper: 0,
            ram_enabled: false,
            banking_mode: BankingMode::Simple,
        }
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
    0b1 << bin_digits(n) - 0b1
}

impl Context {
    fn rom_bank_mask(&self) -> u8 {
        bit_mask(max(
            1,
            ((self.rom_size() as f64 / ROM_BANK_SIZE as f64).ceil() as u8).saturating_sub(1),
        ))
    }

    fn rom_bank(&self, number: u8) -> &[u8] {
        let start = (number & self.rom_bank_mask()) as usize * ROM_BANK_SIZE;
        &self.rom[start..start + ROM_BANK_SIZE]
    }

    fn ram_bank_mask(&self) -> u8 {
        bit_mask(((self.ram_size() as f64 / RAM_BANK_SIZE as f64).ceil() as u8).saturating_sub(1))
    }

    fn ram_bank(&self, number: u8) -> &[u8] {
        let start = (number & self.ram_bank_mask()) as usize * RAM_BANK_SIZE;
        &self.ram[start..start + RAM_BANK_SIZE]
    }
}

impl Mbc for Mbc1 {
    fn read(self: &Self, context: &Context, address: u16) -> u8 {
        use BankingMode::*;
        match address {
            0x0000..=0x3FFF => context.rom_bank(match self.banking_mode {
                Simple => 0,
                Advanced => self.ram_bank_or_rom_bank_upper << 5,
            })[address as usize],
            0x4000..=0x7FFF => context
                .rom_bank(self.rom_bank | self.ram_bank_or_rom_bank_upper << 5)
                [address as usize - 0x4000],
            0xA000..=0xBFFF => context.ram_bank(match self.banking_mode {
                Simple => 0,
                Advanced => self.ram_bank_or_rom_bank_upper,
            })[address as usize - 0xA000],
            _ => {
                log::warn!("MBC1: Reading from the address {:04X}", address);
                0
            }
        }
    }

    fn write(self: &mut Self, context: &mut Context, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram_enabled = value & 0b1111 == 0b1010,
            0x2000..=0x3FFF => {
                self.rom_bank = max(1, value & context.rom_bank_mask() & 0b0001_1111);
            }
            0x4000..=0x5FFF => self.ram_bank_or_rom_bank_upper = value & 0b11,
            0x6000..=0x7FFF => {
                self.banking_mode = if value & 0b1 == 0b0 {
                    BankingMode::Simple
                } else {
                    BankingMode::Advanced
                }
            }
            _ => log::warn!(
                "MBC1: Writing the value {:02X} to the address {:04X}",
                value,
                address
            ),
        }
    }
}

// TODO MBC1m?

use super::{Context, Mbc};

#[derive(Debug)]
pub struct Mbc1 {
    rom_bank_number: u8,
    upper_rom_bank_number: u8,
    advanced_rom_banking_mode: bool,
    ram_bank_number: u8,
    ram_enabled: bool,
}

impl Mbc1 {
    pub fn new() -> Self {
        Self {
            rom_bank_number: 1,
            upper_rom_bank_number: 0,
            advanced_rom_banking_mode: false,
            ram_bank_number: 0,
            ram_enabled: false,
        }
    }
}

impl Mbc for Mbc1 {
    fn read(self: &Self, context: &Context, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => panic!(),
            0x4000..=0x7FFF => panic!(),
            0xA000..=0xBFFF => panic!(),
            _ => 0,
        }
    }

    fn write(self: &mut Self, context: &mut Context, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram_enabled = value & 0xF == 0xA,
            0x2000..=0x3FFF => panic!(),
            _ => panic!(),
        }
    }
}

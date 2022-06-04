use super::{Mbc, MbcContext};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mbc5 {
    rom_bank_number: usize,
    ram_enabled: bool,
    ram_bank_number: usize,
}

impl Default for Mbc5 {
    fn default() -> Self {
        Self {
            rom_bank_number: 1,
            ram_enabled: false,
            ram_bank_number: 0,
        }
    }
}

impl Mbc for Mbc5 {
    fn read_rom(&self, context: &dyn MbcContext, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => context.read_from_rom_bank(0, address),
            0x4000..=0x7FFF => context.read_from_rom_bank(self.rom_bank_number, address - 0x4000),
            _ => unreachable!(),
        }
    }

    fn write_rom(&mut self, _: &mut dyn MbcContext, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                self.ram_enabled = value & 0xF == 0xA;
            }
            0x2000..=0x2FFF => {
                self.rom_bank_number = (self.rom_bank_number & !0xFF) | value as usize;
            }
            0x3000..=0x3FFF => {
                self.rom_bank_number =
                    ((value & 0b1) as usize) << 8 | (self.rom_bank_number & 0xFF);
            }
            0x4000..=0x5FFF => {
                self.ram_bank_number = (value & 0xF) as usize;
            }
            _ => {}
        }
    }

    fn read_ram(&self, context: &dyn MbcContext, address: u16) -> u8 {
        if self.ram_enabled {
            context.read_from_ram_bank(self.ram_bank_number, address)
        } else {
            0xFF
        }
    }

    fn write_ram(&mut self, context: &mut dyn MbcContext, address: u16, value: u8) {
        if self.ram_enabled {
            context.write_to_ram_bank(self.ram_bank_number, address, value);
        }
    }
}

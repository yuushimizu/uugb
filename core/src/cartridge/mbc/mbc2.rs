use super::{Mbc, MbcContext};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mbc2 {
    rom_bank_number: u8,
    ram_enabled: bool,
}

impl Default for Mbc2 {
    fn default() -> Self {
        Self {
            rom_bank_number: 1,
            ram_enabled: false,
        }
    }
}

impl Mbc for Mbc2 {
    fn internal_ram_size(&self) -> usize {
        0x200
    }

    fn read_rom(&self, context: &dyn MbcContext, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => context.get_from_rom_bank(0, address),
            0x4000..=0x7FFF => context.get_from_rom_bank(self.rom_bank_number, address - 0x4000),
            _ => unreachable!(),
        }
    }

    fn write_rom(&mut self, _: &mut dyn MbcContext, address: u16, value: u8) {
        match address {
            0x0000..=0x3FFF => {
                if address & 0x0100 == 0x0100 {
                    self.rom_bank_number = value & 0xF;
                } else {
                    self.ram_enabled = value == 0xA;
                }
            }
            _ => {}
        }
    }

    fn read_ram(&self, context: &dyn MbcContext, address: u16) -> u8 {
        context.get_from_ram_bank(0, address % 0x200)
    }

    fn write_ram(&mut self, context: &mut dyn MbcContext, address: u16, value: u8) {
        if self.ram_enabled {
            context.set_to_ram_bank(0, address % 0x200, value);
        }
    }
}

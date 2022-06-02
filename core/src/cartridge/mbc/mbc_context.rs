use std::cmp::{max, min};

const ROM_BANK_SIZE: usize = 0x4000;

const RAM_BANK_SIZE: usize = 0x2000;

pub trait MbcContext {
    fn rom(&self) -> &[u8];

    fn ram(&self) -> &[u8];

    fn ram_mut(&mut self) -> &mut [u8];

    fn rom_bank(&self, bank_number: u8) -> &[u8] {
        let start = bank_number as usize % max(1, self.rom().len() / ROM_BANK_SIZE) * ROM_BANK_SIZE;
        &self.rom()[start..min(self.rom().len(), start + ROM_BANK_SIZE)]
    }

    fn get_from_rom_bank(&self, bank_number: u8, index: u16) -> u8 {
        *self
            .rom_bank(bank_number)
            .get(index as usize)
            .unwrap_or(&0xFF)
    }

    fn ram_bank(&self, bank_number: u8) -> &[u8] {
        let start = bank_number as usize % max(1, self.ram().len() / RAM_BANK_SIZE) * RAM_BANK_SIZE;
        &self.ram()[start..min(self.ram().len(), start + RAM_BANK_SIZE)]
    }

    fn get_from_ram_bank(&self, bank_number: u8, index: u16) -> u8 {
        *self
            .ram_bank(bank_number)
            .get(index as usize)
            .unwrap_or(&0xFF)
    }

    fn ram_bank_mut(&mut self, bank_number: u8) -> &mut [u8] {
        let ram_size = self.ram().len();
        let start = bank_number as usize % max(1, ram_size / RAM_BANK_SIZE) * RAM_BANK_SIZE;
        &mut self.ram_mut()[start..min(ram_size, start + RAM_BANK_SIZE)]
    }

    fn set_to_ram_bank(&mut self, bank_number: u8, index: u16, value: u8) {
        if let Some(e) = self.ram_bank_mut(bank_number).get_mut(index as usize) {
            *e = value;
        }
    }
}

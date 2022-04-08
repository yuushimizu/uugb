use log;

#[derive(Debug)]
pub struct Wram {
    data: Vec<u8>,
}

impl Default for Wram {
    fn default() -> Self {
        Self {
            data: vec![0x00u8; 0x2000],
        }
    }
}

const PRIMARY_START: u16 = 0xC000;

const PRIMARY_END: u16 = 0xCFFF;

const BANK_START: u16 = 0xD000;

const BANK_END: u16 = 0xDFFF;

const BANK_SWITCH_ADDRESS: u16 = 0xFF70;

impl Wram {
    fn read_primary(&self, address: u16) -> u8 {
        *self.data.get(address as usize).unwrap_or_else(|| {
            log::warn!("WRAM: Attempt to read from out of bounds: {:04X}", address);
            &0x00
        })
    }

    fn write_primary(&mut self, address: u16, value: u8) {
        match self.data.get_mut(address as usize) {
            Some(e) => *e = value,
            None => log::warn!("WRAM: Attempt to write to out of bounds: {:04X}", address),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            PRIMARY_START..=PRIMARY_END => self.read_primary(address - PRIMARY_START),
            BANK_START..=BANK_END => self.read_primary(address - BANK_START),
            BANK_SWITCH_ADDRESS => 0x00,
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            PRIMARY_START..=PRIMARY_END => self.write_primary(address - PRIMARY_START, value),
            BANK_START..=BANK_END => self.write_primary(address - BANK_START, value),
            BANK_SWITCH_ADDRESS => {}
            _ => unreachable!(),
        }
    }
}

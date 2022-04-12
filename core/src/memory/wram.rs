use log;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Wram {
    primary: Vec<u8>,
    bank: Vec<u8>,
}

impl Default for Wram {
    fn default() -> Self {
        Self {
            primary: vec![0x00u8; 0x1000],
            bank: vec![0x00u8; 0x1000],
        }
    }
}

impl Wram {
    pub fn read(&self, address: u16) -> u8 {
        *self.primary.get(address as usize).unwrap_or_else(|| {
            log::warn!("WRAM: Attempt to read from out of bounds: {:04X}", address);
            &0xFF
        })
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match self.primary.get_mut(address as usize) {
            Some(e) => *e = value,
            None => log::warn!("WRAM: Attempt to write to out of bounds: {:04X}", address),
        }
    }

    pub fn read_bank(&self, address: u16) -> u8 {
        *self.bank.get(address as usize).unwrap_or_else(|| {
            log::warn!(
                "WRAM: (Bank) Attempt to read from out of bounds: {:04X}",
                address
            );
            &0xFF
        })
    }

    pub fn write_bank(&mut self, address: u16, value: u8) {
        match self.bank.get_mut(address as usize) {
            Some(e) => *e = value,
            None => log::warn!(
                "WRAM: (Bank) Attempt to write to out of bounds: {:04X}",
                address
            ),
        }
    }

    pub fn bank_switch(&self) -> u8 {
        0x00
    }

    pub fn set_bank_switch(&self, _value: u8) {}
}

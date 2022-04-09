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

impl Wram {
    pub fn read(&self, address: u16) -> u8 {
        *self.data.get(address as usize).unwrap_or_else(|| {
            log::warn!("WRAM: Attempt to read from out of bounds: {:04X}", address);
            &0x00
        })
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match self.data.get_mut(address as usize) {
            Some(e) => *e = value,
            None => log::warn!("WRAM: Attempt to write to out of bounds: {:04X}", address),
        }
    }

    pub fn read_bank(&self, address: u16) -> u8 {
        self.read(address)
    }

    pub fn write_bank(&mut self, address: u16, value: u8) {
        self.write(address, value);
    }

    pub fn bank_switch(&self) -> u8 {
        0x00
    }

    pub fn set_bank_switch(&self, value: u8) {}
}

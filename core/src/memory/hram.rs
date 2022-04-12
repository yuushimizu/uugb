#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hram {
    data: Vec<u8>,
}

impl Default for Hram {
    fn default() -> Self {
        Self {
            data: vec![0x00u8; 0x7F],
        }
    }
}

impl Hram {
    pub fn read(&self, address: u16) -> u8 {
        *self.data.get(address as usize).unwrap_or_else(|| {
            log::warn!("HRAM: Attempt to read from out of bounds: {:04X}", address);
            &0xFF
        })
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match self.data.get_mut(address as usize) {
            Some(e) => *e = value,
            None => log::warn!("HRAM: Attempt to write to out of bounds: {:04X}", address),
        }
    }
}

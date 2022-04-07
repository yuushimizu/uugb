use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GlobalChecksum {
    value: u16,
    calculated_value: u16,
}

const UPPER_ADDRESS: usize = 0x014E;

const LOWER_ADDRESS: usize = 0x014F;

impl GlobalChecksum {
    pub fn calculate(rom: &[u8]) -> u16 {
        rom.iter()
            .enumerate()
            .filter(|&(address, _byte)| address != UPPER_ADDRESS && address != LOWER_ADDRESS)
            .fold(0x0000, |acc, (_address, &byte)| {
                acc.wrapping_add(byte as u16)
            })
    }

    pub fn load(rom: &[u8]) -> Self {
        Self {
            value: (rom[UPPER_ADDRESS] as u16) << 8 | rom[LOWER_ADDRESS] as u16,
            calculated_value: Self::calculate(rom),
        }
    }

    pub fn value(&self) -> u16 {
        self.value
    }

    pub fn calculated_value(&self) -> u16 {
        self.calculated_value
    }

    pub fn is_matched(&self) -> bool {
        self.value == self.calculated_value
    }
}

impl fmt::Display for GlobalChecksum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04X}", self.value)
    }
}

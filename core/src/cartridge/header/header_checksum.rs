use std::fmt;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HeaderChecksum {
    value: u8,
    calculated_value: u8,
}

const ADDRESS: usize = 0x014D;

const TARGET_RANGE: RangeInclusive<usize> = 0x0134..=0x014C;

impl HeaderChecksum {
    pub fn calculate(rom: &[u8]) -> u8 {
        rom[TARGET_RANGE]
            .iter()
            .fold(0x00, |acc, &byte| acc.wrapping_sub(byte).wrapping_sub(1))
    }

    pub fn load(rom: &[u8]) -> Self {
        Self {
            value: rom[ADDRESS],
            calculated_value: Self::calculate(rom),
        }
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn calculated_value(&self) -> u8 {
        self.calculated_value
    }

    pub fn is_matched(&self) -> bool {
        self.value == self.calculated_value
    }
}

impl fmt::Display for HeaderChecksum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02X}", self.value)
    }
}

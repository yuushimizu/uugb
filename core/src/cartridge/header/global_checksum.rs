use std::fmt;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GlobalChecksum {
    bytes: Vec<u8>,
}

const RANGE: RangeInclusive<usize> = 0x014E..=0x014F;

impl GlobalChecksum {
    pub fn load(rom_bytes: &[u8]) -> Self {
        Self {
            bytes: rom_bytes[RANGE].into(),
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn value(&self) -> u16 {
        ((self.bytes[0] as u16) << 8) | self.bytes[1] as u16
    }
}

impl fmt::Display for GlobalChecksum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02X}", self.value())
    }
}

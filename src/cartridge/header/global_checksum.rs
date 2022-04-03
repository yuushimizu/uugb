use byteorder::{BigEndian, ReadBytesExt};
use std::fmt;
use std::io::Cursor;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GlobalChecksum {
    pub bytes: Vec<u8>,
}

const RANGE: RangeInclusive<usize> = 0x014E..=0x014F;

impl GlobalChecksum {
    pub fn load(rom_bytes: &[u8]) -> Self {
        Self {
            bytes: rom_bytes[RANGE].into(),
        }
    }

    pub fn value(&self) -> u16 {
        Cursor::new(&self.bytes)
            .read_u16::<BigEndian>()
            .unwrap_or(0)
    }
}

impl fmt::Display for GlobalChecksum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02X}", self.value())
    }
}

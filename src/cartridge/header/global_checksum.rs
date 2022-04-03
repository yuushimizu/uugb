use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GlobalChecksum {
    bytes: Vec<u8>,
    value: u16,
}

const RANGE: RangeInclusive<usize> = 0x014E..=0x014F;

impl GlobalChecksum {
    pub fn load(rom_bytes: &[u8]) -> Self {
        let bytes = rom_bytes[RANGE].into();
        let value = Cursor::new(&bytes).read_u16::<BigEndian>().unwrap_or(0);
        Self { bytes, value }
    }
}

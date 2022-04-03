use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EntryPoint {
    bytes: Vec<u8>,
}

const RANGE: RangeInclusive<usize> = 0x0100..=0x0103;

impl EntryPoint {
    pub fn load(rom_bytes: &[u8]) -> Self {
        Self {
            bytes: rom_bytes[RANGE].into(),
        }
    }
}

use std::fmt;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EntryPoint {
    pub bytes: Vec<u8>,
}

const RANGE: RangeInclusive<usize> = 0x0100..=0x0103;

impl EntryPoint {
    pub fn load(rom: &[u8]) -> Self {
        Self {
            bytes: rom[RANGE].into(),
        }
    }
}

impl fmt::Display for EntryPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.bytes
                .iter()
                .map(|x| format!("{:02X}", x))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

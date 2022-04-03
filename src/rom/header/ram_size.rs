#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RamSize {
    Unknown,
    Kb0,
    Kb8,
    Kb32,
    Kb128,
    Kb64,
}

impl From<u8> for RamSize {
    fn from(value: u8) -> Self {
        use RamSize::*;
        match value {
            0x00 => Kb0,
            0x02 => Kb8,
            0x03 => Kb32,
            0x04 => Kb128,
            0x05 => Kb64,
            _ => Unknown,
        }
    }
}

const POSITION: usize = 0x0149;

impl RamSize {
    pub fn load(rom_bytes: &[u8]) -> Self {
        rom_bytes[POSITION].into()
    }
}

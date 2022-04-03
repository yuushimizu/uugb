#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RamSizeAmount {
    Unknown,
    None,
    Kb8,
    Kb32,
    Kb128,
    Kb64,
}

impl From<u8> for RamSizeAmount {
    fn from(code: u8) -> Self {
        use RamSizeAmount::*;
        match code {
            0x00 => None,
            0x02 => Kb8,
            0x03 => Kb32,
            0x04 => Kb128,
            0x05 => Kb64,
            _ => Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RamSize {
    code: u8,
    amount: RamSizeAmount,
}

impl From<u8> for RamSize {
    fn from(code: u8) -> Self {
        Self {
            code,
            amount: code.into(),
        }
    }
}

const POSITION: usize = 0x0149;

impl RamSize {
    pub fn load(rom_bytes: &[u8]) -> Self {
        rom_bytes[POSITION].into()
    }
}

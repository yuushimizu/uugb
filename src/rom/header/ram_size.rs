#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RamSizeAmount {
    Unknown,
    Kb(i64),
}

impl From<u8> for RamSizeAmount {
    fn from(code: u8) -> Self {
        use RamSizeAmount::*;
        match code {
            0x00 => Kb(0),
            0x02 => Kb(8),
            0x03 => Kb(32),
            0x04 => Kb(128),
            0x05 => Kb(64),
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

use std::fmt;

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

impl fmt::Display for RamSizeAmount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use RamSizeAmount::*;
        write!(
            f,
            "{}",
            match self {
                Unknown => "Unknown",
                None => "None",
                Kb8 => "8KB",
                Kb32 => "32KB",
                Kb128 => "128KB",
                Kb64 => "64KB",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RamSize {
    pub code: u8,
}

impl From<u8> for RamSize {
    fn from(code: u8) -> Self {
        Self { code }
    }
}

const POSITION: usize = 0x0149;

impl RamSize {
    pub fn load(rom: &[u8]) -> Self {
        rom[POSITION].into()
    }

    pub fn amount(&self) -> RamSizeAmount {
        self.code.into()
    }
}

impl fmt::Display for RamSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({:02X})", self.amount(), self.code)
    }
}

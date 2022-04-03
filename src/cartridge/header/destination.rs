use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DestinationType {
    Japanese,
    NonJapanese,
}

impl From<u8> for DestinationType {
    fn from(code: u8) -> Self {
        use DestinationType::*;
        match code {
            0x00 => Japanese,
            _ => NonJapanese,
        }
    }
}
impl fmt::Display for DestinationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DestinationType::*;
        write!(
            f,
            "{}",
            match self {
                Japanese => "Japanese",
                NonJapanese => "NonJapanese",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Destination {
    pub code: u8,
}

impl From<u8> for Destination {
    fn from(code: u8) -> Self {
        Self { code }
    }
}

const POSITION: usize = 0x014A;

impl Destination {
    pub fn load(rom: &[u8]) -> Self {
        rom[POSITION].into()
    }

    pub fn destination_type(&self) -> DestinationType {
        self.code.into()
    }
}

impl fmt::Display for Destination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({:02X})", self.destination_type(), self.code)
    }
}

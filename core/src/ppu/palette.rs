pub enum Color {
    White,
    LightGray,
    DarkGray,
    Black,
}

impl From<u8> for Color {
    fn from(bits: u8) -> Self {
        match bits & 0b11 {
            0b11 => Self::Black,
            0b10 => Self::DarkGray,
            0b01 => Self::LightGray,
            _ => Self::White,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Palette {
    bits: u8,
}

impl Palette {
    pub fn bits(&self) -> u8 {
        self.bits
    }

    pub fn set_bits(&mut self, value: u8) {
        self.bits = value;
    }

    pub fn apply(&self, data: u8) -> u8 {
        self.bits >> (data * 2) & 0b11
    }
}

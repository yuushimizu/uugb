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

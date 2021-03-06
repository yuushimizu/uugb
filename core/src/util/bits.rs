pub trait Bits {
    fn bit(&self, d: u32) -> bool;

    fn set_bit(&self, d: u32) -> Self;

    fn reset_bit(&self, d: u32) -> Self;

    fn from_bits(bits: &[bool]) -> Self;
}

impl Bits for u8 {
    fn bit(&self, d: u32) -> bool {
        *self & (0b1 << d) != 0
    }

    fn set_bit(&self, d: u32) -> Self {
        *self | 0b1 << d
    }

    fn reset_bit(&self, d: u32) -> Self {
        *self & !(0b1 << d)
    }

    fn from_bits(bits: &[bool]) -> Self {
        bits.iter().fold(0x00, |acc, &bit| acc << 1 | (bit as u8))
    }
}

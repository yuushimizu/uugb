pub trait Bits {
    fn bit(&self, d: u32) -> bool;
}

impl Bits for u8 {
    fn bit(&self, d: u32) -> bool {
        *self & (0b1 << d) != 0
    }
}

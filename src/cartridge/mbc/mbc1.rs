use super::Mbc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mbc1 {
    rom: Vec<u8>,
}

impl Mbc1 {
    pub fn new(rom: Vec<u8>) -> Self {
        Self { rom }
    }
}

impl Mbc for Mbc1 {
    fn read(self: &Self, address: u16) -> u8 {
        0
    }

    fn write(self: &mut Self, address: u16, value: u8) {}
}

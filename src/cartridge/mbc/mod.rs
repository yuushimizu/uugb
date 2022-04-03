mod mbc1;

pub use mbc1::Mbc1;

pub trait Mbc {
    fn read(&self, address: u16) -> u8;

    fn write(&mut self, address: u16, value: u8);
}

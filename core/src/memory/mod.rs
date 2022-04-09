pub mod hram;
pub mod map;
pub mod wram;

pub use hram::Hram;
pub use map::MappedMemory;
pub use wram::Wram;

pub trait Memory {
    fn read(&self, address: u16) -> u8;

    fn write(&mut self, address: u16, value: u8);
}

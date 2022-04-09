pub mod components;
pub mod hram;
pub mod map;
pub mod wram;

pub use components::Components;
pub use hram::Hram;
pub use map::ROOT;
pub use wram::Wram;

#[derive(Debug)]
pub struct Memory<'a>(Components<'a>);

impl<'a> Memory<'a> {
    pub fn new(components: Components<'a>) -> Self {
        Self(components)
    }

    pub fn read(&self, address: u16) -> u8 {
        ROOT.read(&self.0, address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        ROOT.write(&mut self.0, address, value)
    }
}

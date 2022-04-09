pub mod components;
pub mod hram;
pub mod map;
pub mod wram;

pub use components::{Components, ComponentsRefs, ComponentsRefsMut};
pub use hram::Hram;
pub use map::ROOT;
pub use wram::Wram;

use std::fmt;

pub struct Memory<'a>(&'a mut dyn Components);

impl<'a> fmt::Debug for Memory<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Memory").finish()
    }
}

impl<'a> Memory<'a> {
    pub fn new(components: &'a mut impl Components) -> Self {
        Self(components)
    }

    pub fn read(&self, address: u16) -> u8 {
        let refs = self.0.refs();
        ROOT.read(&refs, address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let mut refs = self.0.refs_mut();
        ROOT.write(&mut refs, address, value)
    }
}

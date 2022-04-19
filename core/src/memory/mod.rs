pub mod dma;
pub mod hram;
pub mod wram;

mod components;
mod map;

pub use components::Components;
pub use dma::Dma;
pub use hram::Hram;
pub use map::ROOT;
pub use wram::Wram;

use std::fmt;

pub struct Memory<'a>(Components<'a>);

impl<'a> fmt::Debug for Memory<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Memory").finish()
    }
}

impl<'a> Memory<'a> {
    pub fn new(components: Components<'a>) -> Self {
        Self(components)
    }

    pub fn components(&self) -> &Components {
        &self.0
    }

    pub fn components_mut(&mut self) -> &mut Components<'a> {
        &mut self.0
    }

    pub fn read(&self, address: u16) -> u8 {
        ROOT.read(&self.0, address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        ROOT.write(&mut self.0, address, value)
    }

    pub fn tick(&mut self) {
        if let Some(dma_process) = self.0.dma.running_process().clone() {
            let value = self.read(dma_process.next_source());
            self.write(dma_process.next_destination(), value);
        }
        self.0.dma.tick()
    }
}

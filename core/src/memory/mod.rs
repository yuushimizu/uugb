pub mod dma;
pub mod hram;
pub mod wram;

mod context;
mod map;

pub use context::{ComponentsRefs, ComponentsRefsMut, Context};
pub use dma::Dma;
pub use hram::Hram;
pub use map::ROOT;
pub use wram::Wram;

use std::fmt;

pub struct Memory<'a>(&'a mut dyn Context);

impl<'a> fmt::Debug for Memory<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Memory").finish()
    }
}

impl<'a> Memory<'a> {
    pub fn new(components: &'a mut impl Context) -> Self {
        Self(components)
    }

    pub fn read(&self, address: u16) -> u8 {
        let components = self.0.components();
        ROOT.read(&components, address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let mut components = self.0.components_mut();
        ROOT.write(&mut components, address, value)
    }

    pub fn tick(&mut self) {
        let dma_requests = self
            .0
            .components_mut()
            .dma
            .running_requests()
            .iter()
            .map(|request| request.clone())
            .collect::<Vec<_>>();
        for dma_request in dma_requests {
            let value = self.read(dma_request.next_source());
            self.write(dma_request.next_destination(), value);
        }
        self.0.components_mut().dma.advance_running_requests()
    }
}

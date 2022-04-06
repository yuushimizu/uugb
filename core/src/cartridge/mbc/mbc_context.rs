use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MbcContext {
    pub rom: Rc<Vec<u8>>,
    pub ram: Vec<u8>,
}

impl MbcContext {
    pub fn rom_size(&self) -> usize {
        self.rom.len()
    }

    pub fn ram_size(&self) -> usize {
        self.ram.len()
    }
}

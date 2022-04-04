#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Context {
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
}

impl Context {
    pub fn rom_size(&self) -> usize {
        self.rom.len()
    }

    pub fn ram_size(&self) -> usize {
        self.ram.len()
    }
}

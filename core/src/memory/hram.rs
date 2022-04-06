#[derive(Debug)]
pub struct Hram {
    data: Vec<u8>,
}

impl Default for Hram {
    fn default() -> Self {
        Self {
            data: vec![0x00u8; 0x7F],
        }
    }
}

impl Hram {
    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }
}

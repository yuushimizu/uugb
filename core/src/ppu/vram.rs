#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileDataArea {
    Shifted,
    Origin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileMapArea {
    First,
    Second,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Vram {
    data: Vec<u8>,
}

impl Default for Vram {
    fn default() -> Self {
        Self {
            data: vec![0x00u8; 0x2000],
        }
    }
}

impl Vram {
    pub fn read(&self, address: u16) -> u8 {
        *self.data.get(address as usize).unwrap_or_else(|| {
            log::warn!("VRAM: Attempt to read from out of bounds: {:04X}", address);
            &0x00
        })
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match self.data.get_mut(address as usize) {
            Some(e) => *e = value,
            None => log::warn!("VRAM: Attempt to write to out of bounds: {:04X}", address),
        }
    }
}

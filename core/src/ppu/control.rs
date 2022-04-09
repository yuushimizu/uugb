use super::vram::TileDataArea;
use super::vram::TileMapArea;
use crate::util::bits::Bits;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Control {
    bits: u8,
}

impl Default for Control {
    fn default() -> Self {
        Self { bits: 0b10010001 }
    }
}

impl Control {
    pub fn bits(&self) -> u8 {
        self.bits
    }

    pub fn set_bits(&mut self, value: u8) {
        self.bits = value;
    }

    pub fn is_enabled(&self) -> bool {
        self.bits.bit(7)
    }

    pub fn window_tile_map_area(&self) -> TileMapArea {
        if self.bits.bit(6) {
            TileMapArea::Area1
        } else {
            TileMapArea::Area0
        }
    }

    pub fn window_enabled(&self) -> bool {
        self.bits.bit(5)
    }

    pub fn background_tile_data_area(&self) -> TileDataArea {
        if self.bits.bit(4) {
            TileDataArea::Area1
        } else {
            TileDataArea::Area0
        }
    }

    pub fn background_tile_map_area(&self) -> TileMapArea {
        if self.bits.bit(3) {
            TileMapArea::Area1
        } else {
            TileMapArea::Area0
        }
    }

    pub fn uses_large_object(&self) -> bool {
        self.bits.bit(2)
    }

    pub fn object_enabled(&self) -> bool {
        self.bits.bit(1)
    }

    pub fn background_and_window_enabled(&self) -> bool {
        self.bits.bit(0)
    }
}

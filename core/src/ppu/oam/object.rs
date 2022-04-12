use crate::{ppu::Vec2, util::bits::Bits};

pub const DATA_SIZE: usize = 4;

const X_OFFSET: u8 = 8;

const Y_OFFSET: u8 = 16;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Object {
    pub position: Vec2,
    pub tile_id: u8,
    pub is_under_background: bool,
    pub is_flipped_y: bool,
    pub is_flipped_x: bool,
    pub palette_number: u8,
    pub tile_bank_number: u8,     //  CGB
    pub color_palette_number: u8, // CGB
}

impl From<&[u8]> for Object {
    fn from(bytes: &[u8]) -> Self {
        let flags = bytes[3];
        Self {
            position: Vec2::new(bytes[1], bytes[0]).wrapping_sub(Vec2::new(X_OFFSET, Y_OFFSET)),
            tile_id: bytes[2],
            is_under_background: flags.bit(7),
            is_flipped_y: flags.bit(6),
            is_flipped_x: flags.bit(5),
            palette_number: flags.bit(4) as u8,
            tile_bank_number: flags.bit(3) as u8,
            color_palette_number: flags & 0b11,
        }
    }
}

impl Object {
    pub fn contains_x(&self, x: u8) -> bool {
        let adjusted_x = x.wrapping_add(X_OFFSET);
        let adjusted_object_x = self.position.x.wrapping_add(X_OFFSET);
        adjusted_x >= adjusted_object_x && adjusted_x < adjusted_object_x + 8
    }

    pub fn contains_y(&self, y: u8, large_object: bool) -> bool {
        let adjusted_y = y.wrapping_add(Y_OFFSET);
        let adjusted_object_y = self.position.y.wrapping_add(Y_OFFSET);
        adjusted_y >= adjusted_object_y
            && adjusted_y < adjusted_object_y + if large_object { 16 } else { 8 }
    }

    pub fn position_in_object(&self, position: Vec2, large_object: bool) -> Vec2 {
        let mut result = position.wrapping_sub(self.position);
        if self.is_flipped_x {
            result.x = 7u8.wrapping_sub(result.x);
        }
        if self.is_flipped_y {
            result.y = if large_object { 15u8 } else { 7u8 }.wrapping_sub(result.y);
        }
        result
    }
}

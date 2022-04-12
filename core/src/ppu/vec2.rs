use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Vec2 {
    pub x: u8,
    pub y: u8,
}

impl Vec2 {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn wrapping_add(&self, other: Self) -> Self {
        Self::new(self.x.wrapping_add(other.x), self.y.wrapping_add(other.y))
    }
}

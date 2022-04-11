use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Coordinate {
    pub x: u8,
    pub y: u8,
}

impl ops::Add<Coordinate> for Coordinate {
    type Output = Self;

    fn add(self, rhs: Coordinate) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

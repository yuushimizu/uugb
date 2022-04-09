use super::Coordinate;

pub trait Renderer {
    fn render(&mut self, position: Coordinate, color: u8);
}

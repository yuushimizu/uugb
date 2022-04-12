use super::{Color, Vec2};

pub trait Renderer {
    fn render(&mut self, position: Vec2, color: Color);
}

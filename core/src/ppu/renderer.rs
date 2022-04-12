use super::{Color, Vec2};

pub trait Renderer {
    fn render(&mut self, position: Vec2, color: Color);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoRenderer;

impl Renderer for NoRenderer {
    fn render(&mut self, _: Vec2, _: Color) {}
}

pub mod literal;
pub mod register;

pub use literal::U8_LITERAL;

use super::super::Context;

pub trait U8Source {
    fn read(&self, context: &mut dyn Context) -> u8;
}

pub type U8Writer = Box<dyn Fn(&mut dyn Context, u8)>;

pub trait U8Destination {
    fn writer(&self, context: &mut dyn Context) -> U8Writer;
}

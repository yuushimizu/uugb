use crate::cpu::Context;

pub type U8Writer = Box<dyn Fn(&mut dyn Context, u8)>;

pub trait U8Destination {
    fn writer(&self, context: &mut dyn Context) -> U8Writer;
}

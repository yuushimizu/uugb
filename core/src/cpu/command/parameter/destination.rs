use crate::cpu::Context;

pub type Writer8 = Box<dyn Fn(&mut dyn Context, u8)>;

pub trait Destination8 {
    fn writer(&self, context: &mut dyn Context) -> Writer8;
}

pub type Writer16 = Box<dyn Fn(&mut dyn Context, u16)>;

pub trait Destination16 {
    fn writer(&self, context: &mut dyn Context) -> Writer16;
}

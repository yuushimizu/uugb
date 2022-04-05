use crate::cpu::Context;

pub trait Source8 {
    fn read(&self, context: &mut dyn Context) -> u8;
}

pub trait Source16 {
    fn read(&self, context: &mut dyn Context) -> u16;
}

use crate::cpu::Context;

pub trait U8Source {
    fn read(&self, context: &mut dyn Context) -> u8;
}

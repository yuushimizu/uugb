use crate::cpu::Context;

pub trait Source<T: Copy> {
    fn read(&self, context: &mut dyn Context) -> T;
}

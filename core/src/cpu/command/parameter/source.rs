use crate::cpu::Context;

pub trait Source<T: Copy>
where
    Self: 'static,
{
    fn read(&self, context: &mut dyn Context) -> T;
}

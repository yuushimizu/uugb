use crate::cpu::Context;

pub type Writer<T> = Box<dyn Fn(&mut dyn Context, T)>;

pub trait Destination<T: Copy>
where
    Self: 'static,
{
    fn writer(&self, context: &mut dyn Context) -> Writer<T>;
}

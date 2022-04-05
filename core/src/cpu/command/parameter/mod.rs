pub mod indirection;
pub mod literal;
pub mod register;
pub mod stack_pointer;

pub use literal::LITERAL;

use crate::cpu::Context;

pub trait Read<T: Copy>
where
    Self: 'static,
{
    fn read(&self, context: &mut dyn Context) -> T;
}

pub type ReadRef<T> = &'static dyn Read<T>;

pub type Writer<T> = Box<dyn Fn(&mut dyn Context, T)>;

pub trait Write<T: Copy>
where
    Self: 'static,
{
    fn writer(&self, context: &mut dyn Context) -> Writer<T>;
}

pub type WriteRef<T> = &'static dyn Write<T>;

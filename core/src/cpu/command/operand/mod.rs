pub mod indirection;
pub mod literal;
pub mod register;
pub mod stack_pointer;

pub use indirection::{Indirection, IndirectionRef};
pub use literal::LITERAL;
pub use register::{Register, RegisterRef};

use crate::cpu::Context;

pub trait Value: 'static + Sized + Copy {}

impl<T: 'static + Sized + Copy> Value for T {}

pub trait Read<T: Value> {
    fn read(&self, context: &mut dyn Context) -> T;

    fn as_read(&self) -> &dyn Read<T>;
}

pub type ReadRef<T> = &'static dyn Read<T>;

pub type Writer<T> = Box<dyn Fn(&mut dyn Context, T)>;

pub trait Write<T: Value> {
    fn writer(&self, context: &mut dyn Context) -> Writer<T>;

    fn as_write(&self) -> &dyn Write<T>;
}

pub type WriteRef<T> = &'static dyn Write<T>;

pub trait ReadWrite<T: Value>: Read<T> + Write<T> {
    fn read_write(&self, context: &mut dyn Context) -> (T, Writer<T>);

    fn as_read_write(&self) -> &dyn ReadWrite<T>;
}

pub type ReadWriteRef<T> = &'static dyn ReadWrite<T>;

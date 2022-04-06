pub mod indirection;
pub mod literal;
pub mod register;
pub mod stack_pointer;

pub use indirection::Indirection;
pub use literal::LITERAL;
pub use register::Register;

use crate::cpu::Context;

pub trait Value: 'static + Sized + Copy {}

impl<T: 'static + Sized + Copy> Value for T {}

pub trait Readable<T: Value> {
    fn read(&self, context: &mut dyn Context) -> T;

    fn as_read(&self) -> &dyn Readable<T>;
}

pub type Read<T> = &'static dyn Readable<T>;

pub type Writer<T> = Box<dyn Fn(&mut dyn Context, T)>;

pub trait Writable<T: Value> {
    fn writer(&self, context: &mut dyn Context) -> Writer<T>;

    fn as_write(&self) -> &dyn Writable<T>;
}

pub type Write<T> = &'static dyn Writable<T>;

pub trait ReadWritable<T: Value>: Readable<T> + Writable<T> {
    fn read_write(&self, context: &mut dyn Context) -> (T, Writer<T>);

    fn as_read_write(&self) -> &dyn ReadWritable<T>;
}

pub type ReadWrite<T> = &'static dyn ReadWritable<T>;

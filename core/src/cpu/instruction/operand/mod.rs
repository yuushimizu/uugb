pub mod indirection;
pub mod literal;
pub mod opcode_register;
pub mod register;
pub mod stack_pointer;

pub use indirection::Indirection;
pub use literal::LITERAL;
pub use opcode_register::OpcodeRegister;
pub use register::Register;

use crate::cpu::Context;
use std::fmt;

pub trait Operand: 'static + Copy + fmt::Display + fmt::Debug {}

pub trait Value: 'static + Sized + Copy {}

impl<T: 'static + Sized + Copy> Value for T {}

pub trait Read<T: Value>: Operand {
    fn read(self, context: &mut dyn Context) -> T;
}

pub type Writer<T> = Box<dyn Fn(&mut dyn Context, T)>;

pub trait Write<T: Value>: Operand {
    fn writer(self, context: &mut dyn Context) -> Writer<T>;
}

pub trait ReadWrite<T: Value>: Operand {
    fn read_write(self, context: &mut dyn Context) -> (T, Writer<T>);
}

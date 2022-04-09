pub mod condition;
pub mod indirection;
pub mod literal;
pub mod opcode_register;
pub mod register;
pub mod stack_pointer;

pub use condition::Condition;
pub use indirection::Indirection;
pub use literal::LITERAL;
pub use opcode_register::OpcodeRegister;
pub use register::Register;

use crate::cpu::CpuContext;
use std::fmt;

pub trait Operand: 'static + Sync + Send + Copy + fmt::Display + fmt::Debug {}

pub trait Value: 'static + Sized + Copy {}

impl<T: 'static + Sized + Copy> Value for T {}

pub trait Read<T: Value>: Operand {
    fn read(&self, context: &mut CpuContext) -> T;
}

pub trait Write<T: Value>: Operand {
    fn write(&self, context: &mut CpuContext, value: T);
}

pub mod indirection;
pub mod literal;
pub mod opcode_register;
pub mod register;
pub mod stack_pointer;

pub use indirection::Indirection;
pub use literal::LITERAL;
pub use opcode_register::OpcodeRegister;
pub use register::Register;

use crate::cpu::{Continuation, CpuContext};
use std::fmt;

pub trait Operand: 'static + Sync + Send + Copy + fmt::Display + fmt::Debug {}

pub trait Value: 'static + Sized + Copy {}

impl<T: 'static + Sized + Copy> Value for T {}

pub trait Read<T: Value>: Operand {
    fn read(&self, context: &mut dyn CpuContext) -> Continuation<T>;
}

pub struct Writer<T: Value> {
    write: Box<dyn FnOnce(&mut dyn CpuContext, T) -> Continuation<()>>,
}

impl<T: Value> Writer<T> {
    pub fn write(self, context: &mut dyn CpuContext, value: T) -> Continuation<()> {
        (self.write)(context, value)
    }

    pub fn new<F: FnOnce(&mut dyn CpuContext, T) -> Continuation<()> + 'static>(f: F) -> Self {
        Self { write: Box::new(f) }
    }

    pub fn just<F: FnOnce(&mut dyn CpuContext, T) + 'static>(f: F) -> Self {
        Self::new(move |context, value| {
            f(context, value);
            Continuation::just(())
        })
    }
}

pub trait Write<T: Value>: Operand {
    fn prepare(&self, context: &mut dyn CpuContext) -> Continuation<Writer<T>>;
}

pub trait ReadWrite<T: Value>: Operand {
    fn prepare_and_read(&self, context: &mut dyn CpuContext) -> Continuation<(T, Writer<T>)>;
}

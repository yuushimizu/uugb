pub mod arithmetic;
pub mod ld;
pub mod logic;
pub mod stack;

pub use arithmetic::*;
pub use ld::*;
pub use logic::*;
pub use stack::*;

use crate::cpu::Context;
use std::fmt;

pub struct Operator {
    mnemonic: &'static str,
    execute: Box<dyn Fn(&mut dyn Context)>,
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Operator")
            .field("mnemonic", &self.mnemonic)
            .finish()
    }
}

impl Operator {
    pub fn new<E>(mnemonic: &'static str, execute: E) -> Self
    where
        E: Fn(&mut dyn Context) + 'static,
    {
        Self {
            mnemonic,
            execute: Box::new(execute),
        }
    }

    pub fn execute(&self, context: &mut dyn Context) {
        (self.execute)(context);
    }
}

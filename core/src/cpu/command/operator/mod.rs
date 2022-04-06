pub mod arithmetic;
pub mod bits;
pub mod call;
pub mod cpu_state;
pub mod format;
pub mod jump;
pub mod load;
pub mod logic;
pub mod miscellaneous;
pub mod stack;

pub use arithmetic::*;
pub use bits::*;
pub use call::*;
pub use cpu_state::*;
pub use jump::*;
pub use load::*;
pub use logic::*;
pub use miscellaneous::*;
pub use stack::*;

use crate::cpu::Context;
//use format::Format;
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

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.mnemonic)
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

pub mod arithmetic;
pub mod bits;
pub mod call;
pub mod cpu_state;
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

use crate::cpu::instruction::Context;
use std::fmt;

pub struct Operator {
    execute: Box<dyn Fn(&mut Context) + Sync + Send>,
    debug: Box<dyn Fn(&Context) -> String + Sync + Send>,
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Operator").finish()
    }
}

impl Operator {
    pub fn new(
        execute: impl Fn(&mut Context) + Sync + Send + 'static,
        debug: impl Fn(&Context) -> String + Sync + Send + 'static,
    ) -> Self {
        Self {
            execute: Box::new(execute),
            debug: Box::new(debug),
        }
    }

    pub fn execute(&self, context: &mut Context) {
        (self.execute)(context);
    }

    pub fn debug(&self, context: &Context) -> String {
        (self.debug)(context)
    }
}

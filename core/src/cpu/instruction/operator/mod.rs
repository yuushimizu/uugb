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

use crate::cpu::{Continuation, CpuContext};
use std::fmt;

pub struct Operator {
    format: String,
    execute: Box<dyn Fn(&mut dyn CpuContext) -> Continuation<()> + Sync + Send>,
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Operator")
            .field("format", &self.format)
            .finish()
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format)
    }
}

impl Operator {
    pub fn new<E: Fn(&mut dyn CpuContext) -> Continuation<()> + Sync + Send + 'static>(
        format: String,
        execute: E,
    ) -> Self {
        Self {
            format,
            execute: Box::new(execute),
        }
    }

    pub fn execute(&self, context: &mut dyn CpuContext) -> Continuation<()> {
        (self.execute)(context)
    }
}

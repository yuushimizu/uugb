use super::{DebugOperand, Operand, Read};
use crate::cpu::{instruction::Context, registers::Flags};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Condition {
    Nz,
    Z,
    Nc,
    C,
}

impl Condition {
    fn is_satisfied(&self, flags: &Flags) -> bool {
        use Condition::*;
        match self {
            Nz => !flags.z,
            Z => flags.z,
            Nc => !flags.c,
            C => flags.c,
        }
    }
}

impl Operand for Condition {}

impl Read<bool> for Condition {
    fn read(&self, context: &mut Context) -> bool {
        self.is_satisfied(context.flags())
    }
}

impl DebugOperand<bool> for Condition {
    fn debug(&self, context: &Context) -> String {
        format!(
            "{}={}",
            format!("{:?}", self).to_uppercase(),
            self.is_satisfied(context.flags())
        )
    }
}

pub use Condition::*;

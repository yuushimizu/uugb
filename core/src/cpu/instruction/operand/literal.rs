use super::{Operand, Read};
use crate::cpu::CpuContext;
use std::fmt;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Literal;

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "#")
    }
}

impl Operand for Literal {}

impl Read<u8> for Literal {
    fn read(self, context: &mut dyn CpuContext) -> u8 {
        context.fetch()
    }
}

impl Read<u16> for Literal {
    fn read(self, context: &mut dyn CpuContext) -> u16 {
        context.fetch16()
    }
}

pub const LITERAL: Literal = Literal;

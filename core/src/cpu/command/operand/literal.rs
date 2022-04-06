use super::{Operand, Read};
use crate::cpu::Context;
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
    fn read(&self, context: &mut dyn Context) -> u8 {
        context.fetch_pc()
    }
}

impl Read<u16> for Literal {
    fn read(&self, context: &mut dyn Context) -> u16 {
        context.fetch16_pc()
    }
}

pub const LITERAL: Literal = Literal;

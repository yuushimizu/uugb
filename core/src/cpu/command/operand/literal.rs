use super::Read;
use crate::cpu::Context;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Literal;

impl Read<u8> for Literal {
    fn read(&self, context: &mut dyn Context) -> u8 {
        context.pop_from_pc()
    }
}

impl Read<u16> for Literal {
    fn read(&self, context: &mut dyn Context) -> u16 {
        context.pop16_from_pc()
    }
}

pub const LITERAL: &Literal = &Literal;

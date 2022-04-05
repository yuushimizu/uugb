use super::Read;
use crate::cpu::Context;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Literal;

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

pub const LITERAL: &Literal = &Literal;

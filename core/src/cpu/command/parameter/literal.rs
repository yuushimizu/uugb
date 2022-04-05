use super::U8Source;
use crate::cpu::Context;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct U8Literal;

impl U8Source for U8Literal {
    fn read(&self, context: &mut dyn Context) -> u8 {
        context.pop_from_pc()
    }
}

pub const U8_LITERAL: &U8Literal = &U8Literal;

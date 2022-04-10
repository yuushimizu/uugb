use super::{Operand, Read};
use crate::cpu::instruction::Context;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Literal;

impl Operand for Literal {}

impl Read<u8> for Literal {
    fn read(&self, context: &mut Context) -> u8 {
        context.fetch()
    }

    fn debug(&self, context: &Context) -> String {
        format!("#{:02X}", context.debug_u8(context.registers().pc))
    }
}

impl Read<u16> for Literal {
    fn read(&self, context: &mut Context) -> u16 {
        context.fetch16()
    }

    fn debug(&self, context: &Context) -> String {
        format!("#{:04X}", context.debug_u16(context.registers().pc))
    }
}

pub const LITERAL: Literal = Literal;

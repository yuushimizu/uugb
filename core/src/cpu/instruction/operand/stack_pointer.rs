use super::{Operand, Read};
use crate::cpu::instruction::Context;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct AddLiteral8;

impl Read<u16> for AddLiteral8 {
    fn read(&self, context: &mut Context) -> u16 {
        let value = context.fetch();
        context.add_sp(value)
    }

    fn debug(&self, context: &Context) -> String {
        format!(
            "SP:{:04X}+#{:02X}",
            context.registers().sp,
            context.debug_u8(context.registers().pc)
        )
    }
}

impl Operand for AddLiteral8 {}

pub const ADD_LITERAL_8: AddLiteral8 = AddLiteral8;

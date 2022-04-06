use super::{Operand, Read};
use crate::cpu::Context;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct AddLiteral8;

impl Read<u16> for AddLiteral8 {
    fn read(&self, context: &mut dyn Context) -> u16 {
        let value = context.fetch_pc();
        context.add_sp(value)
    }
}

impl fmt::Display for AddLiteral8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "SP+n")
    }
}

impl Operand for AddLiteral8 {}

pub const ADD_LITERAL_8: AddLiteral8 = AddLiteral8;

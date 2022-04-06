use super::Readable;
use crate::cpu::Context;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Literal;

impl Readable<u8> for Literal {
    fn read(&self, context: &mut dyn Context) -> u8 {
        context.fetch_pc()
    }

    fn as_read(&self) -> &dyn Readable<u8> {
        self
    }
}

impl Readable<u16> for Literal {
    fn read(&self, context: &mut dyn Context) -> u16 {
        context.fetch16_pc()
    }

    fn as_read(&self) -> &dyn Readable<u16> {
        self
    }
}

pub const LITERAL: &Literal = &Literal;

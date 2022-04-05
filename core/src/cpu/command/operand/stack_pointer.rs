use super::Read;
use crate::cpu::Context;

#[derive(Debug, Clone)]
pub struct AddLiteral8;

impl Read<u16> for AddLiteral8 {
    fn read(&self, context: &mut dyn Context) -> u16 {
        let value = context.fetch_pc();
        context.add_sp(value)
    }
}

pub const ADD_LITERAL_8: &AddLiteral8 = &AddLiteral8;

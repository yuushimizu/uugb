use super::Source;
use crate::cpu::Context;

#[derive(Debug, Clone)]
pub struct AddLiteral8;

impl Source<u16> for AddLiteral8 {
    fn read(&self, context: &mut dyn Context) -> u16 {
        let sp = context.registers().sp;
        let n = context.pop_from_pc() as i8 as u16;
        let flags = &mut context.registers_mut().f;
        flags.z = false;
        flags.n = false;
        flags.h = ((sp & 0xF) + (n & 0xF)) & 0x10 != 0;
        flags.c = ((sp & 0xFF) + (n & 0xFF)) & 0x100 != 0;
        sp.wrapping_add(n)
    }
}

pub const ADD_LITERAL_8: &AddLiteral8 = &AddLiteral8;

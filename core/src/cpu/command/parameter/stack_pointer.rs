use super::Read;
use crate::cpu::{registers::Flags, Context};

#[derive(Debug, Clone)]
pub struct AddLiteral8;

impl Read<u16> for AddLiteral8 {
    fn read(&self, context: &mut dyn Context) -> u16 {
        let sp = context.registers().sp;
        let n = context.pop_from_pc() as i8 as u16;
        context.registers_mut().f = Flags {
            z: false,
            n: false,
            h: ((sp & 0xF) + (n & 0xF)) & 0x10 != 0,
            c: ((sp & 0xFF) + (n & 0xFF)) & 0x100 != 0,
        };
        sp.wrapping_add(n)
    }
}

pub const ADD_LITERAL_8: &AddLiteral8 = &AddLiteral8;

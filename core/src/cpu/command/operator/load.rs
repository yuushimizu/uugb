use super::Operator;
use crate::cpu::command::operand::{Read, Write};

fn ld_generic<T: Copy>(destination: Write<T>, source: Read<T>) -> Operator {
    Operator::new("LD", |context| {
        let writer = destination.writer(context);
        let value = source.read(context);
        writer(context, value);
    })
}

pub fn ld(destination: Write<u8>, source: Read<u8>) -> Operator {
    ld_generic(destination, source)
}

pub fn ld16(destination: Write<u16>, source: Read<u16>) -> Operator {
    ld_generic(destination, source)
}

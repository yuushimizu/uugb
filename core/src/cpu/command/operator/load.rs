use super::Operator;
use crate::cpu::command::operand::{Read, Value, Write};

fn ld_generic<T: Value, D: Write<T>, S: Read<T>>(destination: D, source: S) -> Operator {
    Operator::new("LD", move |context| {
        let writer = destination.writer(context);
        let value = source.read(context);
        writer(context, value);
    })
}

pub fn ld<D: Write<u8>, S: Read<u8>>(destination: D, source: S) -> Operator {
    ld_generic(destination, source)
}

pub fn ld16<D: Write<u16>, S: Read<u16>>(destination: D, source: S) -> Operator {
    ld_generic(destination, source)
}

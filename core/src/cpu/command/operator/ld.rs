use super::Operator;
use crate::cpu::command::operand::{ReadRef, WriteRef};

fn ld_generic<T: Copy>(destination: WriteRef<T>, source: ReadRef<T>) -> Operator {
    Operator::new(
        "LD",
        Box::new(|context| {
            let writer = destination.writer(context);
            let value = source.read(context);
            writer(context, value);
        }),
    )
}

pub fn ld(destination: WriteRef<u8>, source: ReadRef<u8>) -> Operator {
    ld_generic(destination, source)
}

pub fn ld16(destination: WriteRef<u16>, source: ReadRef<u16>) -> Operator {
    ld_generic(destination, source)
}

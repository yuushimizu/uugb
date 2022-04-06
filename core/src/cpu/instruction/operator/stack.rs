use super::Operator;
use crate::cpu::instruction::operand::{Read, Write};

pub fn push<S: Read<u16>>(source: S) -> Operator {
    Operator::new("PUSH", move |context| {
        let value = source.read(context);
        context.push16_sp(value);
    })
}

pub fn pop<D: Write<u16>>(destination: D) -> Operator {
    Operator::new("POP", move |context| {
        let writer = destination.writer(context);
        let value = context.pop16_sp();
        writer(context, value);
    })
}

pub fn add_sp<R: Read<u8>>(rhs: R) -> Operator {
    Operator::new("ADD", move |context| {
        let value = rhs.read(context);
        context.registers_mut().sp = context.add_sp(value)
    })
}

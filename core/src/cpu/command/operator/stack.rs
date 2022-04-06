use super::Operator;
use crate::cpu::command::operand::{Read, Write};

pub fn push(source: Read<u16>) -> Operator {
    Operator::new("PUSH", |context| {
        let value = source.read(context);
        context.push16_sp(value);
    })
}

pub fn pop(destination: Write<u16>) -> Operator {
    Operator::new("POP", |context| {
        let writer = destination.writer(context);
        let value = context.pop16_sp();
        writer(context, value);
    })
}

pub fn add_sp(rhs: Read<u8>) -> Operator {
    Operator::new("ADD", |context| {
        let value = rhs.read(context);
        context.registers_mut().sp = context.add_sp(value)
    })
}

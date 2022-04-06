use super::Operator;
use crate::cpu::command::operand::{Read, Write};

pub fn push(source: Read<u16>) -> Operator {
    Operator::new("PUSH", |context| {
        let value = source.read(context);
        let address = context.registers().sp;
        context.write16(address, value);
        context.registers_mut().sp = address.wrapping_sub(2);
    })
}

pub fn pop(destination: Write<u16>) -> Operator {
    Operator::new("POP", |context| {
        let writer = destination.writer(context);
        let address = context.registers().sp;
        let value = context.read16(address);
        writer(context, value);
        context.registers_mut().sp = address.wrapping_add(2);
    })
}

pub fn add_sp(rhs: Read<u8>) -> Operator {
    Operator::new("ADD", |context| {
        let value = rhs.read(context);
        context.registers_mut().sp = context.add_sp(value)
    })
}

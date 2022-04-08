use super::Operator;
use crate::cpu::instruction::operand::{Read, Write};

pub fn push(source: impl Read<u16>) -> Operator {
    Operator::new(format!("PUSH {}", source), move |context| {
        let value = source.read(context);
        context.push16(value);
        context.wait();
    })
}

pub fn pop(destination: impl Write<u16>) -> Operator {
    Operator::new(format!("POP {}", destination), move |context| {
        let writer = destination.prepare(context);
        let value = context.pop16();
        writer.write(context, value);
    })
}

pub fn add_sp(rhs: impl Read<u8>) -> Operator {
    Operator::new(format!("ADD SP, {}", rhs), move |context| {
        let value = rhs.read(context);
        context.registers_mut().sp = context.add_sp(value);
        context.wait();
    })
}

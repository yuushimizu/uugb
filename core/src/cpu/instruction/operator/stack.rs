use super::Operator;
use crate::cpu::instruction::operand::{Read, Write};

pub fn push<S: Read<u16>>(source: S) -> Operator {
    Operator::new(format!("PUSH {}", source), move |context| {
        let value = source.read(context);
        context.push16(value);
        context.wait();
    })
}

pub fn pop<D: Write<u16>>(destination: D) -> Operator {
    Operator::new(format!("POP {}", destination), move |context| {
        let writer = destination.prepare(context);
        let value = context.pop16();
        writer.write(context, value);
    })
}

pub fn add_sp<R: Read<u8>>(rhs: R) -> Operator {
    Operator::new(format!("ADD SP, {}", rhs), move |context| {
        let value = rhs.read(context);
        context.registers_mut().sp = context.add_sp(value);
        context.wait();
    })
}

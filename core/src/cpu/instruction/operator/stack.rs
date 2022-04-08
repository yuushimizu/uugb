use super::Operator;
use crate::cpu::instruction::operand::{Read, Write};

pub fn push<S: Read<u16>>(source: S) -> Operator {
    Operator::new(format!("PUSH {}", source), move |context| {
        source
            .read(context)
            .then(|context, value| context.push16(value).tick())
    })
}

pub fn pop<D: Write<u16>>(destination: D) -> Operator {
    Operator::new(format!("POP {}", destination), move |context| {
        destination.prepare(context).then(|context, writer| {
            context
                .pop16()
                .then(|context, value| writer.write(context, value))
        })
    })
}

pub fn add_sp<R: Read<u8>>(rhs: R) -> Operator {
    Operator::new(format!("ADD SP, {}", rhs), move |context| {
        rhs.read(context).then(|context, value| {
            context
                .add_sp(value)
                .map(|context, result| context.registers_mut().sp = result)
                .tick()
        })
    })
}

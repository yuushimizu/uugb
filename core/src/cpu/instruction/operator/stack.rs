use super::Operator;
use crate::cpu::instruction::operand::{register, DebugOperand, Read, Write};

pub fn push(source: impl Read<u16>) -> Operator {
    Operator::new(
        move |context| {
            let value = source.read(context);
            context.push16(value);
            context.wait();
        },
        move |context| format!("PUSH {}", source.debug(context)),
    )
}

pub fn pop(destination: impl Write<u16>) -> Operator {
    Operator::new(
        move |context| {
            let value = context.pop16();
            destination.write(context, value);
        },
        move |context| {
            format!(
                "POP {} [(SP)={:04X}]",
                destination.debug(context),
                context.debug_u16(context.registers().sp)
            )
        },
    )
}

pub fn add_sp(rhs: impl Read<u8>) -> Operator {
    Operator::new(
        move |context| {
            let value = rhs.read(context);
            context.registers_mut().sp = context.add_sp(value);
            context.wait();
        },
        move |context| {
            format!(
                "ADD {}, {}",
                register::Sp.debug(context),
                rhs.debug(context)
            )
        },
    )
}

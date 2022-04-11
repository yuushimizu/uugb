use super::Operator;
use crate::cpu::instruction::operand::{Condition, DebugOperand, Read};

pub fn call(address: impl Read<u16>) -> Operator {
    Operator::new(
        move |context| {
            let address = address.read(context);
            context.call(address);
        },
        move |context| {
            format!(
                "CALL {} [PC={:04X}]",
                address.debug(context),
                context.registers().pc + 2
            )
        },
    )
}

pub fn call_cc(condition: Condition, address: impl Read<u16>) -> Operator {
    Operator::new(
        move |context| {
            let address = address.read(context);
            if condition.read(context) {
                context.call(address);
            }
        },
        move |context| {
            format!(
                "CALL {}, {} [PC={:04X}]",
                condition.debug(context),
                address.debug(context),
                context.registers().pc + 2
            )
        },
    )
}

pub fn rst(address: u8) -> Operator {
    Operator::new(
        move |context| context.call(address as u16),
        move |_| format!("RST {:02X}", address),
    )
}

pub fn ret() -> Operator {
    Operator::new(
        |context| context.ret(),
        |context| {
            format!(
                "RET [(SP)={:04X}]",
                context.debug_u16(context.registers().sp)
            )
        },
    )
}

pub fn ret_cc(condition: Condition) -> Operator {
    Operator::new(
        move |context| {
            if condition.read(context) {
                context.ret();
            }
            context.wait();
        },
        move |context| {
            format!(
                "RET {} [(SP)={:04X}]",
                condition.debug(context),
                context.debug_u16(context.registers().sp),
            )
        },
    )
}

pub fn reti() -> Operator {
    Operator::new(
        move |context| {
            context.enable_interrupts();
            context.ret();
        },
        |context| {
            format!(
                "RETI [(SP)={:04X}]",
                context.debug_u16(context.registers().sp),
            )
        },
    )
}

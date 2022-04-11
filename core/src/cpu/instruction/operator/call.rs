use super::Operator;
use crate::cpu::instruction::operand::{Condition, Read};

pub fn call(address: impl Read<u16>) -> Operator {
    Operator::new(
        move |context| {
            let address = address.read(context);
            context.call(address);
        },
        move |context| format!("CALL {}", address.debug(context)),
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
                "CALL {}, {}",
                condition.debug(context),
                address.debug(context),
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
        |context| format!("RET:{:04X}", context.debug_u16(context.registers().sp)),
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
                "RET:{:04X} {}",
                context.debug_u16(context.registers().sp),
                condition.debug(context)
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
        |context| format!("RETI:{:04X}", context.debug_u16(context.registers().sp),),
    )
}

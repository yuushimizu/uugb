use super::Operator;
use crate::cpu::instruction::{
    operand::{self, register, Condition, DebugOperand, Read},
    Context,
};

pub fn jp_nn() -> Operator {
    Operator::new(
        move |context| {
            let address = context.fetch16();
            context.jump(address);
            context.wait();
        },
        |context| {
            format!(
                "JP {}",
                DebugOperand::<u16>::debug(&operand::LITERAL, context)
            )
        },
    )
}

pub fn jp_hl() -> Operator {
    Operator::new(
        move |context| {
            context.jump(context.registers().hl());
        },
        |context| format!("JP {}", register::Hl.debug(context)),
    )
}

pub fn jp_cc(condition: Condition, address: impl Read<u16>) -> Operator {
    Operator::new(
        move |context| {
            let address = address.read(context);
            if condition.read(context) {
                context.jump(address);
                context.wait();
            }
        },
        move |context| {
            format!(
                "JP {}, {}",
                condition.debug(context),
                address.debug(context)
            )
        },
    )
}

fn relative_jump(context: &mut Context, offset: u8) {
    context.jump(context.registers().pc.wrapping_add(offset as i8 as u16));
}

pub fn jr(operand: impl Read<u8>) -> Operator {
    Operator::new(
        move |context| {
            let offset = operand.read(context);
            relative_jump(context, offset);
            context.wait();
        },
        move |context| format!("JR {}", operand.debug(context)),
    )
}

pub fn jr_cc(condition: Condition, operand: impl Read<u8>) -> Operator {
    Operator::new(
        move |context| {
            let offset = operand.read(context);
            if condition.read(context) {
                relative_jump(context, offset);
                context.wait();
            }
        },
        move |context| {
            format!(
                "JR {}, {}",
                condition.debug(context),
                operand.debug(context)
            )
        },
    )
}

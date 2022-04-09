use super::Operator;
use crate::cpu::instruction::{
    operand::{Condition, Read},
    Context,
};

pub fn jp_nn() -> Operator {
    Operator::new("JP #".into(), move |context| {
        let address = context.fetch16();
        context.jump(address);
        context.wait();
    })
}

pub fn jp_hl() -> Operator {
    Operator::new("JP HL".into(), move |context| {
        context.jump(context.registers().hl());
    })
}

pub fn jp_cc(condition: Condition, address: impl Read<u16>) -> Operator {
    Operator::new(format!("JP {}, {}", condition, address), move |context| {
        let address = address.read(context);
        if condition.is_satisfied(context) {
            context.jump(address);
            context.wait();
        }
    })
}

fn relative_jump(context: &mut Context, offset: u8) {
    context.jump(context.registers().pc.wrapping_add(offset as i8 as u16));
}

pub fn jr(operand: impl Read<u8>) -> Operator {
    Operator::new(format!("JR {}", operand), move |context| {
        let offset = operand.read(context);
        relative_jump(context, offset);
        context.wait();
    })
}

pub fn jr_cc(condition: Condition, operand: impl Read<u8>) -> Operator {
    Operator::new(format!("JR {}, {}", condition, operand), move |context| {
        let offset = operand.read(context);
        if condition.is_satisfied(context) {
            relative_jump(context, offset);
            context.wait();
        }
    })
}

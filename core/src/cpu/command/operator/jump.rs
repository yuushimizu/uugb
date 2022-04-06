use super::Operator;
use crate::cpu::command::operand::ReadRef;

pub mod condition {
    use crate::cpu::registers::Flags;

    pub type Condition = fn(Flags) -> bool;

    pub const NZ: Condition = |flags| !flags.z;

    pub const Z: Condition = |flags| flags.z;

    pub const NC: Condition = |flags| !flags.c;

    pub const C: Condition = |flags| flags.c;
}

pub use condition::Condition;

pub fn jp(operand: ReadRef<u16>) -> Operator {
    Operator::new("JP", |context| {
        context.registers_mut().pc = operand.read(context);
    })
}

pub fn jp_cc(condition: Condition, location: ReadRef<u16>) -> Operator {
    Operator::new("JP", move |context| {
        let address = location.read(context);
        if condition(context.flags()) {
            context.registers_mut().pc = address;
        }
    })
}

pub fn jr(operand: ReadRef<u8>) -> Operator {
    Operator::new("JR", |context| {
        let offset = operand.read(context);
        context.registers_mut().pc = context.registers().pc.wrapping_add(offset as i8 as u16)
    })
}

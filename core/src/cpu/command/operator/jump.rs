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

pub fn jp_cc(condition: Condition, location: ReadRef<u16>) -> Operator {
    Operator::new("JP", move |context| {
        let address = location.read(context);
        if condition(context.flags()) {
            context.registers_mut().pc = address;
        }
    })
}

pub fn jp(location: ReadRef<u16>) -> Operator {
    jp_cc(|_| true, location)
}

pub fn jr_cc(condition: Condition, operand: ReadRef<u8>) -> Operator {
    Operator::new("JR", move |context| {
        let offset = operand.read(context);
        if condition(context.flags()) {
            context.registers_mut().pc = context.registers().pc.wrapping_add(offset as i8 as u16);
        }
    })
}

pub fn jr(operand: ReadRef<u8>) -> Operator {
    jr_cc(|_| true, operand)
}

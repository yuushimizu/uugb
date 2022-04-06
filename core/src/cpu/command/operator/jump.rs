use super::Operator;
use crate::cpu::command::operand::Read;

pub mod condition {
    use crate::cpu::registers::Flags;

    pub type Condition = fn(Flags) -> bool;

    pub const NZ: Condition = |flags| !flags.z;

    pub const Z: Condition = |flags| flags.z;

    pub const NC: Condition = |flags| !flags.c;

    pub const C: Condition = |flags| flags.c;
}

pub use condition::Condition;

pub fn jp_cc(condition: Condition, location: Read<u16>) -> Operator {
    Operator::new("JP", move |context| {
        let address = location.read(context);
        if condition(context.flags()) {
            context.jump(address);
        }
    })
}

pub fn jp(location: Read<u16>) -> Operator {
    jp_cc(|_| true, location)
}

pub fn jr_cc(condition: Condition, operand: Read<u8>) -> Operator {
    Operator::new("JR", move |context| {
        let offset = operand.read(context);
        if condition(context.flags()) {
            context.jump(context.registers().pc.wrapping_add(offset as i8 as u16));
        }
    })
}

pub fn jr(operand: Read<u8>) -> Operator {
    jr_cc(|_| true, operand)
}

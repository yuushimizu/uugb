use super::Operator;
use crate::cpu::instruction::operand::Read;

pub mod condition {
    use crate::cpu::registers::Flags;

    pub type Condition = fn(Flags) -> bool;

    pub const NZ: Condition = |flags| !flags.z;

    pub const Z: Condition = |flags| flags.z;

    pub const NC: Condition = |flags| !flags.c;

    pub const C: Condition = |flags| flags.c;
}

pub use condition::Condition;

pub fn jp_cc<L: Read<u16>>(condition: Condition, location: L) -> Operator {
    Operator::new("JP", move |context| {
        let address = location.read(context);
        if condition(context.flags()) {
            context.jump(address);
        }
    })
}

pub fn jp<L: Read<u16>>(location: L) -> Operator {
    jp_cc(|_| true, location)
}

pub fn jr_cc<O: Read<u8>>(condition: Condition, operand: O) -> Operator {
    Operator::new("JR", move |context| {
        let offset = operand.read(context);
        if condition(context.flags()) {
            context.jump(context.registers().pc.wrapping_add(offset as i8 as u16));
        }
    })
}

pub fn jr<O: Read<u8>>(operand: O) -> Operator {
    jr_cc(|_| true, operand)
}

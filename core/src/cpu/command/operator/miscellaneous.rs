use super::Operator;
use crate::cpu::{command::operand::ReadWriteRef, registers::Flags};

pub fn swap(operand: ReadWriteRef<u8>) -> Operator {
    Operator::new("SWAP", |context| {
        let (current, writer) = operand.read_and_writer(context);
        let result = current.rotate_left(4);
        writer(context, result);
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: false,
            c: false,
        });
    })
}

pub fn daa() -> Operator {
    Operator::new("DAA", |context| {
        let flags = context.flags();
        let current = context.registers().a;
        let adjust_higher = flags.c || (!flags.n && current > 0x99);
        let adjust_lower = flags.h || (!flags.n && current & 0xF > 0x9);
        let adjust_value =
            if adjust_higher { 0x60 } else { 0 } | if adjust_lower { 0x06 } else { 0 };
        let result = if !flags.n {
            current.wrapping_add(adjust_value)
        } else {
            current.wrapping_sub(adjust_value)
        };
        context.registers_mut().a = result;
        context.set_flags(Flags {
            z: result == 0,
            h: false,
            c: adjust_higher,
            ..flags
        });
    })
}

pub fn cpl() -> Operator {
    Operator::new("CPL", |context| {
        context.registers_mut().a = !context.registers().a;
        context.set_flags(Flags {
            n: true,
            h: true,
            ..context.flags()
        });
    })
}

pub fn ccf() -> Operator {
    Operator::new("CCF", |context| {
        context.set_flags(Flags {
            n: false,
            h: false,
            c: !context.registers().f.c,
            ..context.flags()
        });
    })
}

pub fn scf() -> Operator {
    Operator::new("SCF", |context| {
        context.set_flags(Flags {
            n: false,
            h: false,
            c: true,
            ..context.flags()
        });
    })
}

pub fn nop() -> Operator {
    Operator::new("NOP", |_context| {})
}

use super::Operator;
use crate::cpu::{command::operand::ReadWriteRef, registers::Flags};

pub fn swap(operand: ReadWriteRef<u8>) -> Operator {
    Operator::new("SWAP", |context| {
        let (current, writer) = operand.read_and_writer(context);
        let result = current.rotate_left(4);
        writer(context, result);
        context.registers_mut().f = Flags {
            z: result == 0,
            n: false,
            h: false,
            c: false,
        }
    })
}

pub fn daa() -> Operator {
    Operator::new("DAA", |context| {
        let flags = context.registers().f.clone();
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
        context.registers_mut().f = Flags {
            z: result == 0,
            h: false,
            c: adjust_higher,
            ..flags
        };
    })
}

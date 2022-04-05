use super::Operator;
use crate::cpu::{
    command::operand::{register, ReadWriteRef},
    registers::Flags,
};
use crate::util::bits::Bits;

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

fn rlc_u8(mnemonic: &'static str, operand: ReadWriteRef<u8>) -> Operator {
    Operator::new(mnemonic, |context| {
        let (current, writer) = operand.read_and_writer(context);
        let result = current.rotate_left(1);
        writer(context, result);
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: false,
            c: current.bit(7),
        })
    })
}

pub fn rlca() -> Operator {
    rlc_u8("RLCA", register::A)
}

fn rl_u8(mnemonic: &'static str, operand: ReadWriteRef<u8>) -> Operator {
    Operator::new(mnemonic, |context| {
        let (current, writer) = operand.read_and_writer(context);
        let result = current << 1 | (context.flags().c as u8);
        writer(context, result);
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: false,
            c: current.bit(7),
        })
    })
}

pub fn rla() -> Operator {
    rl_u8("RLA", register::A)
}

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

fn rl_u8(mnemonic: &'static str, operand: ReadWriteRef<u8>, with_carry: bool) -> Operator {
    Operator::new(mnemonic, move |context| {
        let (current, writer) = operand.read_and_writer(context);
        let result = if with_carry {
            current << 1 | context.flags().c as u8
        } else {
            current.rotate_left(1)
        };
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
    rl_u8("RLCA", register::A, false)
}

pub fn rla() -> Operator {
    rl_u8("RLA", register::A, true)
}

pub fn rlc(operand: ReadWriteRef<u8>) -> Operator {
    rl_u8("RLC", operand, false)
}

pub fn rl(operand: ReadWriteRef<u8>) -> Operator {
    rl_u8("RL", operand, true)
}

fn rr_u8(mnemonic: &'static str, operand: ReadWriteRef<u8>, with_carry: bool) -> Operator {
    Operator::new(mnemonic, move |context| {
        let (current, writer) = operand.read_and_writer(context);
        let result = if with_carry {
            current >> 1 | (context.flags().c as u8) << 7
        } else {
            current.rotate_right(1)
        };
        writer(context, result);
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: false,
            c: current.bit(0),
        })
    })
}

pub fn rrca() -> Operator {
    rr_u8("RRCA", register::A, false)
}

pub fn rra() -> Operator {
    rr_u8("RLA", register::A, true)
}

pub fn rrc(operand: ReadWriteRef<u8>) -> Operator {
    rr_u8("RRC", operand, false)
}

pub fn rr(operand: ReadWriteRef<u8>) -> Operator {
    rr_u8("RR", operand, true)
}

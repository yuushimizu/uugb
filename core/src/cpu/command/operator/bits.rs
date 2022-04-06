use super::Operator;
use crate::cpu::{
    command::operand::{register, ReadRef, ReadWriteRef},
    registers::Flags,
};
use crate::util::bits::Bits;

pub fn swap(operand: ReadWriteRef<u8>) -> Operator {
    Operator::new("SWAP", |context| {
        let (current, writer) = operand.read_write(context);
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
        let (current, writer) = operand.read_write(context);
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
        let (current, writer) = operand.read_write(context);
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
    rr_u8("RRA", register::A, true)
}

pub fn rrc(operand: ReadWriteRef<u8>) -> Operator {
    rr_u8("RRC", operand, false)
}

pub fn rr(operand: ReadWriteRef<u8>) -> Operator {
    rr_u8("RR", operand, true)
}

pub fn sla(operand: ReadWriteRef<u8>) -> Operator {
    Operator::new("SLA", |context| {
        let (current, writer) = operand.read_write(context);
        let result = current << 1;
        writer(context, result);
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: false,
            c: current.bit(7),
        });
    })
}

pub fn sr_u8(mnemonic: &'static str, operand: ReadWriteRef<u8>, arithmetic: bool) -> Operator {
    Operator::new(mnemonic, move |context| {
        let (current, writer) = operand.read_write(context);
        let result = current >> 1 | ((arithmetic && current.bit(7)) as u8) << 7;
        writer(context, result);
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: false,
            c: current.bit(0),
        });
    })
}

pub fn sra(operand: ReadWriteRef<u8>) -> Operator {
    sr_u8("SRA", operand, true)
}

pub fn srl(operand: ReadWriteRef<u8>) -> Operator {
    sr_u8("SRL", operand, false)
}

pub fn bit(bit: u8, rhs: ReadRef<u8>) -> Operator {
    Operator::new("BIT", move |context| {
        let value = rhs.read(context);
        context.set_flags(Flags {
            z: !value.bit((bit & 0b111) as u32),
            n: false,
            h: true,
            ..context.flags()
        });
    })
}

pub fn set(bit: u8, rhs: ReadWriteRef<u8>) -> Operator {
    Operator::new("SET", move |context| {
        let (current, writer) = rhs.read_write(context);
        writer(context, current | 0b1 << bit)
    })
}

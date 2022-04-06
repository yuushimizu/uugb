use super::Operator;
use crate::cpu::{
    command::operand::{register, Read, ReadWrite},
    registers::Flags,
};
use crate::util::bits::Bits;

pub fn swap<O: ReadWrite<u8>>(operand: O) -> Operator {
    Operator::new("SWAP", move |context| {
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

fn rl_u8<O: ReadWrite<u8>>(mnemonic: &'static str, operand: O, with_carry: bool) -> Operator {
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

pub fn rlc<O: ReadWrite<u8>>(operand: O) -> Operator {
    rl_u8("RLC", operand, false)
}

pub fn rl<O: ReadWrite<u8>>(operand: O) -> Operator {
    rl_u8("RL", operand, true)
}

fn rr_u8<O: ReadWrite<u8>>(mnemonic: &'static str, operand: O, with_carry: bool) -> Operator {
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

pub fn rrc<O: ReadWrite<u8>>(operand: O) -> Operator {
    rr_u8("RRC", operand, false)
}

pub fn rr<O: ReadWrite<u8>>(operand: O) -> Operator {
    rr_u8("RR", operand, true)
}

pub fn sla<O: ReadWrite<u8>>(operand: O) -> Operator {
    Operator::new("SLA", move |context| {
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

pub fn sr_u8<O: ReadWrite<u8>>(mnemonic: &'static str, operand: O, arithmetic: bool) -> Operator {
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

pub fn sra<O: ReadWrite<u8>>(operand: O) -> Operator {
    sr_u8("SRA", operand, true)
}

pub fn srl<O: ReadWrite<u8>>(operand: O) -> Operator {
    sr_u8("SRL", operand, false)
}

pub fn bit<R: Read<u8>>(bit: u8, rhs: R) -> Operator {
    Operator::new("BIT", move |context| {
        let value = rhs.read(context);
        context.set_flags(Flags {
            z: !value.bit(bit as u32),
            n: false,
            h: true,
            ..context.flags()
        });
    })
}

pub fn set<R: ReadWrite<u8>>(bit: u8, rhs: R) -> Operator {
    Operator::new("SET", move |context| {
        let (current, writer) = rhs.read_write(context);
        writer(context, current.set_bit(bit as u32))
    })
}

pub fn res<R: ReadWrite<u8>>(bit: u8, rhs: R) -> Operator {
    Operator::new("RES", move |context| {
        let (current, writer) = rhs.read_write(context);
        writer(context, current.reset_bit(bit as u32))
    })
}

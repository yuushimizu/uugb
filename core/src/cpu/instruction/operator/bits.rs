use super::Operator;
use crate::{
    cpu::{
        instruction::operand::{register, Read, Write},
        registers::Flags,
    },
    util::bits::Bits,
};

pub fn swap(operand: impl Read<u8> + Write<u8>) -> Operator {
    Operator::new(format!("SWAP {}", operand), move |context| {
        let current = operand.read(context);
        let result = current.rotate_left(4);
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: false,
            c: false,
        });
        operand.write(context, result);
    })
}

pub fn cpl() -> Operator {
    Operator::new("CPL".into(), |context| {
        context.registers_mut().a = !context.registers().a;
        context.set_flags(Flags {
            n: true,
            h: true,
            ..*context.flags()
        });
    })
}

fn rl_u8(format: String, operand: impl Read<u8> + Write<u8>, with_carry: bool) -> Operator {
    Operator::new(format, move |context| {
        let current = operand.read(context);
        let result = if with_carry {
            current << 1 | context.flags().c as u8
        } else {
            current.rotate_left(1)
        };
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: false,
            c: current.bit(7),
        });
        operand.write(context, result);
    })
}

pub fn rlca() -> Operator {
    rl_u8("RLCA".into(), register::A, false)
}

pub fn rla() -> Operator {
    rl_u8("RLA".into(), register::A, true)
}

pub fn rlc(operand: impl Read<u8> + Write<u8>) -> Operator {
    rl_u8(format!("RLC {}", operand), operand, false)
}

pub fn rl(operand: impl Read<u8> + Write<u8>) -> Operator {
    rl_u8(format!("RL {}", operand), operand, true)
}

fn rr_u8(format: String, operand: impl Read<u8> + Write<u8>, with_carry: bool) -> Operator {
    Operator::new(format, move |context| {
        let current = operand.read(context);
        let result = if with_carry {
            current >> 1 | (context.flags().c as u8) << 7
        } else {
            current.rotate_right(1)
        };
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: false,
            c: current.bit(0),
        });
        operand.write(context, result);
    })
}

pub fn rrca() -> Operator {
    rr_u8("RRCA".into(), register::A, false)
}

pub fn rra() -> Operator {
    rr_u8("RRA".into(), register::A, true)
}

pub fn rrc(operand: impl Read<u8> + Write<u8>) -> Operator {
    rr_u8(format!("RRC {}", operand), operand, false)
}

pub fn rr(operand: impl Read<u8> + Write<u8>) -> Operator {
    rr_u8(format!("RR {}", operand), operand, true)
}

pub fn sla(operand: impl Read<u8> + Write<u8>) -> Operator {
    Operator::new(format!("SLA {}", operand), move |context| {
        let current = operand.read(context);
        let result = current << 1;
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: false,
            c: current.bit(7),
        });
        operand.write(context, result);
    })
}

pub fn sr_u8(
    mnemonic: &'static str,
    operand: impl Read<u8> + Write<u8>,
    arithmetic: bool,
) -> Operator {
    Operator::new(format!("{} {}", mnemonic, operand), move |context| {
        let current = operand.read(context);
        let result = current >> 1 | ((arithmetic && current.bit(7)) as u8) << 7;
        context.set_flags(Flags {
            z: result == 0,
            n: false,
            h: false,
            c: current.bit(0),
        });
        operand.write(context, result);
    })
}

pub fn sra(operand: impl Read<u8> + Write<u8>) -> Operator {
    sr_u8("SRA", operand, true)
}

pub fn srl(operand: impl Read<u8> + Write<u8>) -> Operator {
    sr_u8("SRL", operand, false)
}

pub fn bit(bit: u8, rhs: impl Read<u8>) -> Operator {
    Operator::new(format!("BIT {}, {}", bit, rhs), move |context| {
        let value = rhs.read(context);
        context.set_flags(Flags {
            z: !value.bit(bit as u32),
            n: false,
            h: true,
            ..*context.flags()
        });
    })
}

pub fn set(bit: u8, rhs: impl Read<u8> + Write<u8>) -> Operator {
    Operator::new(format!("SET {}, {}", bit, rhs), move |context| {
        let current = rhs.read(context);
        rhs.write(context, current.set_bit(bit as u32));
    })
}

pub fn res(bit: u8, rhs: impl Read<u8> + Write<u8>) -> Operator {
    Operator::new(format!("RES {}, {}", bit, rhs), move |context| {
        let current = rhs.read(context);
        rhs.write(context, current.reset_bit(bit as u32));
    })
}

use super::Operator;
use crate::{
    cpu::{
        instruction::operand::{register, Read, Write},
        registers::Flags,
    },
    util::bits::Bits,
};

pub fn swap(operand: impl Read<u8> + Write<u8>) -> Operator {
    Operator::new(
        move |context| {
            let current = operand.read(context);
            let result = current.rotate_left(4);
            context.set_flags(Flags {
                z: result == 0,
                n: false,
                h: false,
                c: false,
            });
            operand.write(context, result);
        },
        move |context| format!("SWAP {}", Read::<u8>::debug(&operand, context)),
    )
}

pub fn cpl() -> Operator {
    Operator::new(
        |context| {
            context.registers_mut().a = !context.registers().a;
            context.set_flags(Flags {
                n: true,
                h: true,
                ..*context.flags()
            });
        },
        |context| format!("CPL {:02X}", context.registers().a),
    )
}

fn rl_u8(mnemonic: &'static str, operand: impl Read<u8> + Write<u8>, with_carry: bool) -> Operator {
    Operator::new(
        move |context| {
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
                c: current.bit(7).into(),
            });
            operand.write(context, result);
        },
        move |context| format!("{} {}", mnemonic, Read::<u8>::debug(&operand, context)),
    )
}

pub fn rlca() -> Operator {
    rl_u8("RLCA", register::A, false)
}

pub fn rla() -> Operator {
    rl_u8("RLA", register::A, true)
}

pub fn rlc(operand: impl Read<u8> + Write<u8>) -> Operator {
    rl_u8("RLC", operand, false)
}

pub fn rl(operand: impl Read<u8> + Write<u8>) -> Operator {
    rl_u8("RL", operand, true)
}

fn rr_u8(mnemonic: &'static str, operand: impl Read<u8> + Write<u8>, with_carry: bool) -> Operator {
    Operator::new(
        move |context| {
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
                c: current.bit(0).into(),
            });
            operand.write(context, result);
        },
        move |context| format!("{} {}", mnemonic, Read::<u8>::debug(&operand, context)),
    )
}

pub fn rrca() -> Operator {
    rr_u8("RRCA", register::A, false)
}

pub fn rra() -> Operator {
    rr_u8("RRA", register::A, true)
}

pub fn rrc(operand: impl Read<u8> + Write<u8>) -> Operator {
    rr_u8("RRC", operand, false)
}

pub fn rr(operand: impl Read<u8> + Write<u8>) -> Operator {
    rr_u8("RR", operand, true)
}

pub fn sla(operand: impl Read<u8> + Write<u8>) -> Operator {
    Operator::new(
        move |context| {
            let current = operand.read(context);
            let result = current << 1;
            context.set_flags(Flags {
                z: result == 0,
                n: false,
                h: false,
                c: current.bit(7).into(),
            });
            operand.write(context, result);
        },
        move |context| format!("SLA {}", Read::<u8>::debug(&operand, context)),
    )
}

pub fn sr_u8(
    mnemonic: &'static str,
    operand: impl Read<u8> + Write<u8>,
    arithmetic: bool,
) -> Operator {
    Operator::new(
        move |context| {
            let current = operand.read(context);
            let result = current >> 1 | ((arithmetic && bool::from(current.bit(7))) as u8) << 7;
            context.set_flags(Flags {
                z: result == 0,
                n: false,
                h: false,
                c: current.bit(0).into(),
            });
            operand.write(context, result);
        },
        move |context| format!("{} {}", mnemonic, Read::<u8>::debug(&operand, context)),
    )
}

pub fn sra(operand: impl Read<u8> + Write<u8>) -> Operator {
    sr_u8("SRA", operand, true)
}

pub fn srl(operand: impl Read<u8> + Write<u8>) -> Operator {
    sr_u8("SRL", operand, false)
}

pub fn bit(bit: u8, rhs: impl Read<u8>) -> Operator {
    Operator::new(
        move |context| {
            let value = rhs.read(context);
            context.set_flags(Flags {
                z: !value.bit(bit as u32),
                n: false,
                h: true,
                ..*context.flags()
            });
        },
        move |context| format!("BIT {}, {}", bit, rhs.debug(context)),
    )
}

pub fn set(bit: u8, rhs: impl Read<u8> + Write<u8>) -> Operator {
    Operator::new(
        move |context| {
            let current = rhs.read(context);
            rhs.write(context, current.set_bit(bit as u32));
        },
        move |context| format!("SET {}, {}", bit, Read::<u8>::debug(&rhs, context)),
    )
}

pub fn res(bit: u8, rhs: impl Read<u8> + Write<u8>) -> Operator {
    Operator::new(
        move |context| {
            let current = rhs.read(context);
            rhs.write(context, current.reset_bit(bit as u32));
        },
        move |context| format!("RES {}, {}", bit, Read::<u8>::debug(&rhs, context)),
    )
}

use super::Operator;
use crate::cpu::{registers::Flags, Continuation};
use log;

pub fn daa() -> Operator {
    Operator::new("DAA".into(), |context| {
        let flags = context.flags().clone();
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
        Continuation::just(())
    })
}

pub fn ccf() -> Operator {
    Operator::new("CCF".into(), |context| {
        context.set_flags(Flags {
            n: false,
            h: false,
            c: !context.registers().f.c,
            ..*context.flags()
        });
        Continuation::just(())
    })
}

pub fn scf() -> Operator {
    Operator::new("SCF".into(), |context| {
        context.set_flags(Flags {
            n: false,
            h: false,
            c: true,
            ..*context.flags()
        });
        Continuation::just(())
    })
}

pub fn nop() -> Operator {
    Operator::new("NOP".into(), |_context| Continuation::just(()))
}

pub fn unused(opcode: u8) -> Operator {
    Operator::new("*UNUSED*".into(), move |_context| {
        log::warn!("The unused opcode has called: {:02X}", opcode);
        Continuation::just(())
    })
}

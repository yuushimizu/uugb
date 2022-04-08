use super::{jump::condition::Condition, Operator};
use crate::cpu::{instruction::operand::Read, Continuation};

pub fn call<A: Read<u16>>(address: A) -> Operator {
    Operator::new(format!("CALL {}", address), move |context| {
        address
            .read(context)
            .then(|context, address| context.call(address))
    })
}

pub fn call_cc<A: Read<u16>>(condition: Condition, address: A) -> Operator {
    Operator::new(format!("CALL {}, {}", condition, address), move |context| {
        address.read(context).then(move |context, address| {
            if condition.is_satisfied(context) {
                context.call(address)
            } else {
                Continuation::just(())
            }
        })
    })
}

pub fn rst(address: u8) -> Operator {
    Operator::new("RST".into(), move |context| context.call(address as u16))
}

pub fn ret() -> Operator {
    Operator::new("RET".into(), |context| context.ret())
}

pub fn ret_cc(condition: Condition) -> Operator {
    Operator::new(format!("RET {}", condition), move |context| {
        if condition.is_satisfied(context) {
            context.ret()
        } else {
            Continuation::just(())
        }
        .tick()
    })
}

pub fn reti() -> Operator {
    Operator::new("RETI".into(), move |context| {
        context.enable_interrupts();
        context.ret()
    })
}

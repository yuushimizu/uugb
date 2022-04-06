use super::{jump::condition::Condition, Operator};
use crate::cpu::instruction::operand::Read;

pub fn call_cc<L: Read<u16>>(condition: Condition, location: L) -> Operator {
    Operator::new(
        format!("CALL {}, {}", condition, location),
        move |context| {
            let address = location.read(context);
            if condition.is_satisfied(context) {
                context.call(address);
            }
        },
    )
}

pub fn call<L: Read<u16>>(location: L) -> Operator {
    Operator::new(format!("CALL {}", location), move |context| {
        let address = location.read(context);
        context.call(address);
    })
}

pub fn rst(address: u8) -> Operator {
    Operator::new("RST".into(), move |context| context.call(address as u16))
}

pub fn ret_cc(condition: Condition) -> Operator {
    Operator::new(format!("RET {}", condition), move |context| {
        if condition.is_satisfied(context) {
            context.ret();
        }
    })
}

pub fn ret() -> Operator {
    Operator::new("RET".into(), |context| {
        context.ret();
    })
}

pub fn reti() -> Operator {
    Operator::new("RETI".into(), move |context| {
        context.ret();
        context.enable_interrupts();
    })
}

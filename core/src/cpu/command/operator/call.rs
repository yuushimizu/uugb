use super::{jump::condition::Condition, Operator};
use crate::cpu::command::operand::Read;

pub fn call_cc<L: Read<u16>>(condition: Condition, location: L) -> Operator {
    Operator::new("CALL", move |context| {
        let address = location.read(context);
        if condition(context.flags()) {
            context.call(address);
        }
    })
}

pub fn call<L: Read<u16>>(location: L) -> Operator {
    call_cc(|_| true, location)
}

pub fn rst(address: u8) -> Operator {
    Operator::new("RST", move |context| context.call(address as u16))
}

pub fn ret_cc(condition: Condition) -> Operator {
    Operator::new("RET", move |context| {
        if condition(context.flags()) {
            let address = context.pop16_sp();
            context.jump(address);
        }
    })
}

pub fn ret() -> Operator {
    ret_cc(|_| true)
}

pub fn reti() -> Operator {
    Operator::new("RETI", move |context| {
        let address = context.pop16_sp();
        context.jump(address);
        context.enable_interrupts();
    })
}

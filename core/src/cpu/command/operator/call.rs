use super::{jump::condition::Condition, Operator};
use crate::cpu::command::operand::Read;

pub fn call_cc(condition: Condition, location: Read<u16>) -> Operator {
    Operator::new("CALL", move |context| {
        let address = location.read(context);
        if condition(context.flags()) {
            context.call(address);
        }
    })
}

pub fn call(location: Read<u16>) -> Operator {
    call_cc(|_| true, location)
}

pub fn rst(address: u8) -> Operator {
    Operator::new("RST", move |context| context.call(address as u16))
}

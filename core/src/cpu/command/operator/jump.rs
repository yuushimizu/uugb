use super::Operator;
use crate::cpu::command::operand::ReadRef;

pub fn jp(location: ReadRef<u16>) -> Operator {
    Operator::new("JP", |context| {
        context.registers_mut().pc = location.read(context);
    })
}

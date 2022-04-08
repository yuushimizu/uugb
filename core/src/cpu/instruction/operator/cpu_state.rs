use super::Operator;
use crate::cpu::Continuation;

pub fn halt() -> Operator {
    Operator::new("HALT".into(), |context| {
        context.halt();
        Continuation::just(())
    })
}

pub fn stop() -> Operator {
    Operator::new("STOP".into(), |context| {
        context.stop();
        Continuation::just(())
    })
}

pub fn di() -> Operator {
    Operator::new("DI".into(), |context| {
        context.disable_interrupts();
        Continuation::just(())
    })
}

pub fn ei() -> Operator {
    Operator::new("EI".into(), |context| {
        context.enable_interrupts();
        Continuation::just(())
    })
}

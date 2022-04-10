use super::Operator;

pub fn halt() -> Operator {
    Operator::new(
        |context| {
            context.halt();
        },
        |_| "HALT".into(),
    )
}

pub fn stop() -> Operator {
    Operator::new(
        |context| {
            context.stop();
        },
        |_| "STOP".into(),
    )
}

pub fn di() -> Operator {
    Operator::new(
        |context| {
            context.disable_interrupts();
        },
        |_| "DI".into(),
    )
}

pub fn ei() -> Operator {
    Operator::new(
        |context| {
            context.enable_interrupts();
        },
        |_| "EI".into(),
    )
}

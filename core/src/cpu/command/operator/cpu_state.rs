use super::Operator;

pub fn halt() -> Operator {
    Operator::new("HALT", |context| context.halt())
}

pub fn stop() -> Operator {
    Operator::new("STOP", |context| context.stop())
}

pub fn di() -> Operator {
    Operator::new("DI", |context| context.disable_interrupts())
}

pub fn ei() -> Operator {
    Operator::new("EI", |context| context.enable_interrupts())
}

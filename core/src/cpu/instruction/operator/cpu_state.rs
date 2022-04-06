use super::Operator;

pub fn halt() -> Operator {
    Operator::new("HALT".into(), |context| context.halt())
}

pub fn stop() -> Operator {
    Operator::new("STOP".into(), |context| context.stop())
}

pub fn di() -> Operator {
    Operator::new("DI".into(), |context| context.disable_interrupts())
}

pub fn ei() -> Operator {
    Operator::new("EI".into(), |context| context.enable_interrupts())
}

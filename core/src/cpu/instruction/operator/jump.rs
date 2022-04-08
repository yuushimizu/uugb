use super::Operator;
use crate::cpu::{instruction::operand::Read, Continuation};

pub mod condition {
    use crate::cpu::{registers::Flags, CpuContext};
    use std::fmt;

    #[derive(Clone, Copy)]
    pub struct Condition {
        name: &'static str,
        predicate: fn(&Flags) -> bool,
    }

    impl fmt::Debug for Condition {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("Condition")
                .field("name", &self.name)
                .finish()
        }
    }

    impl fmt::Display for Condition {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.name)
        }
    }

    impl Condition {
        pub fn is_satisfied(&self, context: &dyn CpuContext) -> bool {
            (self.predicate)(context.flags())
        }
    }

    pub const NZ: Condition = Condition {
        name: "NZ",
        predicate: |flags| !flags.z,
    };

    pub const Z: Condition = Condition {
        name: "Z",
        predicate: |flags| flags.z,
    };

    pub const NC: Condition = Condition {
        name: "NC",
        predicate: |flags| !flags.c,
    };

    pub const C: Condition = Condition {
        name: "C",
        predicate: |flags| flags.c,
    };
}

pub use condition::Condition;

pub fn jp_nn() -> Operator {
    Operator::new(format!("JP #"), move |context| {
        context
            .fetch16()
            .map(|context, address| context.jump(address))
            .tick()
    })
}

pub fn jp_hl() -> Operator {
    Operator::new(format!("JP HL"), move |context| {
        context.jump(context.registers().hl());
        Continuation::just(())
    })
}

pub fn jp_cc<A: Read<u16>>(condition: Condition, address: A) -> Operator {
    Operator::new(format!("JP {}, {}", condition, address), move |context| {
        address.read(context).then(move |context, address| {
            if condition.is_satisfied(context) {
                context.jump(address);
                Continuation::just(()).tick()
            } else {
                Continuation::just(())
            }
        })
    })
}

pub fn jr<O: Read<u8>>(operand: O) -> Operator {
    Operator::new(format!("JR {}", operand), move |context| {
        operand
            .read(context)
            .map(|context, offset| {
                context.jump(context.registers().pc.wrapping_add(offset as i8 as u16))
            })
            .tick()
    })
}

pub fn jr_cc<O: Read<u8>>(condition: Condition, operand: O) -> Operator {
    Operator::new(format!("JR {}, {}", condition, operand), move |context| {
        operand.read(context).then(move |context, offset| {
            if condition.is_satisfied(context) {
                context.jump(context.registers().pc.wrapping_add(offset as i8 as u16));
                Continuation::just(()).tick()
            } else {
                Continuation::just(())
            }
        })
    })
}

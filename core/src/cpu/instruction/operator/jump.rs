use super::Operator;
use crate::cpu::instruction::operand::Read;

pub mod condition {
    use crate::cpu::{registers::Flags, Context};
    use std::fmt;

    #[derive(Clone)]
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
        pub fn is_satisfied(&self, context: &dyn Context) -> bool {
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

pub fn jp_cc<L: Read<u16>>(condition: Condition, location: L) -> Operator {
    Operator::new(format!("JP {}, {}", condition, location), move |context| {
        let address = location.read(context);
        if condition.is_satisfied(context) {
            context.jump(address);
        }
    })
}

pub fn jp<L: Read<u16>>(location: L) -> Operator {
    Operator::new(format!("JP {}", location), move |context| {
        let address = location.read(context);
        context.jump(address);
    })
}

pub fn jr_cc<O: Read<u8>>(condition: Condition, operand: O) -> Operator {
    Operator::new(format!("JR {}, {}", condition, operand), move |context| {
        let offset = operand.read(context);
        if condition.is_satisfied(context) {
            context.jump(context.registers().pc.wrapping_add(offset as i8 as u16));
        }
    })
}

pub fn jr<O: Read<u8>>(operand: O) -> Operator {
    Operator::new(format!("JR {}", operand), move |context| {
        let offset = operand.read(context);
        context.jump(context.registers().pc.wrapping_add(offset as i8 as u16));
    })
}

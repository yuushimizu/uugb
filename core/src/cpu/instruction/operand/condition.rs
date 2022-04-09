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
    pub fn is_satisfied(&self, context: &CpuContext) -> bool {
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

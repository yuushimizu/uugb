use crate::cpu::command::operand::Operand;
use std::fmt;

/*
#[derive(Debug)]
pub struct Format {
    mnemonic: &'static str,
    operands: Vec<&'static dyn Operand>,
}
*/

/*
impl Format {
    pub fn new(mnemonic: &'static str, operands: &[&'static dyn Operand]) -> Self {
        Self {
            mnemonic,
            operands: operands.into(),
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.operands.is_empty() {
            write!(f, "{}", self.mnemonic)
        } else {
            write!(
                f,
                " {} {}",
                self.mnemonic,
                self.operands
                    .iter()
                    .map(|operand| format!("{}", operand))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
    }
}
*/

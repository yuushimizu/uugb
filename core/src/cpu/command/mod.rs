mod parameter;

use super::Context;

pub struct Command {
    opcode: u8,
    mnemonic: &'static str,
    cycles: u64,
    execute: fn(context: &mut dyn Context),
}

impl Command {
    pub fn execute(&self, context: &mut dyn Context) {
        (self.execute)(context)
    }

    pub fn next(context: &mut dyn Context) -> Self {
        let opcode = context.pop_from_pc();
        let command =
            |mnemonic: &'static str, cycles: u64, execute: fn(context: &mut dyn Context)| Self {
                opcode,
                mnemonic,
                cycles,
                execute,
            };
        match opcode {
            // Miscellaneous
            0x00 => command("NOP", 4, |_| {}),
            // Jumps
            0xC3 => command("JP", 12, |context| {
                context.registers_mut().pc = context.pop16_from_pc();
            }),
            // Not Implemented
            _ => panic!("This opcode is not implemented!: {:02X}", opcode),
        }
    }
}

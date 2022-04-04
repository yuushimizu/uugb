mod command;
mod registers;

pub use command::Command;
pub use registers::Registers;

use crate::memory::Memory;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    fn pop_from_pc(&mut self, memory: &mut Memory) -> u8 {
        let opcode = memory.read(self.registers.pc);
        self.registers.pc += 1;
        opcode
    }

    pub fn execute_next(&mut self, memory: &mut Memory) {
        match self.pop_from_pc(memory) {
            opcode => panic!("This Opcode is not implemented!: {:02X}", opcode),
        }
    }
}

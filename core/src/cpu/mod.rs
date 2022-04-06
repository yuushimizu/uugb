pub mod registers;

mod cpu_context;
mod instruction;

pub use instruction::Instruction;
pub use registers::Registers;

use cpu_context::CpuContext;

use crate::memory::Memory;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Cpu {
    registers: Registers,
}

struct Context<'a> {
    cpu: &'a mut Cpu,
    memory: &'a mut Memory,
}

impl<'a> CpuContext for Context<'a> {
    fn registers(&self) -> &Registers {
        &self.cpu.registers
    }

    fn registers_mut(&mut self) -> &mut Registers {
        &mut self.cpu.registers
    }

    fn memory(&self) -> &Memory {
        &self.memory
    }

    fn memory_mut(&mut self) -> &mut Memory {
        &mut self.memory
    }

    fn halt(&mut self) {
        todo!("HALT");
    }

    fn stop(&mut self) {
        todo!("STOP");
    }

    fn disable_interrupts(&mut self) {
        todo!("DI");
    }

    fn enable_interrupts(&mut self) {
        todo!("EI");
    }
}

impl Cpu {
    pub fn step(&mut self, memory: &mut Memory) -> Instruction {
        let mut context = Context { cpu: self, memory };
        let instruction = Instruction::next(&mut context);
        instruction.execute(&mut context);
        instruction
    }
}

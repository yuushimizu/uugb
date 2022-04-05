pub mod registers;

mod command;
mod context;

pub use command::Command;
pub use registers::Registers;

use context::Context;

use crate::memory::Memory;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Cpu {
    registers: Registers,
}

struct CpuContext<'a> {
    cpu: &'a mut Cpu,
    memory: &'a mut Memory,
}

impl<'a> Context for CpuContext<'a> {
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
}

impl Cpu {
    pub fn step(&mut self, memory: &mut Memory) -> Command {
        let mut context = CpuContext { cpu: self, memory };
        let command = Command::next(&mut context);
        command.execute(&mut context);
        command
    }
}

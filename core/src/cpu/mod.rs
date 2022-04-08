pub mod registers;

mod continuation;
mod cpu_context;
mod instruction;

pub use instruction::Instruction;
pub use registers::Registers;

use continuation::Continuation;
use cpu_context::CpuContext;
use log;

use crate::memory::Memory;

#[derive(Debug, Default)]
pub struct Cpu {
    registers: Registers,
    is_halting: bool,
    interrupt_master_enable_flag: bool,
    interrupt_master_enabling: bool,
    continuation: Option<Continuation<()>>,
}

struct Context<'a> {
    cpu: &'a mut Cpu,
    memory: &'a mut dyn Memory,
}

impl<'a> CpuContext for Context<'a> {
    fn registers(&self) -> &Registers {
        &self.cpu.registers
    }

    fn registers_mut(&mut self) -> &mut Registers {
        &mut self.cpu.registers
    }

    fn memory(&self) -> &dyn Memory {
        self.memory
    }

    fn memory_mut(&mut self) -> &mut dyn Memory {
        self.memory
    }

    fn halt(&mut self) {
        self.cpu.is_halting = true;
        todo!("HALT");
    }

    fn stop(&mut self) {
        self.cpu.is_halting = true;
        todo!("STOP");
    }

    fn disable_interrupts(&mut self) {
        self.cpu.interrupt_master_enable_flag = false;
    }

    fn enable_interrupts(&mut self) {
        self.cpu.interrupt_master_enabling = true;
    }
}

impl Cpu {
    pub fn tick(&mut self, memory: &mut dyn Memory) {
        use Continuation::*;
        let mut continuation = self.continuation.take();
        let mut context = Context { cpu: self, memory };
        loop {
            match continuation {
                None | Some(Return(_)) => {
                    continuation = Some(Instruction::fetch(&mut context).then(
                        |context, instruction| {
                            log::info!(target: "cpu_event", "Instruction: {}", instruction);
                            instruction.execute(context)
                        },
                    ))
                }
                Some(Continue(next)) => {
                    continuation = Some(next(&mut context));
                }
                Some(Tick(next)) => {
                    continuation = Some(*next);
                    break;
                }
            }
        }
        self.continuation = continuation;
    }
}

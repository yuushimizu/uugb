pub mod registers;

mod context;
mod instruction;

pub use context::Context;
pub use registers::Registers;

use instruction::Instruction;

use crate::{interrupt::Interrupt, memory::Memory};
use log;

#[derive(Debug, Default)]
pub struct Cpu {
    registers: Registers,
    is_halted: bool,
    is_stopped: bool,
    interrupt_enabled: bool,
    interrupt_enabling: bool,
    wait_cycles: u64,
}

struct InstructionContextComponents<'a> {
    cpu: &'a mut Cpu,
    memory: Memory<'a>,
}

impl<'a> instruction::context::Components for InstructionContextComponents<'a> {
    fn registers(&self) -> &Registers {
        &self.cpu.registers
    }

    fn registers_mut(&mut self) -> &mut Registers {
        &mut self.cpu.registers
    }

    fn read(&self, address: u16) -> u8 {
        self.memory.read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.memory.write(address, value)
    }

    fn halt(&mut self) {
        self.cpu.is_halted = true;
    }

    fn stop(&mut self) {
        self.cpu.is_halted = true;
        self.cpu.is_stopped = true;
    }

    fn disable_interrupts(&mut self) {
        self.cpu.interrupt_enabled = false;
    }

    fn enable_interrupts(&mut self) {
        self.cpu.interrupt_enabling = true;
    }

    fn wait(&mut self) {
        self.cpu.wait_cycles += 4;
    }
}

impl Cpu {
    fn with_instruction_context(
        &mut self,
        context: &mut impl Context,
        f: impl FnOnce(&mut instruction::Context),
    ) {
        let mut instruction_context_components = InstructionContextComponents {
            cpu: self,
            memory: Memory::new(context),
        };
        let mut instruction_context =
            instruction::Context::new(&mut instruction_context_components);
        f(&mut instruction_context);
    }

    fn perform_interrupt(&mut self, context: &mut impl Context, interrupt: Interrupt) {
        self.interrupt_enabled = false;
        self.interrupt_enabling = false;
        context.interrupt_controller_mut().clear(interrupt);
        self.with_instruction_context(context, |instruction_context| {
            log::info!(target: "cpu_event", "Handle interrupt: {:?}", interrupt);
            instruction_context.wait();
            instruction_context.wait();
            instruction_context.call(interrupt.address());
        });
    }

    pub fn tick(&mut self, context: &mut impl Context) {
        if self.wait_cycles > 0 {
            self.wait_cycles -= 1;
            return;
        }
        if self.interrupt_enabling {
            self.interrupt_enabled = true;
            self.interrupt_enabling = false;
        }
        if let Some(interrupt) = context.interrupt_controller().pending_interrupt() {
            self.is_halted = false;
            if self.interrupt_enabled {
                self.perform_interrupt(context, interrupt);
                return;
            }
        }
        if self.is_halted {
            return;
        }
        self.with_instruction_context(context, |instruction_context| {
            let instruction = Instruction::fetch(instruction_context);
            log::info!(target: "cpu_event", "Instruction: {}", instruction);
            instruction.execute(instruction_context);
        });
        if self.is_stopped {
            context.interrupt_controller_mut().clear_all();
        }
    }
}

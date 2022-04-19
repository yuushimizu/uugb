pub mod registers;

mod instruction;

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
    wait_m_cycles: u64,
}

struct InstructionContextComponents<'a, 'memory> {
    cpu: &'a mut Cpu,
    memory: &'a mut Memory<'memory>,
}

impl<'a, 'memory> instruction::context::Components for InstructionContextComponents<'a, 'memory> {
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
        self.cpu.wait_m_cycles += 1;
    }
}

impl Cpu {
    fn with_instruction_context<'memory>(
        &mut self,
        memory: &mut Memory<'memory>,
        f: impl FnOnce(&mut instruction::Context),
    ) {
        let mut instruction_context_components = InstructionContextComponents { cpu: self, memory };
        let mut instruction_context =
            instruction::Context::new(&mut instruction_context_components);
        f(&mut instruction_context);
    }

    fn perform_interrupt<'memory>(&mut self, memory: &mut Memory<'memory>, interrupt: Interrupt) {
        self.interrupt_enabled = false;
        self.interrupt_enabling = false;
        memory
            .components_mut()
            .interrupt_controller
            .clear(interrupt);
        self.with_instruction_context(memory, |instruction_context| {
            log::info!(target: "cpu_event", "Handle interrupt: {:?}", interrupt);
            instruction_context.wait();
            instruction_context.wait();
            instruction_context.call(interrupt.address());
        });
    }

    pub fn tick<'cpu, 'memory>(&'cpu mut self, memory: &'memory mut Memory<'memory>) {
        self.wait_m_cycles = self.wait_m_cycles.saturating_sub(1);
        if self.wait_m_cycles > 0 {
            return;
        }
        if self.interrupt_enabling {
            self.interrupt_enabled = true;
            self.interrupt_enabling = false;
        }
        if let Some(interrupt) = memory.components().interrupt_controller.pending_interrupt() {
            self.is_halted = false;
            if self.interrupt_enabled {
                self.perform_interrupt(memory, interrupt);
                return;
            }
        }
        if self.is_halted {
            return;
        }
        self.with_instruction_context(memory, |instruction_context| {
            let pc = instruction_context.registers().pc;
            let instruction = Instruction::fetch(instruction_context);
            log::info!(target: "cpu_event", "Instruction: {:04X} {}", pc, instruction.debug(instruction_context));
            instruction.execute(instruction_context);
        });
        if self.is_stopped {
            memory.components_mut().interrupt_controller.clear_all();
        }
    }
}

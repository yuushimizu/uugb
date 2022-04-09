pub mod registers;

mod instruction;

pub use registers::Registers;

use instruction::Instruction;

use crate::memory::Memory;
use log;

#[derive(Debug, Default)]
pub struct Cpu {
    registers: Registers,
    is_halting: bool,
    interrupt_master_enable_flag: bool,
    interrupt_master_enabling: bool,
    wait_cycles: u64,
}

struct Components<'a, M: Memory> {
    cpu: &'a mut Cpu,
    memory: &'a mut M,
}

impl<'a, M: Memory> instruction::context::Components for Components<'a, M> {
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

    fn wait(&mut self) {
        self.cpu.wait_cycles += 4;
    }
}

impl Cpu {
    pub fn tick(&mut self, memory: &mut impl Memory) {
        if self.wait_cycles > 0 {
            self.wait_cycles -= 1;
            return;
        }
        let mut components = Components { cpu: self, memory };
        let mut context = instruction::Context::new(&mut components);
        let instruction = Instruction::fetch(&mut context);
        log::info!(target: "cpu_event", "Instruction: {}", instruction);
        instruction.execute(&mut context);
    }
}

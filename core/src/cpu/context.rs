use super::Registers;
use crate::memory::Memory;

pub trait Context {
    fn registers(&self) -> &Registers;

    fn registers_mut(&mut self) -> &mut Registers;

    fn memory(&self) -> &Memory;

    fn memory_mut(&mut self) -> &mut Memory;

    fn pop_from_pc(&mut self) -> u8 {
        let value = self.memory().read(self.registers().pc);
        self.registers_mut().pc += 1;
        value
    }

    fn pop16_from_pc(&mut self) -> u16 {
        self.pop_from_pc() as u16 | (self.pop_from_pc() as u16) << 8
    }
}

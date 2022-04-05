use super::{registers::Flags, Registers};
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

    fn read16(&mut self, address: u16) -> u16 {
        self.memory().read(address) as u16 | (self.memory().read(address + 1) as u16) << 8
    }

    fn write16(&mut self, address: u16, value: u16) {
        self.memory_mut().write(address, (value & 0xFF) as u8);
        self.memory_mut().write(address + 1, (value >> 8) as u8);
    }

    fn add_i8_to_sp(&mut self, n: u8) -> u16 {
        let sp = self.registers().sp;
        let n16 = n as i8 as u16;
        self.registers_mut().f = Flags {
            z: false,
            n: false,
            h: ((sp & 0xF) + (n16 & 0xF)) > 0xF,
            c: ((sp & 0xFF) + (n16 & 0xFF)) > 0xFF,
        };
        sp.wrapping_add(n16)
    }
}

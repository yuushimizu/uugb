use super::{registers::Flags, Registers};
use crate::memory::Memory;

pub trait Context {
    fn registers(&self) -> &Registers;

    fn registers_mut(&mut self) -> &mut Registers;

    fn memory(&self) -> &Memory;

    fn memory_mut(&mut self) -> &mut Memory;

    fn halt(&mut self);

    fn stop(&mut self);

    fn disable_interrupts(&mut self);

    fn enable_interrupts(&mut self);

    fn flags(&self) -> Flags {
        self.registers().f.clone()
    }

    fn set_flags(&mut self, flags: Flags) {
        self.registers_mut().f = flags;
    }

    fn read(&self, address: u16) -> u8 {
        self.memory().read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.memory_mut().write(address, value);
    }

    fn read16(&self, address: u16) -> u16 {
        self.read(address) as u16 | (self.read(address + 1) as u16) << 8
    }

    fn write16(&mut self, address: u16, value: u16) {
        self.write(address, value as u8);
        self.write(address + 1, (value >> 8) as u8);
    }

    fn fetch_pc(&mut self) -> u8 {
        let value = self.read(self.registers().pc);
        self.registers_mut().pc += 1;
        value
    }

    fn fetch16_pc(&mut self) -> u16 {
        self.fetch_pc() as u16 | (self.fetch_pc() as u16) << 8
    }

    fn jump(&mut self, address: u16) {
        self.registers_mut().pc = address;
    }

    fn add_sp(&mut self, n: u8) -> u16 {
        let sp = self.registers().sp;
        let n16 = n as i8 as u16;
        self.set_flags(Flags {
            z: false,
            n: false,
            h: ((sp & 0xF) + (n16 & 0xF)) > 0xF,
            c: ((sp & 0xFF) + (n16 & 0xFF)) > 0xFF,
        });
        sp.wrapping_add(n16)
    }

    fn push_sp(&mut self, value: u8) {
        let address = self.registers().sp.wrapping_sub(1);
        self.write(address, value);
        self.registers_mut().sp = address;
    }

    fn pop_sp(&mut self) -> u8 {
        let address = self.registers().sp;
        let value = self.read(address);
        self.registers_mut().sp = address.wrapping_add(1);
        value
    }

    fn push16_sp(&mut self, value: u16) {
        self.push_sp((value >> 8) as u8);
        self.push_sp(value as u8);
    }

    fn pop16_sp(&mut self) -> u16 {
        self.pop_sp() as u16 | (self.pop_sp() as u16) << 8
    }

    fn call(&mut self, address: u16) {
        self.push16_sp(self.registers().pc);
        self.jump(address);
    }

    fn ret(&mut self) {
        let address = self.pop16_sp();
        self.jump(address);
    }
}

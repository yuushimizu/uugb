use super::{registers::Flags, Registers};
use crate::memory::Memory;

fn to_u16(lower: u8, upper: u8) -> u16 {
    (upper as u16) << 8 | lower as u16
}

pub trait CpuContext {
    fn registers(&self) -> &Registers;

    fn registers_mut(&mut self) -> &mut Registers;

    fn memory(&self) -> &dyn Memory;

    fn memory_mut(&mut self) -> &mut dyn Memory;

    fn halt(&mut self);

    fn stop(&mut self);

    fn disable_interrupts(&mut self);

    fn enable_interrupts(&mut self);

    fn wait(&mut self);

    fn flags(&self) -> &Flags {
        &self.registers().f
    }

    fn set_flags(&mut self, flags: Flags) {
        self.registers_mut().f = flags;
    }

    fn read(&mut self, address: u16) -> u8 {
        self.wait();
        self.memory().read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.memory_mut().write(address, value);
        self.wait();
    }

    fn read16(&mut self, address: u16) -> u16 {
        to_u16(self.read(address), self.read(address + 1))
    }

    fn write16(&mut self, address: u16, value: u16) {
        self.write(address, value as u8);
        self.write(address + 1, (value >> 8) as u8);
    }

    fn fetch(&mut self) -> u8 {
        let address = self.registers().pc;
        self.registers_mut().pc = address.wrapping_add(1);
        self.read(address)
    }

    fn fetch16(&mut self) -> u16 {
        to_u16(self.fetch(), self.fetch())
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
        self.wait();
        sp.wrapping_add(n16)
    }

    fn push(&mut self, value: u8) {
        let address = self.registers().sp.wrapping_sub(1);
        self.registers_mut().sp = address;
        self.write(address, value);
    }

    fn pop(&mut self) -> u8 {
        let address = self.registers().sp;
        self.registers_mut().sp = address.wrapping_add(1);
        self.read(address)
    }

    fn push16(&mut self, value: u16) {
        self.push((value >> 8) as u8);
        self.push(value as u8);
    }

    fn pop16(&mut self) -> u16 {
        to_u16(self.pop(), self.pop())
    }

    fn call(&mut self, address: u16) {
        self.push16(self.registers().pc);
        self.jump(address);
        self.wait();
    }

    fn ret(&mut self) {
        let address = self.pop16();
        self.jump(address);
        self.wait();
    }
}

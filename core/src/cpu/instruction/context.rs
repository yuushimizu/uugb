use crate::cpu::{registers::Flags, Registers};

pub trait Components {
    fn registers(&self) -> &Registers;

    fn registers_mut(&mut self) -> &mut Registers;

    fn read(&self, address: u16) -> u8;

    fn write(&mut self, address: u16, value: u8);

    fn halt(&mut self);

    fn stop(&mut self);

    fn disable_interrupts(&mut self);

    fn enable_interrupts(&mut self);

    fn wait(&mut self);
}

fn to_u16(lower: u8, upper: u8) -> u16 {
    (upper as u16) << 8 | lower as u16
}

pub struct Context<'a> {
    components: &'a mut dyn Components,
}

impl<'a> Context<'a> {
    pub fn new(components: &'a mut impl Components) -> Self {
        Self { components }
    }

    pub fn registers(&self) -> &Registers {
        self.components.registers()
    }

    pub fn registers_mut(&mut self) -> &mut Registers {
        self.components.registers_mut()
    }

    pub fn halt(&mut self) {
        self.components.halt();
    }

    pub fn stop(&mut self) {
        self.components.stop();
    }

    pub fn disable_interrupts(&mut self) {
        self.components.disable_interrupts();
    }

    pub fn enable_interrupts(&mut self) {
        self.components.enable_interrupts();
    }

    pub fn wait(&mut self) {
        self.components.wait()
    }

    pub fn flags(&self) -> &Flags {
        &self.registers().f
    }

    pub fn set_flags(&mut self, flags: Flags) {
        self.registers_mut().f = flags;
    }

    pub fn read(&mut self, address: u16) -> u8 {
        self.wait();
        self.components.read(address)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.components.write(address, value);
        self.wait();
    }

    pub fn read16(&mut self, address: u16) -> u16 {
        to_u16(self.read(address), self.read(address + 1))
    }

    pub fn write16(&mut self, address: u16, value: u16) {
        self.write(address, value as u8);
        self.write(address + 1, (value >> 8) as u8);
    }

    pub fn fetch(&mut self) -> u8 {
        let address = self.registers().pc;
        self.registers_mut().pc = address.wrapping_add(1);
        self.read(address)
    }

    pub fn fetch16(&mut self) -> u16 {
        to_u16(self.fetch(), self.fetch())
    }

    pub fn jump(&mut self, address: u16) {
        self.registers_mut().pc = address;
    }

    pub fn add_sp(&mut self, n: u8) -> u16 {
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

    pub fn push(&mut self, value: u8) {
        let address = self.registers().sp.wrapping_sub(1);
        self.registers_mut().sp = address;
        self.write(address, value);
    }

    pub fn pop(&mut self) -> u8 {
        let address = self.registers().sp;
        self.registers_mut().sp = address.wrapping_add(1);
        self.read(address)
    }

    pub fn push16(&mut self, value: u16) {
        self.push((value >> 8) as u8);
        self.push(value as u8);
    }

    pub fn pop16(&mut self) -> u16 {
        to_u16(self.pop(), self.pop())
    }

    pub fn call(&mut self, address: u16) {
        self.push16(self.registers().pc);
        self.jump(address);
        self.wait();
    }

    pub fn ret(&mut self) {
        let address = self.pop16();
        self.jump(address);
        self.wait();
    }

    pub fn debug_u8(&self, address: u16) -> u8 {
        self.components.read(address)
    }

    pub fn debug_u16(&self, address: u16) -> u16 {
        to_u16(
            self.components.read(address),
            self.components.read(address + 1),
        )
    }
}

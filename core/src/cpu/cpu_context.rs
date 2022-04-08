use super::{registers::Flags, Continuation, Registers};
use crate::memory::Memory;

fn to_u16(upper: u8, lower: u8) -> u16 {
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

    fn flags(&self) -> &Flags {
        &self.registers().f
    }

    fn set_flags(&mut self, flags: Flags) {
        self.registers_mut().f = flags;
    }

    fn read(&self, address: u16) -> Continuation<u8> {
        Continuation::just(self.memory().read(address)).tick()
    }

    fn write(&mut self, address: u16, value: u8) -> Continuation<()> {
        self.memory_mut().write(address, value);
        Continuation::just(()).tick()
    }

    fn read16(&self, address: u16) -> Continuation<u16> {
        self.read(address).then(move |context, lower| {
            context
                .read(address + 1)
                .map(move |_context, upper| to_u16(upper, lower))
        })
    }

    fn write16(&mut self, address: u16, value: u16) -> Continuation<()> {
        self.write(address, value as u8)
            .then(move |context, _| context.write(address + 1, (value >> 8) as u8))
    }

    fn fetch(&mut self) -> Continuation<u8> {
        let address = self.registers().pc;
        self.registers_mut().pc = address.wrapping_add(1);
        self.read(address)
    }

    fn fetch16(&mut self) -> Continuation<u16> {
        self.fetch().then(|context, lower| {
            context
                .fetch()
                .map(move |_context, upper| to_u16(upper, lower))
        })
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

    fn push(&mut self, value: u8) -> Continuation<()> {
        let address = self.registers().sp.wrapping_sub(1);
        self.registers_mut().sp = address;
        self.write(address, value)
    }

    fn pop(&mut self) -> Continuation<u8> {
        let address = self.registers().sp;
        self.registers_mut().sp = address.wrapping_add(1);
        self.read(address)
    }

    fn push16(&mut self, value: u16) -> Continuation<()> {
        self.push((value >> 8) as u8)
            .then(move |context, _| context.push(value as u8))
    }

    fn pop16(&mut self) -> Continuation<u16> {
        self.pop().then(|context, lower| {
            context
                .pop()
                .map(move |_context, upper| to_u16(upper, lower))
        })
    }

    fn call(&mut self, address: u16) -> Continuation<()> {
        self.push16(self.registers().pc)
            .map(move |context, _| context.jump(address))
    }

    fn ret(&mut self) -> Continuation<()> {
        self.pop16().map(|context, address| context.jump(address))
    }
}

use crate::memory::Memory;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Flags {
    z: bool,
    n: bool,
    h: bool,
    c: bool,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            z: true,
            n: false,
            h: false,
            c: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Registers {
    a: u8,
    f: Flags,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            a: 0x11,
            f: Default::default(),
            b: 0x00,
            c: 0x00,
            d: 0xFF,
            e: 0x56,
            h: 0x00,
            l: 0x0d,
            pc: 0x0100,
            sp: 0xFFFE,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    fn pop_from_pc(&mut self, memory: &mut Memory) -> u8 {
        let opcode = memory.read(self.registers.pc);
        self.registers.pc += 1;
        opcode
    }

    pub fn execute_next(&mut self, memory: &mut Memory) {
        match self.pop_from_pc(memory) {
            opcode => panic!("This Opcode is not implemented!: {:02X}", opcode),
        }
    }
}

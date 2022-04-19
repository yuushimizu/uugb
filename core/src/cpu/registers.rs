use crate::util::bits::Bits;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Flags {
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            z: true,
            n: false,
            h: true,
            c: true,
        }
    }
}

impl From<Flags> for u8 {
    fn from(flags: Flags) -> Self {
        (flags.z as u8) << 7 | (flags.n as u8) << 6 | (flags.h as u8) << 5 | (flags.c as u8) << 4
    }
}

impl From<u8> for Flags {
    fn from(byte: u8) -> Self {
        Self {
            z: byte.bit(7),
            n: byte.bit(6),
            h: byte.bit(5),
            c: byte.bit(4),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Registers {
    pub a: u8,
    pub f: Flags,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            a: 0x01,
            f: Default::default(),
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            pc: 0x0100,
            sp: 0xFFFE,
        }
    }
}

fn to_u16(n: u8, m: u8) -> u16 {
    u16::from_be_bytes([n, m])
}

fn to_bytes(n: u16) -> [u8; 2] {
    n.to_be_bytes()
}

impl Registers {
    pub fn af(&self) -> u16 {
        to_u16(self.a, self.f.clone().into())
    }

    pub fn set_af(&mut self, value: u16) {
        let [a, f] = to_bytes(value);
        self.a = a;
        self.f = f.into();
    }

    pub fn bc(&self) -> u16 {
        to_u16(self.b, self.c)
    }

    pub fn set_bc(&mut self, value: u16) {
        [self.b, self.c] = to_bytes(value);
    }

    pub fn de(&self) -> u16 {
        to_u16(self.d, self.e)
    }

    pub fn set_de(&mut self, value: u16) {
        [self.d, self.e] = to_bytes(value);
    }

    pub fn hl(&self) -> u16 {
        to_u16(self.h, self.l)
    }

    pub fn set_hl(&mut self, value: u16) {
        [self.h, self.l] = to_bytes(value);
    }
}

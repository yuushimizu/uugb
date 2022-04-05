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
            h: false,
            c: false,
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
            a: 0x11,
            f: Default::default(),
            b: 0x00,
            c: 0x00,
            d: 0xFF,
            e: 0x56,
            h: 0x00,
            l: 0x0D,
            pc: 0x0100,
            sp: 0xFFFE,
        }
    }
}

fn to_u16(n: u8, m: u8) -> u16 {
    (n as u16) << 8 | m as u16
}

impl Registers {
    pub fn bc(&self) -> u16 {
        to_u16(self.b, self.c)
    }

    pub fn de(&self) -> u16 {
        to_u16(self.d, self.e)
    }

    pub fn hl(&self) -> u16 {
        to_u16(self.h, self.l)
    }
}

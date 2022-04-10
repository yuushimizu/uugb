use crate::util::bits::Bits;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct InterruptSource {
    ly: bool,
    oam: bool,
    vblank: bool,
    hblank: bool,
}

impl InterruptSource {
    pub fn bits(&self) -> u8 {
        u8::from_bits(&[self.ly, self.oam, self.vblank, self.hblank])
    }

    pub fn set_bits(&mut self, value: u8) {
        self.ly = value.bit(3);
        self.oam = value.bit(2);
        self.vblank = value.bit(1);
        self.hblank = value.bit(0);
    }

    pub fn ly(&self) -> bool {
        self.ly
    }

    pub fn oam(&self) -> bool {
        self.oam
    }

    pub fn vblank(&self) -> bool {
        self.vblank
    }

    pub fn hblank(&self) -> bool {
        self.hblank
    }
}

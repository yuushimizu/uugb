use super::{Mbc, MbcContext};

#[derive(Debug)]
pub struct RomOnly {}

impl Default for RomOnly {
    fn default() -> Self {
        Self {}
    }
}

impl Mbc for RomOnly {
    fn read_rom(&self, context: &dyn MbcContext, address: u16) -> u8 {
        *context.rom().get(address as usize).unwrap_or(&0x00)
    }

    fn write_rom(&mut self, _: &mut dyn MbcContext, _: u16, _: u8) {}

    fn read_ram(&self, context: &dyn MbcContext, address: u16) -> u8 {
        *context.ram().get(address as usize).unwrap_or(&0x00)
    }

    fn write_ram(&mut self, context: &mut dyn MbcContext, address: u16, value: u8) {
        if let Some(e) = context.ram_mut().get_mut(address as usize) {
            *e = value;
        }
    }
}

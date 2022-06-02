mod mbc1;
mod mbc2;
mod mbc_context;
mod rom_only;

pub use mbc1::Mbc1;
pub use mbc2::Mbc2;
pub use mbc_context::MbcContext;
pub use rom_only::RomOnly;

use std::fmt;

pub trait Mbc
where
    Self: fmt::Debug,
{
    fn internal_ram_size(&self) -> usize {
        0
    }

    fn read_rom(&self, context: &dyn MbcContext, address: u16) -> u8;

    fn write_rom(&mut self, context: &mut dyn MbcContext, address: u16, value: u8);

    fn read_ram(&self, context: &dyn MbcContext, address: u16) -> u8;

    fn write_ram(&mut self, context: &mut dyn MbcContext, address: u16, value: u8);
}

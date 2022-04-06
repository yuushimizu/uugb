mod mbc1;
mod mbc_context;

pub use mbc1::Mbc1;
pub use mbc_context::MbcContext;

use std::fmt;

pub trait Mbc
where
    Self: fmt::Debug,
{
    fn read(&self, context: &dyn MbcContext, address: u16) -> u8;

    fn write(&mut self, context: &mut dyn MbcContext, address: u16, value: u8);
}

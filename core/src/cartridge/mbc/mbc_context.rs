pub trait MbcContext {
    fn rom(&self) -> &[u8];

    fn ram(&self) -> &[u8];

    fn ram_mut(&mut self) -> &mut [u8];
}

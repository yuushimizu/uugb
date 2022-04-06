mod joypad {}

pub struct Io {}

impl Io {
    fn read(&self, address: u16) -> u8 {
        0
    }

    fn write(&mut self, address: u16, value: u8) {}
}

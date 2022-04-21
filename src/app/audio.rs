use core;

#[derive(Debug, Default)]
pub struct AudioOutput {}

impl core::AudioTerminal for AudioOutput {
    fn output(&mut self, volume: (u8, u8)) {}
}

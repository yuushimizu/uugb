mod length;
mod rect_wave;

pub use rect_wave::SAMPLE_RATE;

use rect_wave::RectWave;

pub trait AudioTerminal {
    fn output(&mut self, volume: (u8, u8));
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Apu {
    is_enabled: bool,
    rect_wave1: RectWave,
    rect_wave2: RectWave,
}

impl Default for Apu {
    fn default() -> Self {
        Self {
            is_enabled: true,
            rect_wave1: RectWave::new(true),
            rect_wave2: RectWave::new(false),
        }
    }
}

impl Apu {
    pub fn tick(&mut self, terminal: &mut impl AudioTerminal) {
        self.rect_wave1.tick();
        self.rect_wave2.tick();
    }
}

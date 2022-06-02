mod apu;
mod cartridge;
mod cpu;
mod game_boy;
mod interrupt;
mod joypad;
mod memory;
mod ppu;
mod serial;
mod timer;

mod util;

pub use apu::{
    AudioFrame, AudioTerminal, MAX_FRAME_VOLUME as MAX_AUDIO_FRAME_VOLUME,
    SAMPLE_RATE as AUDIO_SAMPLE_RATE,
};
pub use cartridge::{Cartridge, Header};
pub use game_boy::GameBoy;
pub use joypad::ButtonState;
pub use ppu::{display_size, Color, Renderer, Vec2};
pub use serial::{NoSerialConnection, SerialConnection};

pub const CLOCK_CYCLE: u64 = 4194304;

pub const M_CYCLES: u64 = CLOCK_CYCLE / 4;

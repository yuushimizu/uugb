pub mod interrupt_controller;

pub use interrupt_controller::InterruptController;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Interrupt {
    VBlank,
    LcdStat,
    Timer,
    Serial,
    Joypad,
}

impl Interrupt {
    const ORDERED: [Interrupt; 5] = {
        use Interrupt::*;
        [VBlank, LcdStat, Timer, Serial, Joypad]
    };

    pub fn address(&self) -> u8 {
        use Interrupt::*;
        match self {
            VBlank => 0x40,
            LcdStat => 0x48,
            Timer => 0x50,
            Serial => 0x58,
            Joypad => 0x60,
        }
    }
}

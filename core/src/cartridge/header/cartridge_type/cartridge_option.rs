use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CartridgeOption {
    Ram,
    Battery,
    Timer,
    Rumble,
    Sensor,
}

impl fmt::Display for CartridgeOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CartridgeOption::*;
        write!(
            f,
            "{}",
            match self {
                Ram => "RAM",
                Battery => "BATTERY",
                Timer => "TIMER",
                Rumble => "RUMBLE",
                Sensor => "SENSOR",
            }
        )
    }
}

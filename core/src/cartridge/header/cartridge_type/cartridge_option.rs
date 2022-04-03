use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CartridgeOption {
    Ram,
    Battery,
    Rtc,
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
                Rtc => "RTC",
                Rumble => "RUMBLE",
                Sensor => "SENSOR",
            }
        )
    }
}

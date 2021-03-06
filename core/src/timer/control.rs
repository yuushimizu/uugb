use crate::util::bits::Bits;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InputClock {
    cycles: u16,
}

impl InputClock {
    pub fn bit_mask(&self) -> u16 {
        self.cycles >> 1
    }
}

pub const INPUT_CLOCKS: [InputClock; 4] = [
    InputClock { cycles: 1024 },
    InputClock { cycles: 16 },
    InputClock { cycles: 64 },
    InputClock { cycles: 256 },
];

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Control {
    is_enabled: bool,
    input_clock_index: u8,
}

impl Control {
    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    pub fn input_clock(&self) -> InputClock {
        INPUT_CLOCKS[self.input_clock_index as usize]
    }

    pub fn bits(&self) -> u8 {
        0b1111_1000 | (self.is_enabled as u8) << 2 | self.input_clock_index & 0b11
    }

    pub fn set_bits(&mut self, value: u8) {
        self.is_enabled = value.bit(2);
        self.input_clock_index = value & 0b11;
    }
}

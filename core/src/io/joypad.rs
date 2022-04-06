use crate::util::bits::Bits;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct State {
    down: bool,
    up: bool,
    left: bool,
    right: bool,
    start: bool,
    select: bool,
    b: bool,
    a: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Joypad {
    state: State,
    actions_selected: bool,
    directions_selected: bool,
}

impl Joypad {
    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    fn actions_bits(&self) -> u8 {
        !(if self.actions_selected {
            0b1 << 5
                | (self.state.start as u8) << 3
                | (self.state.select as u8) << 2
                | (self.state.b as u8) << 1
                | self.state.a as u8
        } else {
            0x00
        })
    }

    fn directions_bits(&self) -> u8 {
        !(if self.directions_selected {
            0b1 << 4
                | (self.state.down as u8) << 3
                | (self.state.up as u8) << 2
                | (self.state.left as u8) << 1
                | self.state.right as u8
        } else {
            0x00
        })
    }

    pub fn bits(&self) -> u8 {
        self.actions_bits() & self.directions_bits()
    }

    pub fn set_bits(&mut self, value: u8) {
        self.actions_selected = value.bit(5);
        self.directions_selected = value.bit(4);
    }
}

use crate::{
    interrupt::{Interrupt, InterruptController},
    util::bits::Bits,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct ButtonState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Joypad {
    button_state: ButtonState,
    action_is_selected: bool,
    direction_is_selected: bool,
}

impl Default for Joypad {
    fn default() -> Self {
        Self {
            button_state: Default::default(),
            action_is_selected: true,
            direction_is_selected: true,
        }
    }
}

impl Joypad {
    pub fn bits(&self) -> u8 {
        !((self.action_is_selected as u8) << 5
            | (self.direction_is_selected as u8) << 4
            | if self.action_is_selected {
                u8::from_bits(&[
                    self.button_state.start,
                    self.button_state.select,
                    self.button_state.b,
                    self.button_state.a,
                ])
            } else {
                0x00
            }
            | if self.direction_is_selected {
                u8::from_bits(&[
                    self.button_state.down,
                    self.button_state.up,
                    self.button_state.left,
                    self.button_state.right,
                ])
            } else {
                0x00
            })
    }

    fn with_interrupt(
        &mut self,
        interrupt_controller: &mut InterruptController,
        f: impl FnOnce(&mut Self),
    ) {
        let previous_bits = self.bits() & 0xF;
        f(self);
        let current_bits = self.bits() & 0xF;
        if (previous_bits ^ current_bits) & previous_bits != 0x00 {
            interrupt_controller.request(Interrupt::Joypad);
        }
    }

    pub fn set_bits(&mut self, value: u8, interrupt_controller: &mut InterruptController) {
        self.with_interrupt(interrupt_controller, |joypad| {
            joypad.action_is_selected = !value.bit(5);
            joypad.direction_is_selected = !value.bit(4);
        });
    }

    pub fn set_button_state(
        &mut self,
        button_state: ButtonState,
        interrupt_controller: &mut InterruptController,
    ) {
        self.with_interrupt(interrupt_controller, |joypad| {
            joypad.button_state = button_state
        });
    }
}

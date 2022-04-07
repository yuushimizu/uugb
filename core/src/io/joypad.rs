use crate::interrupt::{Interrupt, InterruptController};
use crate::util::bits::Bits;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Button {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Start,
    Select,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum ButtonCategory {
    Action,
    Direction,
}

impl ButtonCategory {
    pub fn definition(&self) -> (u8, [Button; 4]) {
        use Button::*;
        use ButtonCategory::*;
        match self {
            Action => (5, [Start, Select, B, A]),
            Direction => (4, [Down, Up, Left, Right]),
        }
    }

    pub fn bit(&self) -> u8 {
        self.definition().0
    }

    pub fn buttons(&self) -> [Button; 4] {
        self.definition().1
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
struct ButtonState {
    is_pressing: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct State {
    button_states: HashMap<Button, ButtonState>,
}

impl State {
    pub fn is_pressing(&self, button: Button) -> bool {
        self.button_states
            .get(&button)
            .map_or(false, |state| state.is_pressing)
    }

    pub fn set_pressing(&mut self, button: Button, is_pressing: bool) {
        self.button_states
            .insert(button, ButtonState { is_pressing });
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct CategoryState {
    category: ButtonCategory,
    is_selected: bool,
}

impl CategoryState {
    pub fn new(category: ButtonCategory) -> Self {
        Self {
            category,
            is_selected: false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Joypad {
    state: State,
    category_states: Vec<CategoryState>,
}

impl Default for Joypad {
    fn default() -> Self {
        use ButtonCategory::*;
        Self {
            state: Default::default(),
            category_states: vec![CategoryState::new(Action), CategoryState::new(Direction)],
        }
    }
}

impl Joypad {
    fn category_bits(&self, category_state: &CategoryState) -> u8 {
        !(if category_state.is_selected {
            let category = category_state.category;
            category
                .buttons()
                .iter()
                .enumerate()
                .fold(0b1 << category.bit(), |acc, (bit, button)| {
                    acc | (self.state.is_pressing(*button) as u8) << bit
                })
        } else {
            0x00
        })
    }

    pub fn bits(&self) -> u8 {
        self.category_states
            .iter()
            .fold(0xFF, |acc, category_state| {
                acc & self.category_bits(category_state)
            })
    }

    fn change_state<F: FnOnce(&mut Self)>(
        &mut self,
        interrupt_controller: &mut InterruptController,
        f: F,
    ) {
        let previous_bits = self.bits() & 0xF;
        f(self);
        let current_bits = self.bits() & 0xF;
        if (previous_bits ^ current_bits) & previous_bits != 0x00 {
            interrupt_controller.request(Interrupt::Joypad);
        }
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn set_state(&mut self, state: State, interrupt_controller: &mut InterruptController) {
        self.change_state(interrupt_controller, |joypad| joypad.state = state);
    }

    pub fn set_bits(&mut self, value: u8, interrupt_controller: &mut InterruptController) {
        self.change_state(interrupt_controller, |joypad| {
            for category_state in joypad.category_states.iter_mut() {
                category_state.is_selected = value.bit(category_state.category.bit() as u32);
            }
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Divider {
    counter: u16,
}

impl Divider {
    pub fn counter(&self) -> u16 {
        self.counter
    }

    pub fn tick(&mut self) {
        self.counter = self.counter.wrapping_add(1);
    }

    pub fn register(&self) -> u8 {
        (self.counter >> 8) as u8
    }

    pub fn reset(&mut self) {
        self.counter = 0;
    }
}

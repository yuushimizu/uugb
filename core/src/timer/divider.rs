#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Divider {
    counter: u16,
}

impl Default for Divider {
    fn default() -> Self {
        Self { counter: 0xAB00 }
    }
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

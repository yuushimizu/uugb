use crate::{interrupt::InterruptController, memory};

pub trait Context: memory::Context {
    fn interrupt_controller(&self) -> &InterruptController;

    fn interrupt_controller_mut(&mut self) -> &mut InterruptController;
}

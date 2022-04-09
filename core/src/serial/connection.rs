pub trait SerialConnection {
    fn send(&mut self, bit: bool);

    fn receive(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoConnection;

impl SerialConnection for NoConnection {
    fn send(&mut self, _bit: bool) {}

    fn receive(&self) -> bool {
        true
    }
}

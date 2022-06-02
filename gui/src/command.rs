use std::sync::mpsc;

#[derive(Debug, Clone)]
pub enum Command {
    Rom(Vec<u8>),
}

pub type Sender = mpsc::Sender<Command>;

pub type Receiver = mpsc::Receiver<Command>;

pub fn channels() -> (Sender, Receiver) {
    mpsc::channel()
}

use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Request {
    pub source: u16,
    pub destination: u16,
    pub length: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Dma {
    requests: VecDeque<Request>,
}

impl Dma {
    pub fn request(&mut self, source: u16, destination: u16, length: usize) {
        self.requests.push_back(Request {
            source,
            destination,
            length,
        });
    }

    pub fn pop_request(&mut self) -> Option<Request> {
        self.requests.pop_front()
    }
}

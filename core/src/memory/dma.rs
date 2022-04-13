#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Request {
    source: u16,
    destination: u16,
    length: u16,
    transfered: u16,
}

impl Request {
    pub fn next_source(&self) -> u16 {
        self.source.saturating_add(self.transfered)
    }

    pub fn next_destination(&self) -> u16 {
        self.destination.saturating_add(self.transfered)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Dma {
    running_requests: Vec<Request>,
}

impl Dma {
    pub fn request(&mut self, source: u16, destination: u16, length: u16) {
        self.running_requests.push(Request {
            source,
            destination,
            length: length,
            transfered: 0,
        });
    }

    pub fn running_requests(&self) -> &[Request] {
        &self.running_requests
    }

    pub fn advance_running_requests(&mut self) {
        for request in self.running_requests.iter_mut() {
            request.transfered = request.transfered.saturating_add(1);
        }
        self.running_requests
            .retain(|request| request.transfered < request.length);
    }
}

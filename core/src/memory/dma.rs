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
    running_request: Option<Request>,
}

impl Dma {
    pub fn request(&mut self, source: u16, destination: u16, length: u16) {
        if self.running_request.is_some() {
            return;
        }
        self.running_request = Some(Request {
            source,
            destination,
            length: length,
            transfered: 0,
        });
    }

    pub fn running_request(&self) -> Option<Request> {
        self.running_request.clone()
    }

    pub fn advance_running_request(&mut self) {
        if let Some(ref mut request) = self.running_request {
            request.transfered = request.transfered.saturating_add(1);
            if request.transfered >= request.length {
                self.running_request = None
            }
        }
    }
}

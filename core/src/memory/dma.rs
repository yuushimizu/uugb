#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Process {
    source: u16,
    destination: u16,
    length: u16,
    transfered: u16,
}

impl Process {
    pub fn next_source(&self) -> u16 {
        self.source.saturating_add(self.transfered)
    }

    pub fn next_destination(&self) -> u16 {
        self.destination.saturating_add(self.transfered)
    }

    pub fn is_finished(&self) -> bool {
        self.transfered >= self.length
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Dma {
    running_process: Option<Process>,
    queued_process: Option<Process>,
}

impl Dma {
    pub fn request(&mut self, source: u16, destination: u16, length: u16) {
        self.queued_process = Some(Process {
            source,
            destination,
            length: length,
            transfered: 0,
        });
    }

    pub fn is_running(&self) -> bool {
        self.running_process().is_some()
    }

    pub fn running_process(&self) -> &Option<Process> {
        &self.running_process
    }

    pub fn tick(&mut self) {
        if let Some(ref mut process) = self.running_process {
            process.transfered = process.transfered.saturating_add(1);
            if process.is_finished() {
                self.running_process = None;
            }
        }
        if self.queued_process.is_some() {
            self.running_process = self.queued_process.take();
        }
    }
}

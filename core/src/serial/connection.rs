use std::collections::VecDeque;

pub trait SerialConnection {
    fn receive(&mut self, bit: bool);

    fn send(&mut self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoSerialConnection;

impl SerialConnection for NoSerialConnection {
    fn receive(&mut self, _: bool) {}

    fn send(&mut self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct BufferedSerialConnection {
    send_buffer: VecDeque<u8>,
    sending_data: u8,
    sent_bits: u8,
    received_buffer: Vec<u8>,
    received_data: u8,
    received_bits: u8,
}

impl BufferedSerialConnection {
    pub fn push(&mut self, bytes: &[u8]) {
        self.send_buffer.extend(bytes);
    }

    pub fn buffer(&self) -> &Vec<u8> {
        &self.received_buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Vec<u8> {
        &mut self.received_buffer
    }
}

impl SerialConnection for BufferedSerialConnection {
    fn receive(&mut self, bit: bool) {
        self.received_data = self.received_data << 1 & (bit as u8);
        self.received_bits += 1;
        if self.received_bits >= 8 {
            self.received_bits = 0;
        }
    }

    fn send(&mut self) -> bool {
        if self.sent_bits == 0 {
            self.sending_data = self.send_buffer.pop_front().unwrap_or(0xFF);
        }
        let bit = self.sending_data >> 7 != 0;
        self.sending_data <<= 1;
        self.sent_bits += 1;
        if self.sent_bits >= 8 {
            self.sent_bits = 0;
        }
        bit
    }
}

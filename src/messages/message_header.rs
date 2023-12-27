use std::time::Instant;

pub struct MessageHeader {
    message_count: u32,
    username: String,
}

impl MessageHeader {
    pub fn new(message_count: u32, username: String) -> Self {
        Self {
            message_count,
            username,
        }
    }

    pub fn message_count(&self) -> u32 {
        self.message_count
    }

    pub fn username(&self) -> &String {
        &self.username
    }
}

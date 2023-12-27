
#[derive(PartialEq, Eq, Debug)]
pub struct MessageHeader {
    message_id: u32,
    username: String,
}

impl MessageHeader {
    pub fn new(message_id: u32, username: String) -> Self {
        Self {
            message_id,
            username,
        }
    }

    pub fn message_id(&self) -> u32 {
        self.message_id
    }

    pub fn username(&self) -> &String {
        &self.username
    }
}

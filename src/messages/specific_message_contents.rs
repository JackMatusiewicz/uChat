pub enum SpecificMessageContents {
    Message(String),
}

impl SpecificMessageContents {
    pub fn message(message: String) -> Self {
        Self::Message(message)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            SpecificMessageContents::Message(msg) => {
                let message_bytes = &msg.as_bytes()[..];
                let message_len_bytes = &u32::to_be_bytes(message_bytes.len() as u32)[..];
                let concat_bytes = [message_len_bytes, message_bytes].concat();

                concat_bytes
            }
        }
    }
}

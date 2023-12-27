use std::error::Error;

#[derive(PartialEq, Eq, Debug)]
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
                let message_len_bytes = &usize::to_be_bytes(message_bytes.len())[..];
                let concat_bytes = [message_len_bytes, message_bytes].concat();

                concat_bytes
            }
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        let message_length =
        usize::from_be_bytes(bytes[0..8].try_into()?) as usize;
        let message_start_position = 8;
        let contents =
            SpecificMessageContents::message(
                String::from_utf8_lossy(
                    &bytes[message_start_position..message_start_position + message_length])
                .to_string());
        Ok(contents)
    }
}

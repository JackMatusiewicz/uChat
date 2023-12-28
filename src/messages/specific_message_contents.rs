use std::error::Error;

#[derive(PartialEq, Eq, Debug)]
pub enum SpecificMessageContents {
    Message(String),
    Connected,
    Disconnected,
}

impl SpecificMessageContents {

    pub fn get_message_type(&self) -> u8 {
        match &self {
            &SpecificMessageContents::Message(_) => 1,
            &SpecificMessageContents::Connected => 2,
            &SpecificMessageContents::Disconnected => 3
        }
    }

    pub fn is_valid_message_type(b: u8) -> bool {
        b >= 1 && b <= 3
    }

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
            },
            _ => vec![]
        }
    }

    pub fn from_bytes(message_type: u8, bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        match message_type {
            1 => {
                let message_length =
                usize::from_be_bytes(bytes[0..8].try_into()?) as usize;
                let message_start_position = 8;
                let contents =
                    SpecificMessageContents::message(
                        String::from_utf8_lossy(
                            &bytes[message_start_position..message_start_position + message_length])
                        .to_string());
                Ok(contents)
            },
            2 => Ok(SpecificMessageContents::Connected),
            3 => Ok(SpecificMessageContents::Disconnected),
            _ => panic!("We should have failed before we get here.")
        }
    }
}

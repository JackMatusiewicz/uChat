use super::{message_header::MessageHeader, specific_message_contents::SpecificMessageContents};

pub struct Message {
    header: MessageHeader,
    contents: SpecificMessageContents,
}

impl Message {
    fn get_message_id(&self) -> u8 {
        match self.contents {
            SpecificMessageContents::Message(_) => 1,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let username_bytes = self.header.username().as_bytes();
        let username_length_bytes = &u32::to_be_bytes(username_bytes.len() as u32)[..];
        let message_count_bytes = &u32::to_be_bytes(self.header.message_count())[..];
        let message_id = &[Self::get_message_id(&self)][..];
        let message_bytes = self.contents.to_bytes();
        let concat_bytes = [
            message_id,
            message_count_bytes,
            username_length_bytes,
            username_bytes,
            &message_bytes[..],
        ]
        .concat();
        concat_bytes
    }

    fn is_valid_message_id(b: u8) -> bool {
        b == 1
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.is_empty() {
            return None;
        }

        if !Self::is_valid_message_id(bytes[0]) {
            return None;
        }
        // TODO - for now we only have one message type so we will just skip parsing specifics.
        let message_count = u32::from_be_bytes(bytes[1..5].try_into().unwrap());
        let username_length = u32::from_be_bytes(bytes[5..9].try_into().unwrap()) as usize;
        let username = String::from_utf8_lossy(&bytes[9..9+username_length]);

        let message_contents_index = 9 + username_length;
        let message_length =
            u32::from_be_bytes(bytes[message_contents_index..message_contents_index+4].try_into().unwrap()) as usize;
        let message_start_position = message_contents_index + 4;
        let header = MessageHeader::new(message_count, username.to_string());
        let contents =
            SpecificMessageContents::message(
                String::from_utf8_lossy(
                    &bytes[message_start_position..message_start_position + message_length])
                .to_string());

        Some(Self { header, contents })
    }

    pub fn username(&self) -> &String {
        self.header.username()
    }

    pub fn message_contents(&self) -> &String {
        match &self.contents {
            SpecificMessageContents::Message(msg) => msg
        }
    }

    pub fn new_message(header: MessageHeader, contents: String) -> Self {
        Self {
            header,
            contents: SpecificMessageContents::Message(contents)
        }
    }
}

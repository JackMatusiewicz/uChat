use std::error::Error;

use super::{message_header::MessageHeader, specific_message_contents::SpecificMessageContents};

use crate::errors::message_errors::MessageErrors;

#[derive(PartialEq, Eq, Debug)]
pub struct Message {
    header: MessageHeader,
    contents: SpecificMessageContents,
}

impl Message {

    pub fn to_bytes(&self) -> Vec<u8> {
        let username_bytes = self.header.username().as_bytes();
        let username_length_bytes = &usize::to_be_bytes(username_bytes.len())[..];
        let message_id_bytes = &u32::to_be_bytes(self.header.message_id())[..];
        let message_type = &[SpecificMessageContents::get_message_type(&self.contents)][..];
        let message_bytes = self.contents.to_bytes();
        let concat_bytes = [
            message_type,
            message_id_bytes,
            username_length_bytes,
            username_bytes,
            &message_bytes[..],
        ]
        .concat();
        concat_bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {

        if bytes.is_empty() || !SpecificMessageContents::is_valid_message_type(bytes[0]) {
            return Err(Box::new(MessageErrors::InvalidMessageData));
        }

        // TODO - for now we only have one message type so we will just skip parsing specifics.
        let message_id = u32::from_be_bytes(bytes[1..5].try_into()?);
        let username_length = usize::from_be_bytes(bytes[5..13].try_into()?);
        let username = String::from_utf8_lossy(&bytes[13..13+username_length]);

        let message_contents_index = 13 + username_length;
        let contents = SpecificMessageContents::from_bytes(bytes[0], &bytes[message_contents_index..])?;
        
        let header = MessageHeader::new(message_id, username.to_string());

        Ok(Self { header, contents })
    }

    pub fn username(&self) -> &String {
        self.header.username()
    }

    pub fn message_contents(&self) -> Option<&String> {
        match &self.contents {
            SpecificMessageContents::Message(msg) => Some(msg),
            _ => None
        }
    }

    pub fn new_message(header: MessageHeader, contents: String) -> Self {
        Self {
            header,
            contents: SpecificMessageContents::Message(contents)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn simple_round_trip() -> Result<(), Box<dyn Error>> {
        let message =
            Message::new_message(
                MessageHeader::new(9001, "jackma".to_owned()),
                "This is a test message".to_owned());

        let bytes = message.to_bytes();
        let new_message = Message::from_bytes(bytes.as_slice())?;

        assert_eq!(message, new_message);

        Ok(())
    }
}

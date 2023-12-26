use super::{message_header::MessageHeader, specific_message_contents::SpecificMessageContents};

pub struct Message {
    header: MessageHeader,
    contents: SpecificMessageContents
}
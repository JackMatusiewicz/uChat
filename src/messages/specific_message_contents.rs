use super::message_header::UserId;

pub enum SpecificMessageContents {
    Message(String),
    Replay(UserId),
}

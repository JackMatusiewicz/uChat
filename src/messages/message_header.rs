use std::time::Instant;

pub struct UserId(i32);

pub struct MessageHeader {
    user_id: UserId,
    username: String,
    sent_time: Instant,
}

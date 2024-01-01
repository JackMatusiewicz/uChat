use std::net::IpAddr;


#[derive(PartialEq, Eq, Debug)]
pub struct MessageHeader {
    message_id: u32,
    username: String,
    // The IP address will only be populated from messages received, not on outgoing messages.
    ip_address: Option<IpAddr>
}

impl MessageHeader {
    /// To be used for constructing message headers from received messages via UDP,
    /// as we will have access to the IP address of the sender.
    pub fn new_with_ip(message_id: u32, username: String, ip_address: IpAddr) -> Self {
        Self {
            message_id,
            username,
            ip_address: Some(ip_address)
        }
    }

    /// To be used for constructing messages that you want to send out, as you won't have access to your
    /// IP address and receivers will be able to acquire it from the message packet.
    pub fn new(message_id: u32, username: String) -> Self {
        Self {
            message_id,
            username,
            ip_address: None
        }
    }

    pub fn message_id(&self) -> u32 {
        self.message_id
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn ip_address(&self) -> Option<String> {
        self.ip_address.as_ref().map(|c| c.to_string())
    }
}

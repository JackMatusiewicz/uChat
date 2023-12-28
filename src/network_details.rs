use std::{
    sync::mpsc::{Receiver, Sender},
    thread::JoinHandle,
};

use crate::messages::message::Message;

pub struct NetworkDetails {
    pub network_message_receiver: Receiver<Message>,
    pub send_message_to_network: Sender<Message>,
    pub send_to_network_handle: JoinHandle<()>,
    pub receive_from_network_handle: JoinHandle<()>,
}

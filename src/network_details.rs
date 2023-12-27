use std::{
    any::Any,
    sync::mpsc::{Receiver, Sender},
    thread::JoinHandle,
};

pub struct NetworkDetails {
    pub network_message_receiver: Receiver<String>,
    pub send_message_to_network: Sender<String>,
    pub send_to_network_handle: JoinHandle<()>,
    pub receive_from_network_handle: JoinHandle<()>,
}

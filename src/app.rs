use eframe::egui;
use egui::FontId;
use std::{sync::{atomic::AtomicBool, mpsc::{Receiver, Sender}, Arc}, thread::JoinHandle, any::Any};

use crate::network_details::NetworkDetails;

pub struct App {
    is_finished: Arc<AtomicBool>,
    details: Option<NetworkDetails>,
    username: String,
    seen_messages: Vec<String>,
    current_message: String
}

impl App {
    pub fn new (is_finished: Arc<AtomicBool>, details: NetworkDetails) -> Self {
        Self {
            is_finished,
            details: Some(details),
            username: "".to_owned(),
            seen_messages: vec![],
            current_message: "".to_owned()
        }
    }
}

impl eframe::App for App {
    fn on_exit(&mut self, ctx: Option<&eframe::glow::Context>) {
        self.is_finished.store(true, std::sync::atomic::Ordering::Relaxed);
        let details = self.details.take().unwrap();
        drop(details.network_message_receiver);
        drop(details.send_message_to_network);

        let _ = details.send_to_network_handle.join();
        let _ = details.receive_from_network_handle.join();
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Each update we try to pull in another message from the network.
        match self.details.as_ref().unwrap().network_message_receiver.try_recv() {
            Ok(msg) => {
                self.seen_messages.push(msg)
            },
            Err(_) => {}
        }

        egui::TopBottomPanel::top("username").show(ctx, |ui| {
            let widget = egui::TextEdit::singleline(&mut self.username)
            .desired_width(f32::INFINITY)
            .hint_text("Enter your username here.")
            .font(FontId::proportional(16.0))
            .margin(egui::vec2(8.0, 8.0));
            ui.add(widget);
        });

        // Now we draw the UI and potentially send a message if we have one.
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Messages");
            ui.label("N.B: Any data sent to the multicast address will be shown here.");
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                for line in self.seen_messages.iter() {
                    let (addr, msg) = line.split_once(' ').unwrap();
                    ui.horizontal(|ui| {
                        use egui::RichText;
                        ui.label(RichText::new(addr).font(FontId::monospace(13.0)).color(egui::Color32::GOLD));
                        ui.label(RichText::new(msg).font(FontId::monospace(13.0)));
                    });
                }
            })
        });

        egui::TopBottomPanel::bottom("user-input").show(ctx, |ui| {
            let widget = egui::TextEdit::singleline(&mut self.current_message)
                .desired_width(f32::INFINITY)
                .hint_text("Press Enter to send the message")
                .font(FontId::proportional(16.0))
                .margin(egui::vec2(8.0, 8.0));
            if ui.add(widget).lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                let message_to_send = self.username.clone() + ": " + &self.current_message;
                self.current_message = "".to_owned();
                self.details.as_ref().unwrap().send_message_to_network.send(message_to_send).expect("receiver closed");
            }
        });
    }
}
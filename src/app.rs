use eframe::egui;
use egui::{FontId, RichText};
use std::sync::{atomic::AtomicBool, Arc};

use crate::{network_details::NetworkDetails, messages::{message::Message, message_header::MessageHeader}};

pub struct App {
    message_count: u32,
    is_finished: Arc<AtomicBool>,
    details: Option<NetworkDetails>,
    username: String,
    seen_messages: Vec<Message>,
    current_message: String,
}

impl App {
    pub fn new(is_finished: Arc<AtomicBool>, details: NetworkDetails) -> Self {
        Self {
            message_count: 0,
            is_finished,
            details: Some(details),
            username: "".to_owned(),
            seen_messages: vec![],
            current_message: "".to_owned(),
        }
    }

    pub fn publish_message(&mut self) {
        let message_header = MessageHeader::new(self.message_count, self.username.clone());
        self.message_count += 1;
        let message = Message::new_message(message_header, self.current_message.clone());

        self.current_message = "".to_owned();
        self.details
            .as_ref()
            .unwrap()
            .send_message_to_network
            .send(message)
            .expect("receiver closed");
    }
}

impl eframe::App for App {
    fn on_exit(&mut self, _ctx: Option<&eframe::glow::Context>) {
        self.is_finished
            .store(true, std::sync::atomic::Ordering::Relaxed);
        let details = self.details.take().unwrap();
        drop(details.network_message_receiver);
        drop(details.send_message_to_network);

        let _ = details.send_to_network_handle.join();
        let _ = details.receive_from_network_handle.join();
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Each update we try to pull in another message from the network.
        match self
            .details
            .as_ref()
            .unwrap()
            .network_message_receiver
            .try_recv()
        {
            Ok(msg) => self.seen_messages.push(msg),
            Err(_) => {}
        }

        egui::TopBottomPanel::top("username").show(ctx, |ui| {
            let widget = egui::TextEdit::singleline(&mut self.username)
                .desired_width(f32::INFINITY)
                .hint_text("Enter your username here.")
                .font(FontId::proportional(16.0))
                .margin(egui::vec2(8.0, 8.0));
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Username:".to_owned())
                        .font(FontId::monospace(13.0))
                        .color(egui::Color32::GOLD)
                        .line_height(Some(1.0)),
                );
                ui.add(widget);
            });
        });

        // Now we draw the UI and potentially send a message if we have one.
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Messages");
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                for line in self.seen_messages.iter() {
                    if let Some(message) = line.message_contents() {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(line.username())
                                    .font(FontId::monospace(13.0))
                                    .color(egui::Color32::GOLD),
                            );
                            ui.label(RichText::new(message).font(FontId::monospace(13.0)));
                        });
                    }
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
                self.publish_message();
            }
        });
    }
}

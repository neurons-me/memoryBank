//memorybank-gui/src/main.rs
use eframe::egui::{self, ComboBox, RichText, Layout, Align};
use std::os::unix::net::UnixStream;
use std::io::{Write, BufReader, BufRead};
use serde::{Serialize, Deserialize};

const SOCKET_PATH: &str = "/tmp/memorybank.sock";

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
struct Config {
    modifier_1: String,
    modifier_2: String,
    modifier_3: String,
    paste_modifier_1: String,
    paste_modifier_2: String,
    paste_modifier_3: String,
    is_enabled: bool,
}

fn fetch_config_from_daemon() -> Option<Config> {
    let mut stream = UnixStream::connect(SOCKET_PATH).ok()?;
    let mut reader = BufReader::new(stream.try_clone().ok()?);
    let _ = stream.write_all(b"{\"type\":\"get_config\"}\n").ok()?;

    let mut response = String::new();
    reader.read_line(&mut response).ok()?;
    serde_json::from_str::<Config>(&response).ok()
}

fn send_config_to_daemon(cfg: &Config) {
    if let Ok(mut stream) = UnixStream::connect(SOCKET_PATH) {
        if let Ok(json) = serde_json::to_string(&serde_json::json!({
            "type": "update_config",
            "data": cfg
        })) {
            let _ = stream.write_all(json.as_bytes());
            let _ = stream.write_all(b"\n");
        }
    }
}

#[derive(Default)]
struct MemoryBankApp {
    config: Config,
    last_sent_config: Config,
}

impl eframe::App for MemoryBankApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("MemoryBank Shortcut Preferences");
                ui.hyperlink_to("Help", "https://neurons.me")
                    .on_hover_text("More info at neurons.me");
            });

            ui.separator();
            ui.label("Select your preferred modifier keys:");

            let all_modifiers = vec!["Command", "Control", "Option", "Shift", "None"];

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("Copy").strong());
                    ui.push_id("copy", |ui| {
                        ComboBox::from_id_source("key_1")
                            .selected_text(&self.config.modifier_1)
                            .show_ui(ui, |ui| {
                                for &modifier in &all_modifiers {
                                    if modifier != self.config.modifier_2 && modifier != self.config.modifier_3 {
                                        ui.selectable_value(&mut self.config.modifier_1, modifier.to_string(), modifier);
                                    }
                                }
                            });

                        ComboBox::from_id_source("key_2")
                            .selected_text(&self.config.modifier_2)
                            .show_ui(ui, |ui| {
                                for &modifier in &all_modifiers {
                                    if modifier != self.config.modifier_1 && modifier != self.config.modifier_3 {
                                        ui.selectable_value(&mut self.config.modifier_2, modifier.to_string(), modifier);
                                    }
                                }
                            });

                        ComboBox::from_id_source("key_3")
                            .selected_text(&self.config.modifier_3)
                            .show_ui(ui, |ui| {
                                for &modifier in &all_modifiers {
                                    if modifier != self.config.modifier_1 && modifier != self.config.modifier_2 {
                                        ui.selectable_value(&mut self.config.modifier_3, modifier.to_string(), modifier);
                                    }
                                }
                            });
                    });
                });

                ui.vertical(|ui| {
                    ui.label(RichText::new("Paste").strong());
                    ui.push_id("paste", |ui| {
                        ComboBox::from_id_source("key_1")
                            .selected_text(&self.config.paste_modifier_1)
                            .show_ui(ui, |ui| {
                                for &modifier in &all_modifiers {
                                    if modifier != self.config.paste_modifier_2 && modifier != self.config.paste_modifier_3 {
                                        ui.selectable_value(&mut self.config.paste_modifier_1, modifier.to_string(), modifier);
                                    }
                                }
                            });

                        ComboBox::from_id_source("key_2")
                            .selected_text(&self.config.paste_modifier_2)
                            .show_ui(ui, |ui| {
                                for &modifier in &all_modifiers {
                                    if modifier != self.config.paste_modifier_1 && modifier != self.config.paste_modifier_3 {
                                        ui.selectable_value(&mut self.config.paste_modifier_2, modifier.to_string(), modifier);
                                    }
                                }
                            });

                        ComboBox::from_id_source("key_3")
                            .selected_text(&self.config.paste_modifier_3)
                            .show_ui(ui, |ui| {
                                for &modifier in &all_modifiers {
                                    if modifier != self.config.paste_modifier_1 && modifier != self.config.paste_modifier_2 {
                                        ui.selectable_value(&mut self.config.paste_modifier_3, modifier.to_string(), modifier);
                                    }
                                }
                            });
                    });
                });
            });

            ui.separator();

            let mut parts = vec![
                self.config.modifier_1.clone(),
                self.config.modifier_2.clone(),
                self.config.modifier_3.clone(),
            ];
            parts.retain(|p| p != "None");
            let shortcut = format!("Copy: {} + Number [0-9]", parts.join(" + "));
            ui.label(RichText::new("Current shortcut format:").strong());
            ui.label(RichText::new(shortcut));

            let mut paste_parts = vec![
                self.config.paste_modifier_1.clone(),
                self.config.paste_modifier_2.clone(),
                self.config.paste_modifier_3.clone(),
            ];
            paste_parts.retain(|p| p != "None");
            let paste_shortcut = format!("Paste: {} + Number [0-9]", paste_parts.join(" + "));
            ui.label(RichText::new(paste_shortcut));

            if parts == paste_parts {
                ui.colored_label(egui::Color32::RED, "⚠️ Copy y Paste no pueden tener la misma combinación");
            }

            ui.separator();
            ui.checkbox(&mut self.config.is_enabled, "Enable MemoryBank");

            if self.config != self.last_sent_config {
                send_config_to_daemon(&self.config);
                self.last_sent_config = self.config.clone();
            }

            ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
                ui.separator();
                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    ui.hyperlink_to("by neurons.me", "https://neurons.me");
                });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let config = fetch_config_from_daemon().unwrap_or_default();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(460.0, 300.0)),
        ..Default::default()
    };

    eframe::run_native(
        "MemoryBank GUI",
        options,
        Box::new(|_cc| Box::new(MemoryBankApp {
            last_sent_config: config.clone(),
            config,
        })),
    )
}
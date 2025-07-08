mod modifiers;
use modifiers::modifier_selector_ui;
mod validation;
use validation::validate_combinations;
mod config;
use config::{Config, fetch_config_from_daemon, send_config_to_daemon};
use eframe::egui::{self, RichText, Layout, Align, Color32};

#[derive(Default)]
struct MemoryBankApp {
    config: Config,
    last_sent_config: Config,
}

impl eframe::App for MemoryBankApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.heading(
                    RichText::new("MemoryBank")
                        .color(Color32::from_rgb(220, 220, 220))
                        .size(20.0)
                        .strong()
                );
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    ui.hyperlink_to(RichText::new("Help").color(Color32::from_rgb(120, 170, 255)), "https://neurons.me")
                        .on_hover_text("More info at neurons.me");
                });
            });

            ui.separator();
            ui.label("Configure your key combination preferences:");
            ui.separator();

            ui.horizontal(|ui| {
                let _copy_modifiers = [&self.config.modifier_1, &self.config.modifier_2, &self.config.modifier_3];
                ui.add_space(24.0);
                ui.vertical(|ui| {
                    ui.label(RichText::new("Copy").strong());
                    ui.push_id("copy", |ui| {
                        modifier_selector_ui(
                            ui,
                            &mut [
                                &mut self.config.modifier_1,
                                &mut self.config.modifier_2,
                                &mut self.config.modifier_3,
                            ],
                            "copy_mod",
                            &[
                                self.config.paste_modifier_1.clone(),
                                self.config.paste_modifier_2.clone(),
                                self.config.paste_modifier_3.clone(),
                            ],
                        );
                    });
                    ui.label("+ Number [0-9]");
                });

                ui.add_space(16.0); // separación entre columnas
                ui.separator();      // línea divisoria vertical
                ui.add_space(16.0);

                ui.vertical(|ui| {
                    ui.label(RichText::new("Paste").strong());
                    ui.push_id("paste", |ui| {
                        modifier_selector_ui(
                            ui,
                            &mut [
                                &mut self.config.paste_modifier_1,
                                &mut self.config.paste_modifier_2,
                                &mut self.config.paste_modifier_3,
                            ],
                            "paste_mod",
                            &[
                                self.config.modifier_1.clone(),
                                self.config.modifier_2.clone(),
                                self.config.modifier_3.clone(),
                            ],
                        );
                    });
                    ui.label("+ Number [0-9]");
                });
            });

            for error in validate_combinations(
                [
                    &self.config.modifier_1,
                    &self.config.modifier_2,
                    &self.config.modifier_3,
                ],
                [
                    &self.config.paste_modifier_1,
                    &self.config.paste_modifier_2,
                    &self.config.paste_modifier_3,
                ],
            ) {
                ui.colored_label(Color32::RED, error);
            }

            ui.separator();

            if ui.button("Apply").clicked() {
                send_config_to_daemon(&self.config);
                self.last_sent_config = self.config.clone();
            }

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.config.is_enabled, "Enable MemoryBank");

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.hyperlink_to(
                        RichText::new("by neurons.me").color(Color32::from_rgb(130, 130, 150)),
                        "https://neurons.me",
                    );
                });
            });
        });
    }
}


fn main() -> eframe::Result<()> {
    let config = fetch_config_from_daemon().unwrap_or_default();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(377.0, 244.0))
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "",
        options,
        Box::new(|_cc| Box::new(MemoryBankApp {
            last_sent_config: config.clone(),
            config,
        })),
    )
}
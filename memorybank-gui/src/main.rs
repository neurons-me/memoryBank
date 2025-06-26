use eframe::egui::{self, ComboBox, RichText, Checkbox, Image, Layout, Align, Align2};

#[derive(Default)]
struct MemoryBankApp {
    modifier_1: String,
    modifier_2: String,
    modifier_3: String,
    is_enabled: bool,
}

impl eframe::App for MemoryBankApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("MemoryBank Shortcut Preferences");
            ui.separator();

            ui.label("Select your preferred modifier keys:");

            let all_modifiers = vec!["Command", "Control", "Option", "Shift", "None"];

            ComboBox::from_label("Choose Key 1")
                .selected_text(&self.modifier_1)
                .show_ui(ui, |ui| {
                    for &modifier in &all_modifiers {
                        if modifier != self.modifier_2 && modifier != self.modifier_3 {
                            ui.selectable_value(&mut self.modifier_1, modifier.to_string(), modifier);
                        }
                    }
                });

            ComboBox::from_label("Choose Key 2")
                .selected_text(&self.modifier_2)
                .show_ui(ui, |ui| {
                    for &modifier in &all_modifiers {
                        if modifier != self.modifier_1 && modifier != self.modifier_3 {
                            ui.selectable_value(&mut self.modifier_2, modifier.to_string(), modifier);
                        }
                    }
                });

            ComboBox::from_label("Choose Key 3")
                .selected_text(&self.modifier_3)
                .show_ui(ui, |ui| {
                    for &modifier in &all_modifiers {
                        if modifier != self.modifier_1 && modifier != self.modifier_2 {
                            ui.selectable_value(&mut self.modifier_3, modifier.to_string(), modifier);
                        }
                    }
                });

            ui.separator();

            let mut parts = vec![self.modifier_1.clone(), self.modifier_2.clone(), self.modifier_3.clone()];
            parts.retain(|p| p != "None");

            let shortcut = format!("{} + Number [0-9]", parts.join(" + "));
            ui.label(RichText::new("Current shortcut format:").strong());
            ui.label(RichText::new(shortcut));

            ui.separator();
            ui.checkbox(&mut self.is_enabled, "Enable MemoryBank");

            ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
                ui.separator();
                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    ui.label(RichText::new("by neurons.me").italics());
                });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(360.0, 220.0)),
        ..Default::default()
    };
    eframe::run_native(
        "MemoryBank GUI",
        options,
        Box::new(|_cc| Box::new(MemoryBankApp {
            modifier_1: "Command".to_string(),
            modifier_2: "Shift".to_string(),
            modifier_3: "None".to_string(),
            is_enabled: true,
        })),
    )
}

//  by suiGn
//  neurons.me
use eframe::egui;

pub fn modifier_selector_ui(
    ui: &mut egui::Ui,
    modifiers: &mut [&mut String],
    id_prefix: &str,
    _blocked_options: &[String],
) {
    let all_modifiers = vec!["Command", "Control", "Option", "Shift", "None"];

    let current_vals: Vec<String> = modifiers.iter().map(|m| (*(*m)).clone()).collect();

    for (i, modifier) in modifiers.iter_mut().enumerate() {
        let id = format!("{}_{}", id_prefix, i);
        let all = all_modifiers.clone();

        egui::ComboBox::from_id_source(id)
            .selected_text((*modifier).clone())
            .show_ui(ui, {
                let current_vals = current_vals.clone();
                move |ui| {
                    for &candidate in &all {
                        let already_selected = current_vals
                            .iter()
                            .enumerate()
                            .any(|(j, val)| j != i && val == candidate);
                        if !already_selected {
                            ui.selectable_value(*modifier, candidate.to_string(), candidate);
                        }
                    }
                }
            });
    }
}
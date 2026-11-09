use crate::models::{Instance, Page};
use crate::theme::{self, ACCENT, TEXT_PRIMARY, TEXT_SECONDARY};
use eframe::egui;

pub fn show(
    ui: &mut egui::Ui,
    instances: &[Instance],
    selected: &mut Option<usize>,
    current_page: &mut Page,
) {
    ui.heading(egui::RichText::new("🌍  Мои инстансы").size(28.0).color(TEXT_PRIMARY));
    ui.add_space(8.0);
    ui.label(egui::RichText::new("Выберите инстанс или создайте новый").color(TEXT_SECONDARY));
    ui.add_space(16.0);

    // Quick launch card
    theme::card_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("Быстрый запуск").strong().size(18.0));
                ui.label("Последний инстанс или рекомендованная версия");
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if theme::accent_button(ui, "▶  Играть").clicked() {
                    // TODO: launch last instance
                }
            });
        });
    });

    ui.add_space(20.0);

    // Instances grid
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            for (i, inst) in instances.iter().enumerate() {
                let is_selected = *selected == Some(i);
                let frame = if is_selected {
                    egui::Frame::none()
                        .fill(ACCENT.linear_multiply(0.15))
                        .rounding(egui::Rounding::same(12.0))
                        .stroke(egui::Stroke::new(2.0, ACCENT))
                        .inner_margin(egui::Margin::same(12.0))
                } else {
                    theme::card_frame()
                };

                frame.show(ui, |ui| {
                    ui.set_min_size(egui::vec2(220.0, 140.0));
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(&inst.name).strong().size(16.0));
                        ui.label(egui::RichText::new(format!("Версия: {}", inst.version_id)).color(TEXT_SECONDARY));
                        ui.label(egui::RichText::new(format!("RAM: {} MB", inst.memory_mb)).color(TEXT_SECONDARY));
                        ui.add_space(8.0);
                        if ui.button("Выбрать").clicked() {
                            *selected = Some(i);
                        }
                    });
                });
                ui.add_space(8.0);
            }

            // New instance card
            theme::card_frame().show(ui, |ui| {
                ui.set_min_size(egui::vec2(220.0, 140.0));
                ui.centered_and_justified(|ui| {
                    if ui.button(egui::RichText::new("+ Новый инстанс").size(16.0)).clicked() {
                        *current_page = Page::Versions;
                    }
                });
            });
        });
    });
}

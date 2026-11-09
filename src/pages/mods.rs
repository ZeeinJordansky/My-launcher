use crate::theme::{self, TEXT_PRIMARY, TEXT_SECONDARY};
use eframe::egui;

pub fn show(ui: &mut egui::Ui, search: &mut String) {
    ui.heading(egui::RichText::new("📦  Каталог модов").size(28.0).color(TEXT_PRIMARY));
    ui.add_space(4.0);
    ui.label(egui::RichText::new("Modrinth / CurseForge (заглушка)").color(TEXT_SECONDARY));
    ui.add_space(12.0);

    ui.horizontal(|ui| {
        ui.label("Поиск модов:");
        ui.text_edit_singleline(search);
        if theme::accent_button(ui, "Найти").clicked() {
            // TODO: query Modrinth API
        }
    });

    ui.add_space(16.0);

    // Placeholder cards
    for name in ["Sodium", "Iris Shaders", "Fabric API", "Lithium", "FerriteCore", "Mod Menu"] {
        theme::card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new(name).strong().size(16.0));
                    ui.label(egui::RichText::new("Производительность / Оптимизация").color(TEXT_SECONDARY));
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Установить").clicked() {
                        // TODO
                    }
                });
            });
        });
        ui.add_space(6.0);
    }

    ui.add_space(12.0);
    ui.label("Полная интеграция с Modrinth API и установкой в инстанс — в следующих версиях.");
}

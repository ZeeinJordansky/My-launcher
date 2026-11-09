use crate::theme::{self, TEXT_PRIMARY, TEXT_SECONDARY};
use eframe::egui;

pub fn show(ui: &mut egui::Ui, username: &str) {
    ui.heading(egui::RichText::new("🦹  Редактор скинов").size(28.0).color(TEXT_PRIMARY));
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        // Preview placeholder
        theme::card_frame().show(ui, |ui| {
            ui.set_min_size(egui::vec2(200.0, 320.0));
            ui.centered_and_justified(|ui| {
                ui.label(egui::RichText::new("3D Preview\n(скоро)").size(18.0).color(TEXT_SECONDARY));
            });
        });

        ui.add_space(20.0);

        ui.vertical(|ui| {
            ui.label(egui::RichText::new(format!("Игрок: {}", username)).strong());
            ui.add_space(12.0);

            if theme::accent_button(ui, "Загрузить скин (.png)").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("PNG", &["png"])
                    .pick_file()
                {
                    // TODO: validate 64x64 / 64x32 skin and apply
                    let _ = path;
                }
            }

            ui.add_space(8.0);
            if ui.button("Сбросить на Steve").clicked() {
                // TODO
            }
            if ui.button("Сбросить на Alex").clicked() {
                // TODO
            }

            ui.add_space(16.0);
            ui.label("Поддерживаются классические и slim модели.");
            ui.label("Загрузка на сервера Mojang требует авторизации.");
        });
    });
}

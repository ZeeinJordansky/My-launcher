use crate::models::MinecraftVersion;
use crate::theme::{self, TEXT_PRIMARY, TEXT_SECONDARY, SUCCESS};
use eframe::egui;

pub fn show(
    ui: &mut egui::Ui,
    versions: &[MinecraftVersion],
    filter: &mut String,
    show_snapshots: &mut bool,
    loading: bool,
    latest_release: &str,
) {
    ui.heading(egui::RichText::new("📦  Версии Minecraft").size(28.0).color(TEXT_PRIMARY));
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.label("Поиск:");
        ui.text_edit_singleline(filter);
        ui.checkbox(show_snapshots, "Показывать снапшоты");
        if loading {
            ui.spinner();
            ui.label("Загрузка манифеста...");
        }
    });

    ui.add_space(12.0);

    if !latest_release.is_empty() {
        theme::card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(format!("Последний релиз: {}", latest_release)).strong());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if theme::accent_button(ui, "Установить").clicked() {
                        // TODO: install latest
                    }
                });
            });
        });
        ui.add_space(12.0);
    }

    egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
        for v in versions.iter().filter(|v| {
            let matches_filter = filter.is_empty() || v.id.to_lowercase().contains(&filter.to_lowercase());
            let type_ok = *show_snapshots || v.version_type == "release";
            matches_filter && type_ok
        }) {
            theme::card_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(&v.id).strong().size(16.0));
                        ui.label(
                            egui::RichText::new(format!("{} • {}", v.version_type, &v.release_time[..10.min(v.release_time.len())]))
                                .color(TEXT_SECONDARY)
                                .size(13.0),
                        );
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Скачать / Установить").clicked() {
                            // TODO
                        }
                        if v.version_type == "release" {
                            ui.colored_label(SUCCESS, "●");
                        }
                    });
                });
            });
            ui.add_space(6.0);
        }
    });
}

use crate::models::LauncherSettings;
use crate::theme::{self, TEXT_PRIMARY, TEXT_SECONDARY};
use eframe::egui;

pub fn show(ui: &mut egui::Ui, settings: &mut LauncherSettings) {
    ui.heading(egui::RichText::new("⚙️  Настройки").size(28.0).color(TEXT_PRIMARY));
    ui.add_space(16.0);

    theme::card_frame().show(ui, |ui| {
        ui.label(egui::RichText::new("Общие").strong().size(18.0));
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Язык:");
            egui::ComboBox::from_id_source("lang")
                .selected_text(&settings.language)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut settings.language, "ru".into(), "Русский");
                    ui.selectable_value(&mut settings.language, "en".into(), "English");
                });
        });

        ui.checkbox(&mut settings.close_after_launch, "Закрывать лаунчер после запуска игры");
        ui.checkbox(&mut settings.keep_launcher_open, "Оставлять лаунчер открытым");
        ui.checkbox(&mut settings.show_snapshots, "Показывать снапшоты по умолчанию");
        ui.checkbox(&mut settings.show_old_versions, "Показывать старые версии (alpha/beta)");
    });

    ui.add_space(12.0);

    theme::card_frame().show(ui, |ui| {
        ui.label(egui::RichText::new("Java & Память").strong().size(18.0));
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Макс. память (MB):");
            ui.add(egui::Slider::new(&mut settings.max_memory_mb, 1024..=16384).logarithmic(true));
        });

        ui.horizontal(|ui| {
            ui.label("Путь к Java:");
            let mut path = settings.java_path.clone().unwrap_or_default();
            if ui.text_edit_singleline(&mut path).changed() {
                settings.java_path = if path.is_empty() { None } else { Some(path) };
            }
            if ui.button("Обзор...").clicked() {
                if let Some(p) = rfd::FileDialog::new().pick_file() {
                    settings.java_path = Some(p.to_string_lossy().to_string());
                }
            }
        });
    });

    ui.add_space(12.0);

    theme::card_frame().show(ui, |ui| {
        ui.label(egui::RichText::new("Директория игры").strong().size(18.0));
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut settings.game_directory);
            if ui.button("Обзор...").clicked() {
                if let Some(p) = rfd::FileDialog::new().pick_folder() {
                    settings.game_directory = p.to_string_lossy().to_string();
                }
            }
        });
    });

    ui.add_space(20.0);
    if theme::accent_button(ui, "Сохранить настройки").clicked() {
        // TODO: persist to disk
    }
}

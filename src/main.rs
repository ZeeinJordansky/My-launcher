#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod theme;
mod pages;
mod models;
mod services;

use eframe::egui;

fn main() -> eframe::Result<()> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([900.0, 600.0])
            .with_title("My Launcher")
            .with_decorations(true)
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "My Launcher",
        options,
        Box::new(|cc| {
            // Apply custom fonts / style
            theme::setup_custom_style(&cc.egui_ctx);
            Ok(Box::new(app::LauncherApp::new(cc)))
        }),
    )
}

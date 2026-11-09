use crate::models::{Instance, LauncherSettings, MinecraftVersion, Page, UserProfile};
use crate::pages;
use crate::services;
use crate::theme::{self, ACCENT, BG_SIDEBAR, TEXT_PRIMARY, TEXT_SECONDARY};
use eframe::egui;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub struct LauncherApp {
    current_page: Page,
    settings: LauncherSettings,
    profile: UserProfile,
    instances: Vec<Instance>,
    selected_instance: Option<usize>,
    versions: Vec<MinecraftVersion>,
    latest_release: String,
    latest_snapshot: String,
    versions_loading: bool,
    version_filter: String,
    show_snapshots: bool,
    mod_search: String,
    // Shared state for async
    version_fetch_result: Arc<Mutex<Option<Result<(String, String, Vec<MinecraftVersion>), String>>>>,
}

impl LauncherApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut instances = vec![];
        // Demo instance
        instances.push(Instance {
            id: Uuid::new_v4(),
            name: "Vanilla 1.21".into(),
            version_id: "1.21.1".into(),
            java_path: None,
            memory_mb: 4096,
            jvm_args: "-XX:+UseG1GC".into(),
            game_dir: String::new(),
            last_played: None,
            icon: None,
        });

        let app = Self {
            current_page: Page::Home,
            settings: LauncherSettings::default(),
            profile: UserProfile {
                username: "Player".into(),
                uuid: None,
                access_token: None,
                skin_url: None,
                offline: true,
            },
            instances,
            selected_instance: Some(0),
            versions: vec![],
            latest_release: String::new(),
            latest_snapshot: String::new(),
            versions_loading: false,
            version_filter: String::new(),
            show_snapshots: false,
            mod_search: String::new(),
            version_fetch_result: Arc::new(Mutex::new(None)),
        };

        // Trigger initial version fetch in background (simplified)
        // In real app use tokio runtime properly

        app
    }

    fn sidebar(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add_space(12.0);
            ui.heading(egui::RichText::new("My Launcher").color(ACCENT).size(22.0));
            ui.label(egui::RichText::new("v0.1.0").color(TEXT_SECONDARY).size(12.0));
            ui.add_space(20.0);

            self.nav_button(ui, "🏠  Главная", Page::Home);
            self.nav_button(ui, "📦  Версии", Page::Versions);
            self.nav_button(ui, "📦  Моды", Page::Mods);
            self.nav_button(ui, "🦹  Скины", Page::Skins);
            self.nav_button(ui, "👤  Аккаунты", Page::Accounts);
            self.nav_button(ui, "⚙️  Настройки", Page::Settings);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.add_space(12.0);
                ui.label(egui::RichText::new(format!("👤 {}", self.profile.username)).color(TEXT_PRIMARY));
                if self.profile.offline {
                    ui.label(egui::RichText::new("Оффлайн-режим").color(TEXT_SECONDARY).size(12.0));
                }
            });
        });
    }

    fn nav_button(&mut self, ui: &mut egui::Ui, label: &str, page: Page) {
        let selected = self.current_page == page;
        let text = if selected {
            egui::RichText::new(label).color(ACCENT).strong()
        } else {
            egui::RichText::new(label).color(TEXT_PRIMARY)
        };

        let btn = egui::Button::new(text)
            .fill(if selected {
                ACCENT.linear_multiply(0.15)
            } else {
                egui::Color32::TRANSPARENT
            })
            .rounding(egui::Rounding::same(8.0))
            .min_size(egui::vec2(180.0, 36.0));

        if ui.add(btn).clicked() {
            self.current_page = page;
            if self.current_page == Page::Versions && self.versions.is_empty() && !self.versions_loading {
                self.fetch_versions();
            }
        }
        ui.add_space(4.0);
    }

    fn fetch_versions(&mut self) {
        self.versions_loading = true;
        let result_slot = Arc::clone(&self.version_fetch_result);

        // Spawn blocking for simplicity in this skeleton
        // Real app should use proper async runtime integration with egui
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let res = rt.block_on(services::versions::fetch_version_manifest());
            if let Ok(mut guard) = result_slot.lock() {
                *guard = Some(res);
            }
        });
    }

    fn poll_version_fetch(&mut self) {
        if let Ok(mut guard) = self.version_fetch_result.lock() {
            if let Some(res) = guard.take() {
                self.versions_loading = false;
                match res {
                    Ok((release, snapshot, vers)) => {
                        self.latest_release = release;
                        self.latest_snapshot = snapshot;
                        self.versions = vers;
                    }
                    Err(e) => {
                        log::error!("Failed to fetch versions: {}", e);
                    }
                }
            }
        }
    }
}

impl eframe::App for LauncherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_version_fetch();

        // Sidebar
        egui::SidePanel::left("sidebar")
            .exact_width(220.0)
            .resizable(false)
            .frame(egui::Frame::none().fill(BG_SIDEBAR).inner_margin(egui::Margin::same(12.0)))
            .show(ctx, |ui| {
                self.sidebar(ui);
            });

        // Main content
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(theme::BG_DARK).inner_margin(egui::Margin::same(20.0)))
            .show(ctx, |ui| {
                match self.current_page {
                    Page::Home => {
                        pages::home::show(
                            ui,
                            &self.instances,
                            &mut self.selected_instance,
                            &mut self.current_page,
                        );
                    }
                    Page::Versions => {
                        pages::versions::show(
                            ui,
                            &self.versions,
                            &mut self.version_filter,
                            &mut self.show_snapshots,
                            self.versions_loading,
                            &self.latest_release,
                        );
                    }
                    Page::Mods => {
                        pages::mods::show(ui, &mut self.mod_search);
                    }
                    Page::Skins => {
                        pages::skins::show(ui, &self.profile.username);
                    }
                    Page::Settings => {
                        pages::settings::show(ui, &mut self.settings);
                    }
                    Page::Accounts => {
                        ui.heading(egui::RichText::new("👤  Аккаунты").size(28.0));
                        ui.add_space(12.0);
                        theme::card_frame().show(ui, |ui| {
                            ui.label(format!("Текущий: {} ({})", self.profile.username, if self.profile.offline { "оффлайн" } else { "онлайн" }));
                            ui.add_space(8.0);
                            if theme::accent_button(ui, "Войти через Microsoft").clicked() {
                                // TODO
                            }
                            if ui.button("Оффлайн-режим").clicked() {
                                let name = self.profile.username.clone();
                                let res = services::auth::offline_login(&name);
                                self.profile.username = res.username;
                                self.profile.uuid = Some(res.uuid);
                                self.profile.offline = true;
                            }
                        });
                    }
                }
            });

        // Request repaint if loading
        if self.versions_loading {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }
    }
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftVersion {
    pub id: String,
    pub version_type: String, // release, snapshot, old_beta...
    pub url: String,
    pub time: String,
    pub release_time: String,
    pub sha1: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Instance {
    pub id: Uuid,
    pub name: String,
    pub version_id: String,
    pub java_path: Option<String>,
    pub memory_mb: u32,
    pub jvm_args: String,
    pub game_dir: String,
    pub last_played: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserProfile {
    pub username: String,
    pub uuid: Option<String>,
    pub access_token: Option<String>,
    pub skin_url: Option<String>,
    pub offline: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LauncherSettings {
    pub theme: String,
    pub language: String,
    pub close_after_launch: bool,
    pub keep_launcher_open: bool,
    pub max_memory_mb: u32,
    pub java_path: Option<String>,
    pub game_directory: String,
    pub show_snapshots: bool,
    pub show_old_versions: bool,
}

impl Default for LauncherSettings {
    fn default() -> Self {
        let game_dir = dirs::data_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("MyLauncher")
            .join(".minecraft")
            .to_string_lossy()
            .to_string();

        Self {
            theme: "dark".into(),
            language: "ru".into(),
            close_after_launch: false,
            keep_launcher_open: true,
            max_memory_mb: 4096,
            java_path: None,
            game_directory: game_dir,
            show_snapshots: false,
            show_old_versions: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Home,
    Versions,
    Mods,
    Skins,
    Settings,
    Accounts,
}

impl Default for Page {
    fn default() -> Self {
        Page::Home
    }
}

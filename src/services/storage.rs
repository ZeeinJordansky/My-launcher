use crate::models::{Instance, LauncherSettings, UserProfile};
use std::fs;
use std::path::PathBuf;

fn data_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("MyLauncher")
}

pub fn ensure_dirs() -> Result<(), String> {
    let base = data_dir();
    fs::create_dir_all(&base).map_err(|e| e.to_string())?;
    fs::create_dir_all(base.join("instances")).map_err(|e| e.to_string())?;
    fs::create_dir_all(base.join("skins")).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn settings_path() -> PathBuf {
    data_dir().join("settings.toml")
}

pub fn instances_path() -> PathBuf {
    data_dir().join("instances.json")
}

pub fn profile_path() -> PathBuf {
    data_dir().join("profile.json")
}

pub fn load_settings() -> LauncherSettings {
    let path = settings_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(s) = toml::from_str(&content) {
                return s;
            }
        }
    }
    LauncherSettings::default()
}

pub fn save_settings(settings: &LauncherSettings) -> Result<(), String> {
    ensure_dirs()?;
    let content = toml::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(settings_path(), content).map_err(|e| e.to_string())
}

pub fn load_instances() -> Vec<Instance> {
    let path = instances_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(list) = serde_json::from_str(&content) {
                return list;
            }
        }
    }
    vec![]
}

pub fn save_instances(instances: &[Instance]) -> Result<(), String> {
    ensure_dirs()?;
    let content = serde_json::to_string_pretty(instances).map_err(|e| e.to_string())?;
    fs::write(instances_path(), content).map_err(|e| e.to_string())
}

pub fn load_profile() -> UserProfile {
    let path = profile_path();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(p) = serde_json::from_str(&content) {
                return p;
            }
        }
    }
    UserProfile {
        username: "Player".into(),
        offline: true,
        ..Default::default()
    }
}

pub fn save_profile(profile: &UserProfile) -> Result<(), String> {
    ensure_dirs()?;
    let content = serde_json::to_string_pretty(profile).map_err(|e| e.to_string())?;
    fs::write(profile_path(), content).map_err(|e| e.to_string())
}

pub fn skins_dir() -> PathBuf {
    data_dir().join("skins")
}

pub fn instances_dir() -> PathBuf {
    data_dir().join("instances")
}

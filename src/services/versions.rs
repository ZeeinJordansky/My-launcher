use crate::models::MinecraftVersion;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    latest: Latest,
    versions: Vec<VersionEntry>,
}

#[derive(Debug, Deserialize)]
struct Latest {
    release: String,
    snapshot: String,
}

#[derive(Debug, Deserialize)]
struct VersionEntry {
    id: String,
    #[serde(rename = "type")]
    version_type: String,
    url: String,
    time: String,
    #[serde(rename = "releaseTime")]
    release_time: String,
    sha1: Option<String>,
}

const MANIFEST_URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

pub async fn fetch_version_manifest() -> Result<(String, String, Vec<MinecraftVersion>), String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(MANIFEST_URL)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("HTTP {}", resp.status()));
    }

    let manifest: Manifest = resp
        .json()
        .await
        .map_err(|e| format!("JSON parse error: {}", e))?;

    let versions = manifest
        .versions
        .into_iter()
        .map(|v| MinecraftVersion {
            id: v.id,
            version_type: v.version_type,
            url: v.url,
            time: v.time,
            release_time: v.release_time,
            sha1: v.sha1,
        })
        .collect();

    Ok((manifest.latest.release, manifest.latest.snapshot, versions))
}

// Placeholder for actual download + launch logic
pub fn launch_instance(_instance_name: &str, _version_id: &str) -> Result<(), String> {
    // TODO: implement full launch pipeline:
    // 1. Download version JSON
    // 2. Download client jar + libraries + assets + natives
    // 3. Extract natives
    // 4. Build classpath + JVM args
    // 5. Spawn Java process
    Err("Launch not fully implemented yet. See README for roadmap.".into())
}

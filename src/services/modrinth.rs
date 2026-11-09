use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ModrinthProject {
    pub project_id: String,
    pub project_type: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub downloads: u64,
    pub icon_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    hits: Vec<ModrinthProject>,
}

pub async fn search_mods(query: &str, limit: u32) -> Result<Vec<ModrinthProject>, String> {
    let client = reqwest::Client::builder()
        .user_agent("MyLauncher/0.2 (github.com/ZeeinJordansky/My-launcher)")
        .build()
        .map_err(|e| e.to_string())?;

    let q = query.replace(' ', "%20");
    let url = format!(
        "https://api.modrinth.com/v2/search?query={}&limit={}&index=relevance&facets=[[\"project_type:mod\"]]",
        q, limit
    );

    let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("Modrinth HTTP {}", resp.status()));
    }

    let data: SearchResponse = resp.json().await.map_err(|e| e.to_string())?;
    Ok(data.hits)
}

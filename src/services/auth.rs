use uuid::Uuid;

pub struct AuthResult {
    pub username: String,
    pub uuid: String,
    pub access_token: String,
    pub offline: bool,
}

/// Offline mode — works without Microsoft account.
pub fn offline_login(username: &str) -> AuthResult {
    let name = if username.trim().is_empty() {
        "Player".to_string()
    } else {
        username.trim().to_string()
    };

    let uuid = Uuid::new_v4().to_string();

    AuthResult {
        username: name,
        uuid,
        access_token: "0".into(),
        offline: true,
    }
}

/// Microsoft login requires Azure App + OAuth2. Kept as clear error for now.
pub async fn login_microsoft() -> Result<AuthResult, String> {
    Err(
        "Microsoft-авторизация требует регистрации Azure App и OAuth2 flow.\n\
         В этой версии используйте оффлайн-режим.\n\
         Полная реализация — в следующих релизах.".into(),
    )
}

// Microsoft / Mojang authentication placeholders
// Real implementation requires OAuth2 device code or browser flow + token exchange

pub struct AuthResult {
    pub username: String,
    pub uuid: String,
    pub access_token: String,
}

pub async fn login_microsoft() -> Result<AuthResult, String> {
    // TODO: implement MSAL / device code flow
    Err("Microsoft authentication not implemented in this skeleton.".into())
}

pub fn offline_login(username: &str) -> AuthResult {
    AuthResult {
        username: username.to_string(),
        uuid: uuid::Uuid::new_v4().to_string(),
        access_token: "offline".into(),
    }
}

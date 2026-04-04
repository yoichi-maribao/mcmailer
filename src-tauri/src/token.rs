use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::oauth::GOOGLE_TOKEN_ENDPOINT;

const EXPIRY_SAFETY_MARGIN_SECS: i64 = 60;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,
    pub email: String,
}

pub fn is_token_expired(token: &TokenData) -> bool {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    token.expires_at - now < EXPIRY_SAFETY_MARGIN_SECS
}

#[derive(Deserialize)]
struct RefreshResponse {
    access_token: String,
    expires_in: i64,
}

pub async fn refresh_access_token(
    refresh_token: &str,
    client_id: &str,
    client_secret: &str,
    email: &str,
) -> Result<TokenData, String> {
    if refresh_token.is_empty() {
        return Err("Refresh token is empty".to_string());
    }

    let client = reqwest::Client::new();
    let response = client
        .post(GOOGLE_TOKEN_ENDPOINT)
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", client_id),
            ("client_secret", client_secret),
        ])
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Token refresh failed: {}", body));
    }

    let refresh_resp: RefreshResponse = response
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    Ok(TokenData {
        access_token: refresh_resp.access_token,
        refresh_token: refresh_token.to_string(),
        expires_at: now + refresh_resp.expires_in,
        email: email.to_string(),
    })
}

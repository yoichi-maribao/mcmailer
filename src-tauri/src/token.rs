use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::account::{self, Account, AccountStore};
use crate::commands::{SETTING_CLIENT_ID, SETTING_CLIENT_SECRET};
use crate::db::Database;
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

pub async fn get_access_token_for_account(
    email: &str,
    store: &Mutex<AccountStore>,
    db: &Database,
) -> Result<String, String> {
    let (refresh_token, account_email) = {
        let store = store
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let acct = store
            .accounts
            .iter()
            .find(|a| a.email == email)
            .ok_or_else(|| format!("Account not found: {}", email))?;

        let token_data = TokenData {
            access_token: acct.access_token.clone(),
            refresh_token: acct.refresh_token.clone(),
            expires_at: acct.expires_at,
            email: acct.email.clone(),
        };

        if !is_token_expired(&token_data) {
            return Ok(acct.access_token.clone());
        }

        (acct.refresh_token.clone(), acct.email.clone())
    };

    let client_id = db
        .get_setting(SETTING_CLIENT_ID)?
        .ok_or_else(|| "OAuth client ID is not configured".to_string())?;
    let client_secret = db
        .get_setting(SETTING_CLIENT_SECRET)?
        .ok_or_else(|| "OAuth client secret is not configured".to_string())?;

    let refreshed =
        refresh_access_token(&refresh_token, &client_id, &client_secret, &account_email)
            .await?;

    let access_token = refreshed.access_token.clone();

    let updated_account = Account {
        email: refreshed.email,
        access_token: refreshed.access_token,
        refresh_token: refreshed.refresh_token,
        expires_at: refreshed.expires_at,
    };

    db.upsert_account(&updated_account)?;

    let mut store = store
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;
    account::add_account(&mut store, updated_account);

    Ok(access_token)
}

use std::sync::Mutex;

use tauri::State;

use crate::account::{self, Account, AccountStore};
use crate::db::Database;
use crate::oauth;
use crate::types::AccountInfo;

pub const SETTING_ACTIVE_ACCOUNT_EMAIL: &str = "active_account_email";
pub(crate) const SETTING_CLIENT_ID: &str = "client_id";
pub(crate) const SETTING_CLIENT_SECRET: &str = "client_secret";

pub struct AppState {
    pub store: Mutex<AccountStore>,
    pub db: Database,
}

#[tauri::command]
pub async fn start_oauth(
    state: State<'_, AppState>,
) -> Result<(), String> {
    let client_id = state.db.get_setting(SETTING_CLIENT_ID)?
        .ok_or_else(|| "OAuth client ID is not configured".to_string())?;
    let client_secret = state.db.get_setting(SETTING_CLIENT_SECRET)?
        .ok_or_else(|| "OAuth client secret is not configured".to_string())?;

    if client_id.is_empty() || client_secret.is_empty() {
        return Err("OAuth client ID and secret must be configured".to_string());
    }

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .map_err(|e| format!("Failed to bind local server: {}", e))?;
    let port = listener.local_addr()
        .map_err(|e| format!("Failed to get local address: {}", e))?
        .port();

    let auth_url = oauth::build_auth_url(&client_id, port);
    open::that(&auth_url)
        .map_err(|e| format!("Failed to open browser: {}", e))?;

    let (stream, _) = listener.accept()
        .await
        .map_err(|e| format!("Failed to accept connection: {}", e))?;

    let mut buf = vec![0u8; 4096];
    stream.readable()
        .await
        .map_err(|e| format!("Stream not readable: {}", e))?;
    let n = stream.try_read(&mut buf)
        .map_err(|e| format!("Failed to read request: {}", e))?;

    let request = String::from_utf8_lossy(&buf[..n]);
    let path = request
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .ok_or_else(|| "Failed to parse HTTP request".to_string())?;

    let callback_url = format!("http://127.0.0.1:{}{}", port, path);

    let response_body = "Authentication complete. You can close this tab.";
    let http_response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        response_body.len(),
        response_body
    );
    stream.writable()
        .await
        .map_err(|e| format!("Stream not writable: {}", e))?;
    stream.try_write(http_response.as_bytes())
        .map_err(|e| format!("Failed to write response: {}", e))?;

    let code = oauth::extract_code_from_callback(&callback_url)?;
    let token_data = oauth::exchange_code(
        &code, &client_id, &client_secret, port,
    ).await?;

    let new_account = Account {
        email: token_data.email,
        access_token: token_data.access_token,
        refresh_token: token_data.refresh_token,
        expires_at: token_data.expires_at,
    };

    state.db.upsert_account(&new_account)?;

    let mut store = state.store.lock()
        .map_err(|e| format!("Lock error: {}", e))?;

    let is_first = store.accounts.is_empty();
    account::add_account(&mut store, new_account);

    if is_first {
        if let Some(ref email) = store.active_account_email {
            state.db.set_setting(SETTING_ACTIVE_ACCOUNT_EMAIL, email)?;
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn list_accounts(
    state: State<'_, AppState>,
) -> Result<Vec<AccountInfo>, String> {
    let store = state.store.lock()
        .map_err(|e| format!("Lock error: {}", e))?;

    let active_email = store.active_account_email.clone();
    let accounts = account::list_accounts(&store)
        .iter()
        .map(|a| AccountInfo {
            email: a.email.clone(),
            is_active: active_email.as_deref() == Some(&a.email),
        })
        .collect();

    Ok(accounts)
}

#[tauri::command]
pub async fn switch_account(
    email: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    {
        let store = state.store.lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        if !store.accounts.iter().any(|a| a.email == email) {
            return Err(format!("Account not found: {}", email));
        }
    }

    state.db.set_setting(SETTING_ACTIVE_ACCOUNT_EMAIL, &email)?;

    let mut store = state.store.lock()
        .map_err(|e| format!("Lock error: {}", e))?;
    account::switch_account(&mut store, &email)?;
    Ok(())
}

#[tauri::command]
pub async fn remove_account(
    email: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let was_active = {
        let store = state.store.lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        if !store.accounts.iter().any(|a| a.email == email) {
            return Err(format!("Account not found: {}", email));
        }
        store.active_account_email.as_deref() == Some(email.as_str())
    };

    state.db.delete_account(&email)?;
    if was_active {
        state.db.delete_setting(SETTING_ACTIVE_ACCOUNT_EMAIL)?;
    }

    let mut store = state.store.lock()
        .map_err(|e| format!("Lock error: {}", e))?;
    account::remove_account(&mut store, &email)?;

    Ok(())
}

#[tauri::command]
pub async fn get_active_account(
    state: State<'_, AppState>,
) -> Result<Option<AccountInfo>, String> {
    let store = state.store.lock()
        .map_err(|e| format!("Lock error: {}", e))?;

    let result = account::get_active_account(&store).map(|a| AccountInfo {
        email: a.email.clone(),
        is_active: true,
    });

    Ok(result)
}

#[tauri::command]
pub async fn has_oauth_credentials(
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let client_id = state.db.get_setting(SETTING_CLIENT_ID)?;
    let client_secret = state.db.get_setting(SETTING_CLIENT_SECRET)?;
    Ok(client_id.is_some_and(|v| !v.is_empty()) && client_secret.is_some_and(|v| !v.is_empty()))
}

#[tauri::command]
pub async fn set_oauth_credentials(
    client_id: String,
    client_secret: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    if client_id.is_empty() || client_secret.is_empty() {
        return Err("OAuth client ID and secret must not be empty".to_string());
    }
    state.db.set_setting(SETTING_CLIENT_ID, &client_id)?;
    state.db.set_setting(SETTING_CLIENT_SECRET, &client_secret)?;
    Ok(())
}

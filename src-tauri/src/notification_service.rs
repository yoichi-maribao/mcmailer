use std::collections::HashSet;
use std::sync::Mutex;

use crate::account::{self, Account};
use crate::commands::{AppState, SETTING_CLIENT_ID, SETTING_CLIENT_SECRET};
use crate::token;

pub struct NotifiedMessages {
    inner: Mutex<HashSet<String>>,
}

impl NotifiedMessages {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(HashSet::new()),
        }
    }

    pub fn try_mark_new(&self, id: &str) -> bool {
        let mut set = self.inner.lock().unwrap();
        set.insert(id.to_string())
    }
}

pub async fn get_access_token_for_account(
    email: &str,
    state: &AppState,
) -> Result<String, String> {
    let (refresh_token, account_email) = {
        let store = state
            .store
            .lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let acct = store
            .accounts
            .iter()
            .find(|a| a.email == email)
            .ok_or_else(|| format!("Account not found: {}", email))?;

        let token_data = token::TokenData {
            access_token: acct.access_token.clone(),
            refresh_token: acct.refresh_token.clone(),
            expires_at: acct.expires_at,
            email: acct.email.clone(),
        };

        if !token::is_token_expired(&token_data) {
            return Ok(acct.access_token.clone());
        }

        (acct.refresh_token.clone(), acct.email.clone())
    };

    let client_id = state
        .db
        .get_setting(SETTING_CLIENT_ID)?
        .ok_or_else(|| "OAuth client ID is not configured".to_string())?;
    let client_secret = state
        .db
        .get_setting(SETTING_CLIENT_SECRET)?
        .ok_or_else(|| "OAuth client secret is not configured".to_string())?;

    let refreshed =
        token::refresh_access_token(&refresh_token, &client_id, &client_secret, &account_email)
            .await?;

    let access_token = refreshed.access_token.clone();

    let updated_account = Account {
        email: refreshed.email,
        access_token: refreshed.access_token,
        refresh_token: refreshed.refresh_token,
        expires_at: refreshed.expires_at,
    };

    state.db.upsert_account(&updated_account)?;

    let mut store = state
        .store
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;
    account::add_account(&mut store, updated_account);

    Ok(access_token)
}

pub async fn process_notification(
    email: &str,
    history_id: &str,
    state: &AppState,
    app_handle: &tauri::AppHandle,
) -> Result<(), String> {
    use crate::gmail;
    use crate::history;
    use tauri::Emitter;
    use tauri_plugin_notification::NotificationExt;

    let access_token = get_access_token_for_account(email, state).await?;

    let history_response = history::list_history(&access_token, history_id).await?;

    let mut new_message_ids = Vec::new();
    if let Some(entries) = &history_response.history {
        for entry in entries {
            if let Some(messages_added) = &entry.messages_added {
                for added in messages_added {
                    if state.notified_messages.try_mark_new(&added.message.id) {
                        new_message_ids.push(added.message.id.clone());
                    }
                }
            }
        }
    }

    for msg_id in &new_message_ids {
        match gmail::get_message(&access_token, msg_id).await {
            Ok(msg) => {
                let headers = msg
                    .payload
                    .as_ref()
                    .map(|p| &p.headers[..])
                    .unwrap_or(&[]);
                let subject = gmail::parse_message_headers(headers, "Subject").unwrap_or_default();
                let from = gmail::parse_message_headers(headers, "From").unwrap_or_default();

                let _ = app_handle
                    .notification()
                    .builder()
                    .title(&subject)
                    .body(&from)
                    .show();

                // Store navigation target for when user focuses window after seeing notification
                let nav_payload = serde_json::json!({
                    "accountEmail": email,
                    "messageId": &msg.id,
                });
                if let Ok(mut pending) = state.pending_navigation.lock() {
                    *pending = Some(nav_payload);
                }

                let payload = serde_json::json!({
                    "accountEmail": email,
                    "messageId": msg.id,
                    "subject": subject,
                    "from": from,
                });

                let _ = app_handle.emit("new-mail-received", payload);
            }
            Err(e) => {
                println!(
                    "[notification_service] Failed to get message {}: {}",
                    msg_id, e
                );
            }
        }
    }

    let _ = state
        .db
        .update_history_id(email, &history_response.history_id);

    Ok(())
}

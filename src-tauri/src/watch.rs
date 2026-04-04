use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchResponse {
    pub history_id: String,
    pub expiration: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct WatchRequest {
    topic_name: String,
    label_ids: Vec<String>,
}

const EXPIRY_THRESHOLD_MS: i64 = 6 * 60 * 60 * 1000;

pub fn is_watch_expiring_soon(expiration_ms: i64, now_ms: i64) -> bool {
    expiration_ms - now_ms <= EXPIRY_THRESHOLD_MS
}

pub async fn register_watch(
    access_token: &str,
    topic_name: &str,
) -> Result<WatchResponse, String> {
    let url = "https://gmail.googleapis.com/gmail/v1/users/me/watch";

    let body = WatchRequest {
        topic_name: topic_name.to_string(),
        label_ids: vec!["INBOX".to_string()],
    };

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .bearer_auth(access_token)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Gmail API error: {}", body));
    }

    response
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))
}

/// Returns account emails that do not yet have a watch_state entry.
pub fn find_unregistered_accounts(
    all_emails: &[String],
    registered_emails: &[String],
) -> Vec<String> {
    let registered: std::collections::HashSet<&str> =
        registered_emails.iter().map(|e| e.as_str()).collect();
    all_emails
        .iter()
        .filter(|email| !registered.contains(email.as_str()))
        .cloned()
        .collect()
}

/// Registers a Gmail API watch for the given account and persists the state to DB.
async fn register_and_persist(
    email: &str,
    state: &crate::commands::AppState,
    topic: &str,
) -> Result<(), String> {
    let access_token =
        crate::token::get_access_token_for_account(email, &state.store, &state.db).await?;
    let watch_resp = register_watch(&access_token, topic).await?;
    let exp: i64 = watch_resp.expiration.parse().unwrap_or(0);
    state.db.upsert_watch_state(email, &watch_resp.history_id, exp)
}

pub async fn start_renewal_loop(app_handle: tauri::AppHandle) {
    use std::time::Duration;
    use tauri::Manager;
    use crate::commands::AppState;

    // Run immediately on first iteration, then every 30 minutes
    let mut first = true;
    loop {
        if first {
            first = false;
        } else {
            tokio::time::sleep(Duration::from_secs(60 * 30)).await;
        }

        let state = app_handle.state::<AppState>();
        let settings = match crate::notification_commands::load_notification_settings(&state.db) {
            Ok(s) if s.enabled => s,
            _ => continue,
        };

        let watch_states = match state.db.load_all_watch_states() {
            Ok(states) => states,
            Err(_) => continue,
        };

        // Initial registration: register watches for accounts not yet in watch_state
        let all_emails: Vec<String> = {
            let store = match state.store.lock() {
                Ok(s) => s,
                Err(_) => continue,
            };
            store.accounts.iter().map(|a| a.email.clone()).collect()
        };
        let registered_emails: Vec<String> =
            watch_states.iter().map(|(e, _, _)| e.clone()).collect();
        let unregistered = find_unregistered_accounts(&all_emails, &registered_emails);

        for email in &unregistered {
            if let Err(e) = register_and_persist(email, &state, &settings.pubsub_topic).await {
                println!("[watch] Failed to register watch for {}: {}", email, e);
            }
        }

        // Renewal: re-register watches that are expiring soon
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        for (email, _history_id, expiration) in &watch_states {
            if is_watch_expiring_soon(*expiration, now_ms) {
                if let Err(e) = register_and_persist(email, &state, &settings.pubsub_topic).await {
                    println!("[watch] Failed to renew watch for {}: {}", email, e);
                }
            }
        }
    }
}

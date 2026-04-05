use std::time::Duration;

use serde_json::json;
use tauri::Manager;

use crate::commands::AppState;
use crate::pubsub_message::{decode_message_data, PullResponse};

const PULL_INTERVAL_SECS: u64 = 5;

/// Extracts `emailAddress` and `historyId` from a Gmail Pub/Sub notification.
/// `historyId` can be either a JSON string or a JSON number.
pub fn extract_notification_fields(
    notification: &serde_json::Value,
) -> Option<(String, String)> {
    let email = notification["emailAddress"].as_str()?;

    let history_id = notification["historyId"]
        .as_str()
        .map(|s| s.to_string())
        .or_else(|| notification["historyId"].as_u64().map(|n| n.to_string()))?;

    Some((email.to_string(), history_id))
}

async fn pull_messages(
    access_token: &str,
    subscription: &str,
) -> Result<PullResponse, String> {
    let url = format!(
        "https://pubsub.googleapis.com/v1/{}:pull",
        subscription
    );

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .bearer_auth(access_token)
        .json(&json!({ "maxMessages": 10 }))
        .send()
        .await
        .map_err(|e| format!("Pull request failed: {}", e))?;

    if !response.status().is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Pub/Sub pull error: {}", body));
    }

    response
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))
}

async fn acknowledge_messages(
    access_token: &str,
    subscription: &str,
    ack_ids: &[String],
) -> Result<(), String> {
    if ack_ids.is_empty() {
        return Ok(());
    }

    let url = format!(
        "https://pubsub.googleapis.com/v1/{}:acknowledge",
        subscription
    );

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .bearer_auth(access_token)
        .json(&json!({ "ackIds": ack_ids }))
        .send()
        .await
        .map_err(|e| format!("Acknowledge request failed: {}", e))?;

    if !response.status().is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Pub/Sub acknowledge error: {}", body));
    }

    Ok(())
}

/// Returns an access token from any available account for Pub/Sub API calls.
fn get_any_account_email(state: &AppState) -> Option<String> {
    let store = state.store.lock().ok()?;
    store.accounts.first().map(|a| a.email.clone())
}

pub async fn start(app_handle: tauri::AppHandle) {
    let mut interval = tokio::time::interval(Duration::from_secs(PULL_INTERVAL_SECS));

    loop {
        interval.tick().await;

        let state = app_handle.state::<AppState>();

        let settings = match crate::notification_commands::load_notification_settings(&state.db) {
            Ok(s) => s,
            Err(_) => continue,
        };

        if !settings.enabled || settings.pubsub_subscription.is_empty() {
            continue;
        }

        let email = match get_any_account_email(&state) {
            Some(e) => e,
            None => continue,
        };

        let access_token = match crate::token::get_access_token_for_account(
            &email,
            &state.store,
            &state.db,
        )
        .await
        {
            Ok(t) => t,
            Err(e) => {
                println!("[pubsub_pull] Failed to get access token: {}", e);
                continue;
            }
        };

        let pull_response = match pull_messages(&access_token, &settings.pubsub_subscription).await
        {
            Ok(r) => r,
            Err(e) => {
                println!("[pubsub_pull] {}", e);
                continue;
            }
        };

        if pull_response.received_messages.is_empty() {
            continue;
        }

        let mut ack_ids = Vec::new();

        for received in &pull_response.received_messages {
            ack_ids.push(received.ack_id.clone());

            let decoded = match decode_message_data(&received.message.data) {
                Ok(d) => d,
                Err(e) => {
                    println!("[pubsub_pull] Failed to decode message: {}", e);
                    continue;
                }
            };

            println!("[pubsub_pull] Received message: {}", decoded);

            let notification = match serde_json::from_str::<serde_json::Value>(&decoded) {
                Ok(v) => v,
                Err(e) => {
                    println!("[pubsub_pull] Failed to parse notification JSON: {}", e);
                    continue;
                }
            };

            match extract_notification_fields(&notification) {
                Some((notif_email, history_id)) => {
                    if let Err(e) = crate::notification_service::process_notification(
                        &notif_email,
                        &history_id,
                        &state,
                        &app_handle,
                    )
                    .await
                    {
                        println!("[pubsub_pull] Failed to process notification: {}", e);
                    }
                }
                None => {
                    println!(
                        "[pubsub_pull] Failed to extract notification fields from: {}",
                        decoded
                    );
                }
            }
        }

        if let Err(e) =
            acknowledge_messages(&access_token, &settings.pubsub_subscription, &ack_ids).await
        {
            println!("[pubsub_pull] {}", e);
        }
    }
}

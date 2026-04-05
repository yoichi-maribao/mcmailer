use std::collections::HashSet;
use std::sync::Mutex;

use crate::commands::AppState;
use crate::gmail::{self, GmailMessage};
use crate::history::{self, HistoryListResponse};
use crate::token;

use tauri::Emitter;
use tauri_plugin_notification::NotificationExt;

pub const EVENT_NEW_MAIL_RECEIVED: &str = "new-mail-received";
pub const EVENT_NAVIGATE_TO_MAIL: &str = "navigate-to-mail";

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

fn fetch_new_message_ids(
    history_response: &HistoryListResponse,
    notified_messages: &NotifiedMessages,
) -> Vec<String> {
    let mut new_message_ids = Vec::new();
    if let Some(entries) = &history_response.history {
        for entry in entries {
            if let Some(messages_added) = &entry.messages_added {
                for added in messages_added {
                    if notified_messages.try_mark_new(&added.message.id) {
                        new_message_ids.push(added.message.id.clone());
                    }
                }
            }
        }
    }
    new_message_ids
}

fn notify_single_message(
    msg: &GmailMessage,
    email: &str,
    app_handle: &tauri::AppHandle,
    pending_navigation: &Mutex<Option<serde_json::Value>>,
) -> Result<(), String> {
    let headers = msg
        .payload
        .as_ref()
        .map(|p| &p.headers[..])
        .unwrap_or(&[]);
    let subject = gmail::parse_message_headers(headers, "Subject")
        .unwrap_or_else(|| "(No Subject)".to_string());
    let from = gmail::parse_message_headers(headers, "From")
        .unwrap_or_else(|| "(Unknown Sender)".to_string());

    println!(
        "[notification_service] Message {}: subject=\"{}\", from=\"{}\"",
        msg.id, subject, from
    );

    if let Err(e) = app_handle
        .notification()
        .builder()
        .title(&subject)
        .body(&from)
        .show()
    {
        println!("[notification_service] Failed to show notification: {}", e);
    }

    let nav_payload = serde_json::json!({
        "accountEmail": email,
        "messageId": &msg.id,
    });
    if let Ok(mut pending) = pending_navigation.lock() {
        *pending = Some(nav_payload);
    }

    let payload = serde_json::json!({
        "accountEmail": email,
        "messageId": msg.id,
        "subject": subject,
        "from": from,
    });

    if let Err(e) = app_handle.emit(EVENT_NEW_MAIL_RECEIVED, payload) {
        println!("[notification_service] Failed to emit new-mail-received: {}", e);
    }

    Ok(())
}

pub async fn process_notification(
    email: &str,
    history_id: &str,
    state: &AppState,
    app_handle: &tauri::AppHandle,
) -> Result<(), String> {
    println!(
        "[notification_service] Processing notification for email={}, history_id={}",
        email, history_id
    );

    let access_token =
        token::get_access_token_for_account(email, &state.store, &state.db).await?;

    let history_response = history::list_history(&access_token, history_id).await?;

    let entry_count = history_response
        .history
        .as_ref()
        .map(|h| h.len())
        .unwrap_or(0);
    println!(
        "[notification_service] History API response: {} entries, history_id={}",
        entry_count, history_response.history_id
    );

    let new_message_ids = fetch_new_message_ids(&history_response, &state.notified_messages);

    println!(
        "[notification_service] New messages detected: {:?}",
        new_message_ids
    );

    for msg_id in &new_message_ids {
        match gmail::get_message(&access_token, msg_id).await {
            Ok(msg) => {
                notify_single_message(&msg, email, app_handle, &state.pending_navigation)?;
            }
            Err(e) => {
                println!(
                    "[notification_service] Failed to get message {}: {}",
                    msg_id, e
                );
            }
        }
    }

    if let Err(e) = state
        .db
        .update_history_id(email, &history_response.history_id)
    {
        println!(
            "[notification_service] Failed to update history_id for {}: {}",
            email, e
        );
    }

    Ok(())
}

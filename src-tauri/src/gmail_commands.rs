use tauri::State;

use crate::account::{self, Account};
use crate::commands::{AppState, SETTING_CLIENT_ID, SETTING_CLIENT_SECRET};
use crate::gmail;
use crate::token;
use crate::types::{MessageDetail, MessageListResult, MessageSummary};

pub(crate) async fn get_active_access_token(
    state: &AppState,
) -> Result<String, String> {
    let (refresh_token, email) = {
        let store = state.store.lock()
            .map_err(|e| format!("Lock error: {}", e))?;
        let active = account::get_active_account(&store)
            .ok_or_else(|| "No active account".to_string())?;

        let token_data = token::TokenData {
            access_token: active.access_token.clone(),
            refresh_token: active.refresh_token.clone(),
            expires_at: active.expires_at,
            email: active.email.clone(),
        };

        if !token::is_token_expired(&token_data) {
            return Ok(active.access_token.clone());
        }

        (active.refresh_token.clone(), active.email.clone())
    };

    let client_id = state.db.get_setting(SETTING_CLIENT_ID)?
        .ok_or_else(|| "OAuth client ID is not configured".to_string())?;
    let client_secret = state.db.get_setting(SETTING_CLIENT_SECRET)?
        .ok_or_else(|| "OAuth client secret is not configured".to_string())?;

    let refreshed = token::refresh_access_token(
        &refresh_token, &client_id, &client_secret, &email,
    ).await?;

    let access_token = refreshed.access_token.clone();

    let updated_account = Account {
        email: refreshed.email,
        access_token: refreshed.access_token,
        refresh_token: refreshed.refresh_token,
        expires_at: refreshed.expires_at,
    };

    state.db.upsert_account(&updated_account)?;

    let mut store = state.store.lock()
        .map_err(|e| format!("Lock error: {}", e))?;
    account::add_account(&mut store, updated_account);

    Ok(access_token)
}

#[tauri::command]
pub async fn list_messages(
    page_token: Option<String>,
    state: State<'_, AppState>,
) -> Result<MessageListResult, String> {
    let access_token = get_active_access_token(&state).await?;

    println!("[list_messages] メール一覧をGmail APIから取得中...");
    let response = gmail::list_messages(
        &access_token,
        page_token.as_deref(),
    ).await?;
    println!("[list_messages] {}件のメールIDを取得", response.messages.len());

    let mut summaries = Vec::with_capacity(response.messages.len());
    for (i, entry) in response.messages.iter().enumerate() {
        println!("[list_messages] メール詳細を取得中 ({}/{}): {}", i + 1, response.messages.len(), entry.id);
        let msg = gmail::get_message(&access_token, &entry.id).await?;
        let headers = msg.payload.as_ref()
            .map(|p| &p.headers[..])
            .unwrap_or(&[]);

        summaries.push(MessageSummary {
            id: msg.id,
            subject: gmail::parse_message_headers(headers, "Subject")
                .unwrap_or_default(),
            from: gmail::parse_message_headers(headers, "From")
                .unwrap_or_default(),
            snippet: msg.snippet,
            date: gmail::parse_message_headers(headers, "Date")
                .unwrap_or_default(),
            is_unread: msg.label_ids.contains(&"UNREAD".to_string()),
        });
    }

    println!("[list_messages] メール一覧の取得完了: {}件", summaries.len());
    Ok(MessageListResult {
        messages: summaries,
        next_page_token: response.next_page_token,
    })
}

#[tauri::command]
pub async fn get_message(
    id: String,
    state: State<'_, AppState>,
) -> Result<MessageDetail, String> {
    let access_token = get_active_access_token(&state).await?;

    let msg = gmail::get_message(&access_token, &id).await?;
    let payload = msg.payload
        .ok_or_else(|| "Message has no payload".to_string())?;

    let headers = &payload.headers;
    let subject = gmail::parse_message_headers(headers, "Subject").unwrap_or_default();
    println!("[get_message] id={}, subject={}", id, subject);

    let (body, content_type) = gmail::extract_body_from_payload(&payload);
    println!("[get_message] body_len={}, content_type={}", body.len(), content_type);

    Ok(MessageDetail {
        id: msg.id,
        subject,
        from: gmail::parse_message_headers(headers, "From")
            .unwrap_or_default(),
        date: gmail::parse_message_headers(headers, "Date")
            .unwrap_or_default(),
        body,
        content_type,
    })
}

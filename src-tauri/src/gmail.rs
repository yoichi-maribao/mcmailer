use base64::engine::general_purpose::{URL_SAFE, URL_SAFE_NO_PAD};
use base64::Engine;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageHeader {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePart {
    pub data: Option<String>,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagePayload {
    pub mime_type: String,
    pub body: Option<MessagePart>,
    pub parts: Option<Vec<MessagePayload>>,
    #[serde(default)]
    pub headers: Vec<MessageHeader>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GmailMessage {
    pub id: String,
    pub thread_id: String,
    #[serde(default)]
    pub label_ids: Vec<String>,
    #[serde(default)]
    pub snippet: String,
    pub payload: Option<MessagePayload>,
    #[serde(default)]
    pub internal_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageListEntry {
    pub id: String,
    #[serde(rename = "threadId")]
    pub thread_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GmailMessageListResponse {
    pub messages: Vec<MessageListEntry>,
    pub next_page_token: Option<String>,
    #[serde(default)]
    pub result_size_estimate: Option<u32>,
}

pub fn parse_message_headers(headers: &[MessageHeader], name: &str) -> Option<String> {
    headers
        .iter()
        .find(|h| h.name == name)
        .map(|h| h.value.clone())
}

pub fn extract_body_from_payload(payload: &MessagePayload) -> (String, String) {
    println!("[extract_body] payload mime_type={}, has_body={}, has_parts={}",
        payload.mime_type,
        payload.body.is_some(),
        payload.parts.is_some(),
    );

    if let Some(ref parts) = payload.parts {
        println!("[extract_body] parts count={}, mime_types={:?}",
            parts.len(),
            parts.iter().map(|p| p.mime_type.as_str()).collect::<Vec<_>>(),
        );

        // First, look for text/html or text/plain at this level
        let html_part = parts.iter().find(|p| p.mime_type == "text/html");
        if let Some(part) = html_part {
            return extract_single_body(part);
        }

        let plain_part = parts.iter().find(|p| p.mime_type == "text/plain");
        if let Some(part) = plain_part {
            return extract_single_body(part);
        }

        // Recurse into nested multipart parts (e.g. multipart/alternative inside multipart/mixed)
        for part in parts {
            if part.parts.is_some() {
                let result = extract_body_from_payload(part);
                if !result.0.is_empty() {
                    return result;
                }
            }
        }

        return (String::new(), "text/plain".to_string());
    }

    extract_single_body(payload)
}

fn decode_base64(data: &str) -> Option<Vec<u8>> {
    // Strip whitespace and newlines that Gmail API may include
    let cleaned: String = data.chars().filter(|c| !c.is_whitespace()).collect();

    // Try URL-safe without padding first (most common for Gmail API)
    if let Ok(bytes) = URL_SAFE_NO_PAD.decode(&cleaned) {
        return Some(bytes);
    }
    // Try URL-safe with padding
    if let Ok(bytes) = URL_SAFE.decode(&cleaned) {
        return Some(bytes);
    }
    println!("[decode_base64] all decoders failed for data (len={})", cleaned.len());
    None
}

fn extract_single_body(payload: &MessagePayload) -> (String, String) {
    let has_data = payload.body.as_ref().and_then(|b| b.data.as_ref()).is_some();
    let data_len = payload.body.as_ref()
        .and_then(|b| b.data.as_ref())
        .map(|d| d.len())
        .unwrap_or(0);
    println!("[extract_single_body] mime_type={}, has_body={}, has_data={}, data_len={}",
        payload.mime_type,
        payload.body.is_some(),
        has_data,
        data_len,
    );

    let body = payload
        .body
        .as_ref()
        .and_then(|b| b.data.as_ref())
        .and_then(|data| decode_base64(data))
        .and_then(|bytes| String::from_utf8(bytes).ok())
        .unwrap_or_default();

    if body.is_empty() && has_data {
        println!("[extract_single_body] WARNING: had data but decoded to empty body");
    }

    (body, payload.mime_type.clone())
}

pub async fn list_messages(
    access_token: &str,
    page_token: Option<&str>,
) -> Result<GmailMessageListResponse, String> {
    let mut url =
        "https://gmail.googleapis.com/gmail/v1/users/me/messages?maxResults=20".to_string();
    if let Some(token) = page_token {
        url.push_str(&format!("&pageToken={}", token));
    }

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .bearer_auth(access_token)
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

pub async fn get_message(
    access_token: &str,
    message_id: &str,
) -> Result<GmailMessage, String> {
    let url = format!(
        "https://gmail.googleapis.com/gmail/v1/users/me/messages/{}",
        message_id
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .bearer_auth(access_token)
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

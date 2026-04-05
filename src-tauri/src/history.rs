use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryListResponse {
    pub history: Option<Vec<HistoryEntry>>,
    pub history_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryEntry {
    pub id: String,
    pub messages_added: Option<Vec<MessageAdded>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageAdded {
    pub message: HistoryMessage,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryMessage {
    pub id: String,
    pub thread_id: String,
    #[serde(default)]
    pub label_ids: Vec<String>,
}

pub async fn list_history(
    access_token: &str,
    start_history_id: &str,
) -> Result<HistoryListResponse, String> {
    let url = format!(
        "https://gmail.googleapis.com/gmail/v1/users/me/history?startHistoryId={}&historyTypes=messageAdded&labelIds=INBOX",
        start_history_id
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

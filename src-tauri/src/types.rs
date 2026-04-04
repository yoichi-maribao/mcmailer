use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub email: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageSummary {
    pub id: String,
    pub subject: String,
    pub from: String,
    pub snippet: String,
    pub date: String,
    pub is_unread: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageListResult {
    pub messages: Vec<MessageSummary>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageDetail {
    pub id: String,
    pub subject: String,
    pub from: String,
    pub date: String,
    pub body: String,
    pub content_type: String,
}

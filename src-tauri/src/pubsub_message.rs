use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PubSubPushBody {
    pub message: PubSubPushMessage,
    #[allow(dead_code)]
    pub subscription: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PubSubPushMessage {
    pub data: String,
    #[allow(dead_code)]
    pub message_id: String,
}

/// Decodes base64-encoded Pub/Sub message data into a UTF-8 string.
pub fn parse_pubsub_push_message(data: &str) -> Result<String, PubSubParseError> {
    let decoded_bytes = STANDARD
        .decode(data)
        .map_err(|e| PubSubParseError::InvalidBase64(e.to_string()))?;

    String::from_utf8(decoded_bytes)
        .map_err(|e| PubSubParseError::InvalidUtf8(e.to_string()))
}

#[derive(Debug)]
pub enum PubSubParseError {
    InvalidBase64(String),
    InvalidUtf8(String),
}

impl std::fmt::Display for PubSubParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PubSubParseError::InvalidBase64(e) => write!(f, "Invalid base64: {}", e),
            PubSubParseError::InvalidUtf8(e) => write!(f, "Invalid UTF-8: {}", e),
        }
    }
}

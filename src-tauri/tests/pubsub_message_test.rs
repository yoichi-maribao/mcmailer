use mcmailer_lib::pubsub_message::{parse_pubsub_push_message, PubSubPushBody, PubSubParseError};

#[cfg(test)]
mod tests {
    use super::*;

    // --- parse_pubsub_push_message ---

    #[test]
    fn should_decode_valid_base64_message() {
        // Given: a valid base64-encoded UTF-8 string
        // "hello world" -> "aGVsbG8gd29ybGQ="
        let data = "aGVsbG8gd29ybGQ=";

        // When: parsing the message
        let result = parse_pubsub_push_message(data);

        // Then: returns the decoded string
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello world");
    }

    #[test]
    fn should_decode_json_payload_from_base64() {
        // Given: a base64-encoded JSON string (typical Pub/Sub payload)
        use base64::engine::general_purpose::STANDARD;
        use base64::Engine;
        let json = r#"{"emailAddress":"user@gmail.com","historyId":"12345"}"#;
        let encoded = STANDARD.encode(json);

        // When: parsing the message
        let result = parse_pubsub_push_message(&encoded);

        // Then: returns the decoded JSON string
        assert!(result.is_ok());
        let decoded = result.unwrap();
        assert!(decoded.contains("user@gmail.com"));
        assert!(decoded.contains("12345"));
    }

    #[test]
    fn should_return_error_for_invalid_base64() {
        // Given: an invalid base64 string
        let data = "not-valid-base64!!!";

        // When: parsing the message
        let result = parse_pubsub_push_message(data);

        // Then: returns InvalidBase64 error
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PubSubParseError::InvalidBase64(_)));
    }

    #[test]
    fn should_return_error_for_invalid_utf8() {
        // Given: base64-encoded bytes that are not valid UTF-8
        use base64::engine::general_purpose::STANDARD;
        use base64::Engine;
        let invalid_utf8: &[u8] = &[0xFF, 0xFE, 0xFD];
        let encoded = STANDARD.encode(invalid_utf8);

        // When: parsing the message
        let result = parse_pubsub_push_message(&encoded);

        // Then: returns InvalidUtf8 error
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PubSubParseError::InvalidUtf8(_)));
    }

    // --- PubSubPushBody deserialization ---

    #[test]
    fn should_deserialize_pubsub_push_body() {
        // Given: a JSON payload matching Google Pub/Sub push format
        let json = r#"{
            "message": {
                "data": "aGVsbG8=",
                "messageId": "msg-001"
            },
            "subscription": "projects/my-project/subscriptions/my-sub"
        }"#;

        // When: deserializing
        let body: PubSubPushBody = serde_json::from_str(json).unwrap();

        // Then: fields are correctly mapped
        assert_eq!(body.message.data, "aGVsbG8=");
        assert_eq!(body.message.message_id, "msg-001");
        assert_eq!(body.subscription, "projects/my-project/subscriptions/my-sub");
    }
}

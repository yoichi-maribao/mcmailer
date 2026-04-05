use mcmailer_lib::pubsub_message::{decode_message_data, PubSubParseError, PullResponse};

#[cfg(test)]
mod tests {
    use super::*;

    // --- decode_message_data ---

    #[test]
    fn should_decode_valid_base64_message() {
        // Given: a valid base64-encoded UTF-8 string
        // "hello world" -> "aGVsbG8gd29ybGQ="
        let data = "aGVsbG8gd29ybGQ=";

        // When: parsing the message
        let result = decode_message_data(data);

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
        let result = decode_message_data(&encoded);

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
        let result = decode_message_data(data);

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
        let result = decode_message_data(&encoded);

        // Then: returns InvalidUtf8 error
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PubSubParseError::InvalidUtf8(_)));
    }

    // --- PullResponse deserialization ---

    #[test]
    fn should_deserialize_pull_response_with_messages() {
        // Given: a JSON payload matching Google Pub/Sub pull response format
        let json = r#"{
            "receivedMessages": [
                {
                    "ackId": "ack-001",
                    "message": {
                        "data": "aGVsbG8=",
                        "messageId": "msg-001"
                    }
                }
            ]
        }"#;

        // When: deserializing
        let response: PullResponse = serde_json::from_str(json).unwrap();

        // Then: fields are correctly mapped
        assert_eq!(response.received_messages.len(), 1);
        assert_eq!(response.received_messages[0].ack_id, "ack-001");
        assert_eq!(response.received_messages[0].message.data, "aGVsbG8=");
        assert_eq!(response.received_messages[0].message.message_id, "msg-001");
    }

    #[test]
    fn should_deserialize_empty_pull_response() {
        // Given: a JSON payload with no messages (empty pull)
        let json = r#"{}"#;

        // When: deserializing
        let response: PullResponse = serde_json::from_str(json).unwrap();

        // Then: received_messages defaults to empty vec
        assert!(response.received_messages.is_empty());
    }
}

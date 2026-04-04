use mcmailer_lib::pubsub_server::{parse_pubsub_push_message, PubSubPushBody, PubSubNotification};

#[cfg(test)]
mod tests {
    use super::*;

    fn base64_standard_encode(input: &str) -> String {
        use base64::engine::general_purpose::STANDARD;
        use base64::Engine;
        STANDARD.encode(input.as_bytes())
    }

    // --- PubSubPushBody deserialization ---

    #[test]
    fn should_deserialize_pubsub_push_body() {
        // Given: a Google Pub/Sub push message HTTP body
        let notification_data = r#"{"emailAddress":"user@gmail.com","historyId":12345}"#;
        let encoded = base64_standard_encode(notification_data);
        let json = format!(
            r#"{{"message": {{"data": "{}", "messageId": "msg-id-001"}}, "subscription": "projects/my-project/subscriptions/my-sub"}}"#,
            encoded
        );

        // When: deserializing
        let body: PubSubPushBody = serde_json::from_str(&json).unwrap();

        // Then: message data and subscription are parsed
        assert_eq!(body.message.data, encoded);
        assert_eq!(body.message.message_id, "msg-id-001");
        assert_eq!(
            body.subscription,
            "projects/my-project/subscriptions/my-sub"
        );
    }

    // --- parse_pubsub_push_message ---

    #[test]
    fn should_parse_valid_pubsub_notification() {
        // Given: a base64-encoded Gmail notification payload
        let notification_json = r#"{"emailAddress":"alice@gmail.com","historyId":67890}"#;
        let encoded = base64_standard_encode(notification_json);

        // When: parsing the push message data
        let result = parse_pubsub_push_message(&encoded);

        // Then: email and historyId are extracted
        assert!(result.is_ok());
        let notification = result.unwrap();
        assert_eq!(notification.email_address, "alice@gmail.com");
        assert_eq!(notification.history_id, 67890);
    }

    #[test]
    fn should_return_error_for_invalid_base64() {
        // Given: invalid base64 data
        let invalid_base64 = "not-valid-base64!!!";

        // When: parsing
        let result = parse_pubsub_push_message(invalid_base64);

        // Then: returns error
        assert!(result.is_err());
    }

    #[test]
    fn should_return_error_for_invalid_json_after_decode() {
        // Given: valid base64 but invalid JSON
        let not_json = base64_standard_encode("this is not json");

        // When: parsing
        let result = parse_pubsub_push_message(&not_json);

        // Then: returns error
        assert!(result.is_err());
    }

    #[test]
    fn should_return_error_for_missing_email_address_field() {
        // Given: JSON without emailAddress field
        let json = r#"{"historyId": 12345}"#;
        let encoded = base64_standard_encode(json);

        // When: parsing
        let result = parse_pubsub_push_message(&encoded);

        // Then: returns error
        assert!(result.is_err());
    }

    #[test]
    fn should_return_error_for_missing_history_id_field() {
        // Given: JSON without historyId field
        let json = r#"{"emailAddress": "user@gmail.com"}"#;
        let encoded = base64_standard_encode(json);

        // When: parsing
        let result = parse_pubsub_push_message(&encoded);

        // Then: returns error
        assert!(result.is_err());
    }

    #[test]
    fn should_parse_notification_with_large_history_id() {
        // Given: a notification with a large historyId value
        let json = r#"{"emailAddress":"bob@gmail.com","historyId":9999999999}"#;
        let encoded = base64_standard_encode(json);

        // When: parsing
        let result = parse_pubsub_push_message(&encoded);

        // Then: large historyId is preserved
        assert!(result.is_ok());
        let notification = result.unwrap();
        assert_eq!(notification.history_id, 9999999999);
    }

    #[test]
    fn should_parse_notification_with_plus_in_email() {
        // Given: a notification with a + character in the email
        let json = r#"{"emailAddress":"user+tag@gmail.com","historyId":100}"#;
        let encoded = base64_standard_encode(json);

        // When: parsing
        let result = parse_pubsub_push_message(&encoded);

        // Then: email with + is preserved
        assert!(result.is_ok());
        assert_eq!(result.unwrap().email_address, "user+tag@gmail.com");
    }

    // --- PubSubNotification deserialization ---

    #[test]
    fn should_deserialize_pubsub_notification_directly() {
        // Given: a Gmail Pub/Sub notification JSON
        let json = r#"{"emailAddress":"test@gmail.com","historyId":42}"#;

        // When: deserializing
        let notification: PubSubNotification = serde_json::from_str(json).unwrap();

        // Then: fields are correctly parsed
        assert_eq!(notification.email_address, "test@gmail.com");
        assert_eq!(notification.history_id, 42);
    }

    #[test]
    fn should_return_error_for_empty_base64_data() {
        // Given: empty string as base64 data
        let empty = base64_standard_encode("");

        // When: parsing
        let result = parse_pubsub_push_message(&empty);

        // Then: returns error (empty JSON is invalid)
        assert!(result.is_err());
    }
}

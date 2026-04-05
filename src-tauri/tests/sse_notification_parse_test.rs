use mcmailer_lib::pubsub_pull::extract_notification_fields;

#[cfg(test)]
mod tests {
    use super::*;

    // --- historyId as string (existing behavior) ---

    #[test]
    fn should_extract_fields_when_history_id_is_string() {
        // Given: a Pub/Sub notification where historyId is a JSON string
        let notification = serde_json::json!({
            "emailAddress": "user@gmail.com",
            "historyId": "12345"
        });

        // When: extracting notification fields
        let result = extract_notification_fields(&notification);

        // Then: both fields are extracted
        let (email, history_id) = result.unwrap();
        assert_eq!(email, "user@gmail.com");
        assert_eq!(history_id, "12345");
    }

    // --- historyId as number (bug fix: previously returned None) ---

    #[test]
    fn should_extract_fields_when_history_id_is_number() {
        // Given: a Pub/Sub notification where historyId is a JSON number
        // (Gmail Pub/Sub can send historyId as a number)
        let notification = serde_json::json!({
            "emailAddress": "user@gmail.com",
            "historyId": 12345
        });

        // When: extracting notification fields
        let result = extract_notification_fields(&notification);

        // Then: historyId is converted to string
        let (email, history_id) = result.unwrap();
        assert_eq!(email, "user@gmail.com");
        assert_eq!(history_id, "12345");
    }

    #[test]
    fn should_extract_large_numeric_history_id() {
        // Given: a large numeric historyId (realistic Gmail value)
        let notification = serde_json::json!({
            "emailAddress": "user@gmail.com",
            "historyId": 9876543210u64
        });

        // When: extracting notification fields
        let result = extract_notification_fields(&notification);

        // Then: large number is correctly converted to string
        let (_, history_id) = result.unwrap();
        assert_eq!(history_id, "9876543210");
    }

    // --- Missing fields ---

    #[test]
    fn should_return_none_when_email_address_is_missing() {
        // Given: a notification without emailAddress
        let notification = serde_json::json!({
            "historyId": "12345"
        });

        // When: extracting notification fields
        let result = extract_notification_fields(&notification);

        // Then: returns None
        assert!(result.is_none());
    }

    #[test]
    fn should_return_none_when_history_id_is_missing() {
        // Given: a notification without historyId
        let notification = serde_json::json!({
            "emailAddress": "user@gmail.com"
        });

        // When: extracting notification fields
        let result = extract_notification_fields(&notification);

        // Then: returns None
        assert!(result.is_none());
    }

    #[test]
    fn should_return_none_for_empty_object() {
        // Given: an empty JSON object
        let notification = serde_json::json!({});

        // When: extracting notification fields
        let result = extract_notification_fields(&notification);

        // Then: returns None
        assert!(result.is_none());
    }

    // --- Invalid types ---

    #[test]
    fn should_return_none_when_history_id_is_boolean() {
        // Given: a notification where historyId is a boolean (invalid type)
        let notification = serde_json::json!({
            "emailAddress": "user@gmail.com",
            "historyId": true
        });

        // When: extracting notification fields
        let result = extract_notification_fields(&notification);

        // Then: returns None (boolean is neither string nor number)
        assert!(result.is_none());
    }

    #[test]
    fn should_return_none_when_history_id_is_null() {
        // Given: a notification where historyId is null
        let notification = serde_json::json!({
            "emailAddress": "user@gmail.com",
            "historyId": null
        });

        // When: extracting notification fields
        let result = extract_notification_fields(&notification);

        // Then: returns None
        assert!(result.is_none());
    }

    #[test]
    fn should_return_none_when_email_address_is_not_a_string() {
        // Given: a notification where emailAddress is a number (invalid type)
        let notification = serde_json::json!({
            "emailAddress": 12345,
            "historyId": "12345"
        });

        // When: extracting notification fields
        let result = extract_notification_fields(&notification);

        // Then: returns None (emailAddress must be a string)
        assert!(result.is_none());
    }

    // --- Realistic payloads ---

    #[test]
    fn should_extract_fields_from_realistic_gmail_pubsub_payload() {
        // Given: a realistic Gmail Pub/Sub notification payload
        // (may contain additional fields that should be ignored)
        let notification = serde_json::json!({
            "emailAddress": "alice@example.com",
            "historyId": 4810235
        });

        // When: extracting notification fields
        let result = extract_notification_fields(&notification);

        // Then: relevant fields are extracted, extras are ignored
        let (email, history_id) = result.unwrap();
        assert_eq!(email, "alice@example.com");
        assert_eq!(history_id, "4810235");
    }
}

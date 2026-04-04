use mcmailer_lib::notification_commands::NotificationSettings;

#[cfg(test)]
mod tests {
    use super::*;

    // --- NotificationSettings deserialization ---

    #[test]
    fn should_deserialize_notification_settings_with_all_fields() {
        // Given: a JSON object with all notification settings fields
        let json = r#"{
            "enabled": true,
            "pubsubServerUrl": "http://localhost:8090",
            "pubsubTopic": "projects/my-project/topics/gmail"
        }"#;

        // When: deserializing
        let settings: NotificationSettings = serde_json::from_str(json).unwrap();

        // Then: all fields are parsed
        assert!(settings.enabled);
        assert_eq!(settings.pubsub_server_url, "http://localhost:8090");
        assert_eq!(
            settings.pubsub_topic,
            "projects/my-project/topics/gmail"
        );
    }

    #[test]
    fn should_deserialize_notification_settings_with_disabled() {
        // Given: a JSON object with notifications disabled
        let json = r#"{
            "enabled": false,
            "pubsubServerUrl": "http://192.168.1.100:8090",
            "pubsubTopic": "projects/my-project/topics/gmail-push"
        }"#;

        // When: deserializing
        let settings: NotificationSettings = serde_json::from_str(json).unwrap();

        // Then: enabled is false, other fields are preserved
        assert!(!settings.enabled);
        assert_eq!(settings.pubsub_server_url, "http://192.168.1.100:8090");
        assert_eq!(
            settings.pubsub_topic,
            "projects/my-project/topics/gmail-push"
        );
    }

    // --- NotificationSettings serialization ---

    #[test]
    fn should_serialize_notification_settings_to_json() {
        // Given: a NotificationSettings struct
        let settings = NotificationSettings {
            enabled: true,
            pubsub_server_url: "http://localhost:8090".to_string(),
            pubsub_topic: "projects/my-project/topics/gmail".to_string(),
        };

        // When: serializing to JSON
        let json = serde_json::to_string(&settings).unwrap();

        // Then: JSON is parseable and contains all fields
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["enabled"], true);
        assert_eq!(parsed["pubsubServerUrl"], "http://localhost:8090");
        assert_eq!(
            parsed["pubsubTopic"],
            "projects/my-project/topics/gmail"
        );
    }

    #[test]
    fn should_serialize_notification_settings_with_disabled() {
        // Given: a NotificationSettings struct with enabled false
        let settings = NotificationSettings {
            enabled: false,
            pubsub_server_url: "http://192.168.1.100:8090".to_string(),
            pubsub_topic: "projects/test/topics/gmail-push".to_string(),
        };

        // When: serializing to JSON
        let json = serde_json::to_string(&settings).unwrap();

        // Then: JSON contains all fields with correct values
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["enabled"], false);
        assert_eq!(parsed["pubsubServerUrl"], "http://192.168.1.100:8090");
        assert_eq!(
            parsed["pubsubTopic"],
            "projects/test/topics/gmail-push"
        );
    }

    // --- Missing fields cause deserialization error ---

    #[test]
    fn should_fail_to_deserialize_without_enabled_field() {
        // Given: a JSON missing the enabled field
        let json = r#"{
            "pubsubServerUrl": "http://localhost:8090",
            "pubsubTopic": "projects/my-project/topics/gmail"
        }"#;

        // When: deserializing
        let result: Result<NotificationSettings, _> = serde_json::from_str(json);

        // Then: returns error
        assert!(result.is_err());
    }

    #[test]
    fn should_fail_to_deserialize_without_pubsub_server_url() {
        // Given: a JSON missing the pubsub_server_url field
        let json = r#"{
            "enabled": true,
            "pubsubTopic": "projects/my-project/topics/gmail"
        }"#;

        // When: deserializing
        let result: Result<NotificationSettings, _> = serde_json::from_str(json);

        // Then: returns error
        assert!(result.is_err());
    }

    #[test]
    fn should_fail_to_deserialize_without_pubsub_topic() {
        // Given: a JSON missing the pubsub_topic field
        let json = r#"{
            "enabled": true,
            "pubsubServerUrl": "http://localhost:8090"
        }"#;

        // When: deserializing
        let result: Result<NotificationSettings, _> = serde_json::from_str(json);

        // Then: returns error
        assert!(result.is_err());
    }

    // --- Edge cases ---

    #[test]
    fn should_handle_empty_url_and_topic() {
        // Given: a JSON with empty strings for url and topic
        let json = r#"{
            "enabled": true,
            "pubsubServerUrl": "",
            "pubsubTopic": ""
        }"#;

        // When: deserializing
        let settings: NotificationSettings = serde_json::from_str(json).unwrap();

        // Then: empty strings are preserved
        assert_eq!(settings.pubsub_server_url, "");
        assert_eq!(settings.pubsub_topic, "");
    }

    #[test]
    fn should_handle_url_with_path_and_port() {
        // Given: a JSON with a complex URL
        let json = r#"{
            "enabled": true,
            "pubsubServerUrl": "https://my-server.example.com:9443/pubsub",
            "pubsubTopic": "projects/production-12345/topics/gmail-notifications"
        }"#;

        // When: deserializing
        let settings: NotificationSettings = serde_json::from_str(json).unwrap();

        // Then: complex URL is preserved
        assert_eq!(
            settings.pubsub_server_url,
            "https://my-server.example.com:9443/pubsub"
        );
        assert_eq!(
            settings.pubsub_topic,
            "projects/production-12345/topics/gmail-notifications"
        );
    }

    // --- Round-trip ---

    #[test]
    fn should_round_trip_through_serialization_and_deserialization() {
        // Given: a NotificationSettings struct
        let original = NotificationSettings {
            enabled: false,
            pubsub_server_url: "http://10.0.0.1:8090".to_string(),
            pubsub_topic: "projects/test/topics/mail".to_string(),
        };

        // When: serializing then deserializing
        let json = serde_json::to_string(&original).unwrap();
        let restored: NotificationSettings = serde_json::from_str(&json).unwrap();

        // Then: all fields match
        assert_eq!(restored.enabled, original.enabled);
        assert_eq!(restored.pubsub_server_url, original.pubsub_server_url);
        assert_eq!(restored.pubsub_topic, original.pubsub_topic);
    }
}

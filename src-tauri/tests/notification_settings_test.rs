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
            "pubsubSubscription": "projects/my-project/subscriptions/gmail-sub",
            "pubsubTopic": "projects/my-project/topics/gmail"
        }"#;

        // When: deserializing
        let settings: NotificationSettings = serde_json::from_str(json).unwrap();

        // Then: all fields are parsed
        assert!(settings.enabled);
        assert_eq!(settings.pubsub_subscription, "projects/my-project/subscriptions/gmail-sub");
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
            "pubsubSubscription": "projects/my-project/subscriptions/gmail-push-sub",
            "pubsubTopic": "projects/my-project/topics/gmail-push"
        }"#;

        // When: deserializing
        let settings: NotificationSettings = serde_json::from_str(json).unwrap();

        // Then: enabled is false, other fields are preserved
        assert!(!settings.enabled);
        assert_eq!(settings.pubsub_subscription, "projects/my-project/subscriptions/gmail-push-sub");
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
            pubsub_subscription: "projects/my-project/subscriptions/gmail-sub".to_string(),
            pubsub_topic: "projects/my-project/topics/gmail".to_string(),
        };

        // When: serializing to JSON
        let json = serde_json::to_string(&settings).unwrap();

        // Then: JSON is parseable and contains all fields
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["enabled"], true);
        assert_eq!(parsed["pubsubSubscription"], "projects/my-project/subscriptions/gmail-sub");
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
            pubsub_subscription: "projects/test/subscriptions/gmail-push-sub".to_string(),
            pubsub_topic: "projects/test/topics/gmail-push".to_string(),
        };

        // When: serializing to JSON
        let json = serde_json::to_string(&settings).unwrap();

        // Then: JSON contains all fields with correct values
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["enabled"], false);
        assert_eq!(parsed["pubsubSubscription"], "projects/test/subscriptions/gmail-push-sub");
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
            "pubsubSubscription": "projects/my-project/subscriptions/gmail-sub",
            "pubsubTopic": "projects/my-project/topics/gmail"
        }"#;

        // When: deserializing
        let result: Result<NotificationSettings, _> = serde_json::from_str(json);

        // Then: returns error
        assert!(result.is_err());
    }

    #[test]
    fn should_fail_to_deserialize_without_pubsub_subscription() {
        // Given: a JSON missing the pubsub_subscription field
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
            "pubsubSubscription": "projects/my-project/subscriptions/gmail-sub"
        }"#;

        // When: deserializing
        let result: Result<NotificationSettings, _> = serde_json::from_str(json);

        // Then: returns error
        assert!(result.is_err());
    }

    // --- Edge cases ---

    #[test]
    fn should_handle_empty_subscription_and_topic() {
        // Given: a JSON with empty strings for subscription and topic
        let json = r#"{
            "enabled": true,
            "pubsubSubscription": "",
            "pubsubTopic": ""
        }"#;

        // When: deserializing
        let settings: NotificationSettings = serde_json::from_str(json).unwrap();

        // Then: empty strings are preserved
        assert_eq!(settings.pubsub_subscription, "");
        assert_eq!(settings.pubsub_topic, "");
    }

    #[test]
    fn should_handle_subscription_with_complex_path() {
        // Given: a JSON with a complex subscription path
        let json = r#"{
            "enabled": true,
            "pubsubSubscription": "projects/production-12345/subscriptions/gmail-notifications-sub",
            "pubsubTopic": "projects/production-12345/topics/gmail-notifications"
        }"#;

        // When: deserializing
        let settings: NotificationSettings = serde_json::from_str(json).unwrap();

        // Then: complex paths are preserved
        assert_eq!(
            settings.pubsub_subscription,
            "projects/production-12345/subscriptions/gmail-notifications-sub"
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
            pubsub_subscription: "projects/test/subscriptions/mail-sub".to_string(),
            pubsub_topic: "projects/test/topics/mail".to_string(),
        };

        // When: serializing then deserializing
        let json = serde_json::to_string(&original).unwrap();
        let restored: NotificationSettings = serde_json::from_str(&json).unwrap();

        // Then: all fields match
        assert_eq!(restored.enabled, original.enabled);
        assert_eq!(restored.pubsub_subscription, original.pubsub_subscription);
        assert_eq!(restored.pubsub_topic, original.pubsub_topic);
    }
}

use std::path::Path;

use mcmailer_lib::db::Database;
use mcmailer_lib::notification_commands::load_notification_settings;

#[cfg(test)]
mod tests {
    use super::*;

    fn open_in_memory_db() -> Database {
        Database::open(Path::new(":memory:")).expect("Failed to open in-memory database")
    }

    // Regression: ARCH-002 contract-string
    // Setting keys must be consistent between load and save.
    // These tests verify that load_notification_settings reads from
    // the same keys that set_notification_settings writes to.

    #[test]
    fn should_load_default_settings_when_no_settings_stored() {
        let db = open_in_memory_db();
        let settings = load_notification_settings(&db).unwrap();

        assert!(!settings.enabled);
        assert_eq!(settings.pubsub_subscription, "");
        assert_eq!(settings.pubsub_topic, "");
    }

    #[test]
    fn should_load_enabled_setting_from_db() {
        let db = open_in_memory_db();
        db.set_setting("notifications_enabled", "true").unwrap();

        let settings = load_notification_settings(&db).unwrap();
        assert!(settings.enabled);
    }

    #[test]
    fn should_load_pubsub_subscription_from_db() {
        let db = open_in_memory_db();
        db.set_setting("pubsub_subscription", "projects/my-project/subscriptions/gmail-sub")
            .unwrap();

        let settings = load_notification_settings(&db).unwrap();
        assert_eq!(settings.pubsub_subscription, "projects/my-project/subscriptions/gmail-sub");
    }

    #[test]
    fn should_load_pubsub_topic_from_db() {
        let db = open_in_memory_db();
        db.set_setting("pubsub_topic", "projects/test/topics/gmail")
            .unwrap();

        let settings = load_notification_settings(&db).unwrap();
        assert_eq!(settings.pubsub_topic, "projects/test/topics/gmail");
    }

    #[test]
    fn should_round_trip_all_settings_through_db() {
        let db = open_in_memory_db();
        db.set_setting("notifications_enabled", "true").unwrap();
        db.set_setting("pubsub_subscription", "projects/prod/subscriptions/mail-sub")
            .unwrap();
        db.set_setting("pubsub_topic", "projects/prod/topics/mail")
            .unwrap();

        let settings = load_notification_settings(&db).unwrap();
        assert!(settings.enabled);
        assert_eq!(settings.pubsub_subscription, "projects/prod/subscriptions/mail-sub");
        assert_eq!(settings.pubsub_topic, "projects/prod/topics/mail");
    }
}

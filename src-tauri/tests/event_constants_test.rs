use mcmailer_lib::notification_service::{EVENT_NAVIGATE_TO_MAIL, EVENT_NEW_MAIL_RECEIVED};

#[cfg(test)]
mod tests {
    use super::*;

    // Regression: ARCH-001 contract-string
    // These constants must match the TypeScript side (src/shared/events.ts).
    // Changing them without updating the TS constants will break the event pipeline.

    #[test]
    fn should_define_new_mail_received_event_constant() {
        assert_eq!(EVENT_NEW_MAIL_RECEIVED, "new-mail-received");
    }

    #[test]
    fn should_define_navigate_to_mail_event_constant() {
        assert_eq!(EVENT_NAVIGATE_TO_MAIL, "navigate-to-mail");
    }
}

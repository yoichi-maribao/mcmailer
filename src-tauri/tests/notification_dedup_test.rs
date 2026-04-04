use mcmailer_lib::notification_service::NotifiedMessages;

#[cfg(test)]
mod tests {
    use super::*;

    // --- NotifiedMessages dedup tracking ---

    #[test]
    fn should_return_false_when_message_not_yet_notified() {
        // Given: an empty notified messages tracker
        let tracker = NotifiedMessages::new();

        // When: checking if a message was already notified
        let was_notified = tracker.contains("msg_001");

        // Then: returns false
        assert!(!was_notified);
    }

    #[test]
    fn should_return_true_after_marking_message_as_notified() {
        // Given: a tracker with a notified message
        let tracker = NotifiedMessages::new();
        tracker.mark_notified("msg_001");

        // When: checking if the same message was notified
        let was_notified = tracker.contains("msg_001");

        // Then: returns true
        assert!(was_notified);
    }

    #[test]
    fn should_track_multiple_messages_independently() {
        // Given: a tracker with two notified messages
        let tracker = NotifiedMessages::new();
        tracker.mark_notified("msg_001");
        tracker.mark_notified("msg_002");

        // When: checking each message
        // Then: both are tracked, untracked ones are not
        assert!(tracker.contains("msg_001"));
        assert!(tracker.contains("msg_002"));
        assert!(!tracker.contains("msg_003"));
    }

    #[test]
    fn should_not_duplicate_when_marking_same_message_twice() {
        // Given: a tracker
        let tracker = NotifiedMessages::new();

        // When: marking the same message twice
        tracker.mark_notified("msg_001");
        tracker.mark_notified("msg_001");

        // Then: message is still tracked (idempotent)
        assert!(tracker.contains("msg_001"));
    }

    #[test]
    fn should_handle_empty_message_id() {
        // Given: a tracker
        let tracker = NotifiedMessages::new();

        // When: marking an empty string as notified
        tracker.mark_notified("");

        // Then: empty string is tracked like any other id
        assert!(tracker.contains(""));
        assert!(!tracker.contains("msg_001"));
    }

    #[test]
    fn should_distinguish_similar_message_ids() {
        // Given: a tracker with one notified message
        let tracker = NotifiedMessages::new();
        tracker.mark_notified("msg_001");

        // When: checking a similar but different message ID
        // Then: similar IDs are not confused
        assert!(!tracker.contains("msg_0010"));
        assert!(!tracker.contains("msg_00"));
        assert!(!tracker.contains("MSG_001"));
    }

    // --- try_mark_new: atomic check-and-mark ---

    #[test]
    fn should_return_true_for_new_message_on_try_mark() {
        // Given: an empty tracker
        let tracker = NotifiedMessages::new();

        // When: trying to mark a new message
        let is_new = tracker.try_mark_new("msg_001");

        // Then: returns true (it was new)
        assert!(is_new);
    }

    #[test]
    fn should_return_false_for_already_notified_message_on_try_mark() {
        // Given: a tracker with a notified message
        let tracker = NotifiedMessages::new();
        tracker.mark_notified("msg_001");

        // When: trying to mark the same message again
        let is_new = tracker.try_mark_new("msg_001");

        // Then: returns false (already notified)
        assert!(!is_new);
    }

    #[test]
    fn should_mark_message_on_successful_try_mark() {
        // Given: an empty tracker
        let tracker = NotifiedMessages::new();

        // When: try_mark_new returns true
        let _ = tracker.try_mark_new("msg_002");

        // Then: message is now tracked
        assert!(tracker.contains("msg_002"));
    }
}

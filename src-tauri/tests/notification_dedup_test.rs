use mcmailer_lib::notification_service::NotifiedMessages;

#[cfg(test)]
mod tests {
    use super::*;

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
    fn should_return_false_for_already_marked_message_on_try_mark() {
        // Given: a tracker with a message already marked
        let tracker = NotifiedMessages::new();
        tracker.try_mark_new("msg_001");

        // When: trying to mark the same message again
        let is_new = tracker.try_mark_new("msg_001");

        // Then: returns false (already notified)
        assert!(!is_new);
    }

    #[test]
    fn should_track_multiple_messages_independently_via_try_mark() {
        // Given: a tracker
        let tracker = NotifiedMessages::new();

        // When: marking different messages
        let first = tracker.try_mark_new("msg_001");
        let second = tracker.try_mark_new("msg_002");
        let duplicate = tracker.try_mark_new("msg_001");

        // Then: new messages return true, duplicates return false
        assert!(first);
        assert!(second);
        assert!(!duplicate);
    }

    #[test]
    fn should_handle_empty_message_id_via_try_mark() {
        // Given: a tracker
        let tracker = NotifiedMessages::new();

        // When: marking an empty string
        let first = tracker.try_mark_new("");
        let second = tracker.try_mark_new("");

        // Then: empty string is tracked like any other id
        assert!(first);
        assert!(!second);
    }

    #[test]
    fn should_distinguish_similar_message_ids_via_try_mark() {
        // Given: a tracker with one marked message
        let tracker = NotifiedMessages::new();
        tracker.try_mark_new("msg_001");

        // When: trying similar but different IDs
        // Then: similar IDs are not confused
        assert!(tracker.try_mark_new("msg_0010"));
        assert!(tracker.try_mark_new("msg_00"));
        assert!(tracker.try_mark_new("MSG_001"));
    }
}

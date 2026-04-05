use mcmailer_lib::watch::{WatchResponse, find_unregistered_accounts, is_watch_expiring_soon};

#[cfg(test)]
mod tests {
    use super::*;

    // --- WatchResponse deserialization ---

    #[test]
    fn should_deserialize_watch_response() {
        // Given: a Gmail users.watch API response
        let json = r#"{
            "historyId": "12345",
            "expiration": "1700006400000"
        }"#;

        // When: deserializing
        let response: WatchResponse = serde_json::from_str(json).unwrap();

        // Then: historyId and expiration are parsed
        assert_eq!(response.history_id, "12345");
        assert_eq!(response.expiration, "1700006400000");
    }

    #[test]
    fn should_deserialize_watch_response_with_large_expiration() {
        // Given: a watch response with expiration far in the future
        let json = r#"{
            "historyId": "99999",
            "expiration": "9999999999999"
        }"#;

        // When: deserializing
        let response: WatchResponse = serde_json::from_str(json).unwrap();

        // Then: large expiration value is preserved
        assert_eq!(response.expiration, "9999999999999");
    }

    // --- is_watch_expiring_soon ---

    #[test]
    fn should_return_true_when_watch_expires_within_threshold() {
        // Given: an expiration timestamp 5 hours from now (threshold is 6 hours)
        let now_ms = 1700000000000_i64;
        let five_hours_ms = 5 * 60 * 60 * 1000;
        let expiration_ms = now_ms + five_hours_ms;

        // When: checking if watch is expiring soon
        let result = is_watch_expiring_soon(expiration_ms, now_ms);

        // Then: returns true (5 hours < 6 hours threshold)
        assert!(result);
    }

    #[test]
    fn should_return_false_when_watch_has_plenty_of_time() {
        // Given: an expiration timestamp 3 days from now
        let now_ms = 1700000000000_i64;
        let three_days_ms = 3 * 24 * 60 * 60 * 1000;
        let expiration_ms = now_ms + three_days_ms;

        // When: checking if watch is expiring soon
        let result = is_watch_expiring_soon(expiration_ms, now_ms);

        // Then: returns false (3 days > 6 hours threshold)
        assert!(!result);
    }

    #[test]
    fn should_return_true_when_watch_already_expired() {
        // Given: an expiration timestamp in the past
        let now_ms = 1700000000000_i64;
        let expiration_ms = now_ms - 1000;

        // When: checking if watch is expiring soon
        let result = is_watch_expiring_soon(expiration_ms, now_ms);

        // Then: returns true (already expired)
        assert!(result);
    }

    #[test]
    fn should_return_true_when_watch_expires_exactly_at_threshold() {
        // Given: an expiration timestamp exactly 6 hours from now
        let now_ms = 1700000000000_i64;
        let six_hours_ms = 6 * 60 * 60 * 1000;
        let expiration_ms = now_ms + six_hours_ms;

        // When: checking if watch is expiring soon
        let result = is_watch_expiring_soon(expiration_ms, now_ms);

        // Then: returns true (at threshold boundary, should renew)
        assert!(result);
    }

    // --- find_unregistered_accounts ---

    #[test]
    fn should_return_unregistered_accounts_on_cold_start() {
        // Given: 3 accounts exist but no watch_state entries (cold start)
        let all_emails = vec![
            "a@example.com".to_string(),
            "b@example.com".to_string(),
            "c@example.com".to_string(),
        ];
        let registered_emails: Vec<String> = vec![];

        // When: finding unregistered accounts
        let result = find_unregistered_accounts(&all_emails, &registered_emails);

        // Then: all accounts are returned
        assert_eq!(result.len(), 3);
        assert!(result.contains(&"a@example.com".to_string()));
        assert!(result.contains(&"b@example.com".to_string()));
        assert!(result.contains(&"c@example.com".to_string()));
    }

    #[test]
    fn should_return_only_new_accounts_when_some_already_registered() {
        // Given: 3 accounts exist, 2 already have watch_state entries
        let all_emails = vec![
            "a@example.com".to_string(),
            "b@example.com".to_string(),
            "c@example.com".to_string(),
        ];
        let registered_emails = vec![
            "a@example.com".to_string(),
            "c@example.com".to_string(),
        ];

        // When: finding unregistered accounts
        let result = find_unregistered_accounts(&all_emails, &registered_emails);

        // Then: only the unregistered account is returned
        assert_eq!(result, vec!["b@example.com".to_string()]);
    }

    #[test]
    fn should_return_empty_when_all_accounts_registered() {
        // Given: all accounts already have watch_state entries
        let all_emails = vec!["a@example.com".to_string()];
        let registered_emails = vec!["a@example.com".to_string()];

        // When: finding unregistered accounts
        let result = find_unregistered_accounts(&all_emails, &registered_emails);

        // Then: no accounts need registration
        assert!(result.is_empty());
    }

    #[test]
    fn should_return_empty_when_no_accounts_exist() {
        // Given: no accounts exist
        let all_emails: Vec<String> = vec![];
        let registered_emails: Vec<String> = vec![];

        // When: finding unregistered accounts
        let result = find_unregistered_accounts(&all_emails, &registered_emails);

        // Then: empty result
        assert!(result.is_empty());
    }

    #[test]
    fn should_return_false_when_watch_expires_just_after_threshold() {
        // Given: an expiration timestamp 6 hours and 1 second from now
        let now_ms = 1700000000000_i64;
        let just_over_six_hours_ms = 6 * 60 * 60 * 1000 + 1000;
        let expiration_ms = now_ms + just_over_six_hours_ms;

        // When: checking if watch is expiring soon
        let result = is_watch_expiring_soon(expiration_ms, now_ms);

        // Then: returns false (just over threshold)
        assert!(!result);
    }
}

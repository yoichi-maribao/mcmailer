use mcmailer_lib::sse_client::calculate_backoff;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    // --- Exponential backoff sequence: 5s → 10s → 30s → 60s (max) ---

    #[test]
    fn should_return_5_seconds_for_first_attempt() {
        // Given: first reconnection attempt
        let attempt = 0;

        // When: calculating backoff
        let backoff = calculate_backoff(attempt);

        // Then: 5 second delay
        assert_eq!(backoff, Duration::from_secs(5));
    }

    #[test]
    fn should_return_10_seconds_for_second_attempt() {
        // Given: second reconnection attempt
        let attempt = 1;

        // When: calculating backoff
        let backoff = calculate_backoff(attempt);

        // Then: 10 second delay
        assert_eq!(backoff, Duration::from_secs(10));
    }

    #[test]
    fn should_return_30_seconds_for_third_attempt() {
        // Given: third reconnection attempt
        let attempt = 2;

        // When: calculating backoff
        let backoff = calculate_backoff(attempt);

        // Then: 30 second delay
        assert_eq!(backoff, Duration::from_secs(30));
    }

    #[test]
    fn should_return_60_seconds_for_fourth_attempt() {
        // Given: fourth reconnection attempt
        let attempt = 3;

        // When: calculating backoff
        let backoff = calculate_backoff(attempt);

        // Then: 60 second max delay
        assert_eq!(backoff, Duration::from_secs(60));
    }

    // --- Max cap ---

    #[test]
    fn should_cap_at_60_seconds_for_attempts_beyond_fourth() {
        // Given: fifth and subsequent attempts
        // When/Then: all return 60 second max
        assert_eq!(calculate_backoff(4), Duration::from_secs(60));
        assert_eq!(calculate_backoff(5), Duration::from_secs(60));
        assert_eq!(calculate_backoff(10), Duration::from_secs(60));
        assert_eq!(calculate_backoff(100), Duration::from_secs(60));
    }

    // --- Monotonic increase ---

    #[test]
    fn should_return_monotonically_increasing_delays() {
        // Given: sequential attempts
        let delays: Vec<Duration> = (0..4).map(calculate_backoff).collect();

        // When/Then: each delay is greater than or equal to the previous
        for window in delays.windows(2) {
            assert!(
                window[1] >= window[0],
                "Backoff should not decrease: {:?} < {:?}",
                window[1],
                window[0]
            );
        }
    }
}

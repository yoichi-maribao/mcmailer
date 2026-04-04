use mcmailer_lib::token::{TokenData, refresh_access_token, is_token_expired};
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_token_data(expires_in_secs: i64) -> TokenData {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        TokenData {
            access_token: "test_access_token_abc123".to_string(),
            refresh_token: "test_refresh_token_xyz789".to_string(),
            expires_at: now + expires_in_secs,
            email: "test@gmail.com".to_string(),
        }
    }

    // --- is_token_expired ---

    #[test]
    fn should_return_false_when_token_has_not_expired() {
        // Given: a token that expires in 3600 seconds
        let token = create_test_token_data(3600);

        // When: checking if token is expired
        let expired = is_token_expired(&token);

        // Then: it should not be expired
        assert!(!expired);
    }

    #[test]
    fn should_return_true_when_token_has_expired() {
        // Given: a token that expired 100 seconds ago
        let token = create_test_token_data(-100);

        // When: checking if token is expired
        let expired = is_token_expired(&token);

        // Then: it should be expired
        assert!(expired);
    }

    #[test]
    fn should_return_true_when_token_expires_within_safety_margin() {
        // Given: a token that expires in 30 seconds (within typical safety margin)
        let token = create_test_token_data(30);

        // When: checking if token is expired
        let expired = is_token_expired(&token);

        // Then: it should be considered expired (safety margin is typically 60s)
        assert!(expired);
    }

    #[test]
    fn should_return_false_when_token_expires_beyond_safety_margin() {
        // Given: a token that expires in 120 seconds (beyond typical 60s safety margin)
        let token = create_test_token_data(120);

        // When: checking if token is expired
        let expired = is_token_expired(&token);

        // Then: it should not be considered expired
        assert!(!expired);
    }

    // --- TokenData serialization ---

    #[test]
    fn should_serialize_token_data_to_json() {
        // Given: valid token data
        let token = create_test_token_data(3600);

        // When: serializing to JSON
        let json = serde_json::to_string(&token).unwrap();

        // Then: JSON contains all fields
        assert!(json.contains("test_access_token_abc123"));
        assert!(json.contains("test_refresh_token_xyz789"));
        assert!(json.contains("test@gmail.com"));
    }

    #[test]
    fn should_deserialize_token_data_from_json() {
        // Given: valid JSON representing token data
        let json = r#"{
            "access_token": "access_abc",
            "refresh_token": "refresh_xyz",
            "expires_at": 1700000000,
            "email": "user@gmail.com"
        }"#;

        // When: deserializing from JSON
        let token: TokenData = serde_json::from_str(json).unwrap();

        // Then: all fields are correctly parsed
        assert_eq!(token.access_token, "access_abc");
        assert_eq!(token.refresh_token, "refresh_xyz");
        assert_eq!(token.expires_at, 1700000000);
        assert_eq!(token.email, "user@gmail.com");
    }

    #[test]
    fn should_fail_to_deserialize_token_data_with_missing_fields() {
        // Given: JSON missing required fields
        let json = r#"{
            "access_token": "access_abc"
        }"#;

        // When: deserializing from JSON
        let result = serde_json::from_str::<TokenData>(json);

        // Then: deserialization fails
        assert!(result.is_err());
    }

    // --- refresh_access_token ---

    #[tokio::test]
    async fn should_return_error_when_refresh_token_is_empty() {
        // Given: token data with empty refresh token
        let mut token = create_test_token_data(-100);
        token.refresh_token = "".to_string();

        // When: attempting to refresh the token
        let result = refresh_access_token(
            &token.refresh_token,
            "test_client_id",
            "test_client_secret",
            &token.email,
        )
        .await;

        // Then: returns an error
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_return_error_when_client_credentials_are_invalid() {
        // Given: invalid client credentials
        let token = create_test_token_data(-100);

        // When: attempting to refresh with invalid credentials
        let result = refresh_access_token(
            &token.refresh_token,
            "invalid_client_id",
            "invalid_client_secret",
            &token.email,
        )
        .await;

        // Then: returns an error (network or auth error)
        assert!(result.is_err());
    }
}

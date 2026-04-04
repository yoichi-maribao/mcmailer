use mcmailer_lib::oauth::{build_auth_url, extract_code_from_callback, exchange_code};

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CLIENT_ID: &str = "test_client_id_123.apps.googleusercontent.com";
    const TEST_REDIRECT_PORT: u16 = 8234;

    // --- build_auth_url ---

    #[test]
    fn should_build_valid_google_oauth_url() {
        // Given: valid client ID and redirect port
        let client_id = TEST_CLIENT_ID;
        let port = TEST_REDIRECT_PORT;

        // When: building the auth URL
        let url = build_auth_url(client_id, port);

        // Then: URL contains required OAuth parameters
        assert!(url.starts_with("https://accounts.google.com/o/oauth2/v2/auth"));
        assert!(url.contains(&format!("client_id={}", client_id)));
        assert!(url.contains(&format!(
            "redirect_uri=http%3A%2F%2F127.0.0.1%3A{}%2Fcallback",
            port
        )));
        assert!(url.contains("response_type=code"));
        assert!(url.contains("access_type=offline"));
    }

    #[test]
    fn should_include_gmail_readonly_scope() {
        // Given: valid parameters
        let url = build_auth_url(TEST_CLIENT_ID, TEST_REDIRECT_PORT);

        // Then: URL contains the Gmail readonly scope
        assert!(url.contains("scope="));
        assert!(url.contains("gmail.readonly"));
    }

    #[test]
    fn should_include_email_scope_for_profile_info() {
        // Given: valid parameters
        let url = build_auth_url(TEST_CLIENT_ID, TEST_REDIRECT_PORT);

        // Then: URL contains email scope for fetching user profile
        assert!(url.contains("email"));
    }

    #[test]
    fn should_request_offline_access_for_refresh_token() {
        // Given: valid parameters
        let url = build_auth_url(TEST_CLIENT_ID, TEST_REDIRECT_PORT);

        // Then: URL requests offline access (needed for refresh token)
        assert!(url.contains("access_type=offline"));
    }

    // --- extract_code_from_callback ---

    #[test]
    fn should_extract_auth_code_from_valid_callback_url() {
        // Given: a valid OAuth callback URL with authorization code
        let callback_url = "http://127.0.0.1:8234/callback?code=4/0abc_DEF_ghi&scope=email";

        // When: extracting the code
        let code = extract_code_from_callback(callback_url);

        // Then: returns the authorization code
        assert!(code.is_ok());
        assert_eq!(code.unwrap(), "4/0abc_DEF_ghi");
    }

    #[test]
    fn should_return_error_when_callback_has_no_code() {
        // Given: a callback URL without a code parameter
        let callback_url = "http://127.0.0.1:8234/callback?error=access_denied";

        // When: extracting the code
        let code = extract_code_from_callback(callback_url);

        // Then: returns an error
        assert!(code.is_err());
    }

    #[test]
    fn should_return_error_for_error_response_in_callback() {
        // Given: a callback URL with an error parameter
        let callback_url =
            "http://127.0.0.1:8234/callback?error=access_denied&error_description=User+denied";

        // When: extracting the code
        let code = extract_code_from_callback(callback_url);

        // Then: returns an error indicating the user denied access
        assert!(code.is_err());
    }

    #[test]
    fn should_handle_url_encoded_code_in_callback() {
        // Given: a callback URL with URL-encoded characters in the code
        let callback_url = "http://127.0.0.1:8234/callback?code=4%2F0abc_DEF&scope=email";

        // When: extracting the code
        let code = extract_code_from_callback(callback_url);

        // Then: returns the decoded authorization code
        assert!(code.is_ok());
        assert_eq!(code.unwrap(), "4/0abc_DEF");
    }

    // --- exchange_code ---

    #[tokio::test]
    async fn should_return_error_when_exchanging_empty_code() {
        // Given: an empty authorization code
        let code = "";

        // When: attempting to exchange
        let result = exchange_code(
            code,
            TEST_CLIENT_ID,
            "test_client_secret",
            TEST_REDIRECT_PORT,
        )
        .await;

        // Then: returns an error
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_return_error_with_invalid_client_credentials() {
        // Given: valid code but invalid credentials
        let code = "valid_code_123";

        // When: exchanging with invalid credentials
        let result = exchange_code(
            code,
            "invalid_client_id",
            "invalid_secret",
            TEST_REDIRECT_PORT,
        )
        .await;

        // Then: returns an error
        assert!(result.is_err());
    }
}

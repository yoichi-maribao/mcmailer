use url::Url;

const GOOGLE_AUTH_ENDPOINT: &str = "https://accounts.google.com/o/oauth2/v2/auth";
pub const GOOGLE_TOKEN_ENDPOINT: &str = "https://oauth2.googleapis.com/token";
const GMAIL_READONLY_SCOPE: &str = "https://www.googleapis.com/auth/gmail.readonly";
const EMAIL_SCOPE: &str = "email";
const PUBSUB_SCOPE: &str = "https://www.googleapis.com/auth/pubsub";
const GOOGLE_USERINFO_ENDPOINT: &str = "https://www.googleapis.com/oauth2/v2/userinfo";

pub fn build_auth_url(client_id: &str, port: u16) -> String {
    let redirect_uri = format!("http://127.0.0.1:{}/callback", port);
    let scope = format!("{} {} {}", GMAIL_READONLY_SCOPE, EMAIL_SCOPE, PUBSUB_SCOPE);

    let mut url = Url::parse(GOOGLE_AUTH_ENDPOINT).unwrap();
    url.query_pairs_mut()
        .append_pair("client_id", client_id)
        .append_pair("redirect_uri", &redirect_uri)
        .append_pair("response_type", "code")
        .append_pair("scope", &scope)
        .append_pair("access_type", "offline");

    url.to_string()
}

pub fn extract_code_from_callback(callback_url: &str) -> Result<String, String> {
    let url = Url::parse(callback_url).map_err(|e| format!("Invalid URL: {}", e))?;

    let pairs: std::collections::HashMap<String, String> =
        url.query_pairs().map(|(k, v)| (k.to_string(), v.to_string())).collect();

    if let Some(error) = pairs.get("error") {
        return Err(format!("OAuth error: {}", error));
    }

    pairs
        .get("code")
        .cloned()
        .ok_or_else(|| "No authorization code in callback URL".to_string())
}

#[derive(serde::Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: i64,
}

pub async fn exchange_code(
    code: &str,
    client_id: &str,
    client_secret: &str,
    port: u16,
) -> Result<crate::token::TokenData, String> {
    if code.is_empty() {
        return Err("Authorization code is empty".to_string());
    }

    let redirect_uri = format!("http://127.0.0.1:{}/callback", port);
    let client = reqwest::Client::new();
    let response = client
        .post(GOOGLE_TOKEN_ENDPOINT)
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("redirect_uri", &redirect_uri),
        ])
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Token exchange failed: {}", body));
    }

    let token_resp: TokenResponse = response
        .json()
        .await
        .map_err(|e| format!("Parse error: {}", e))?;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let refresh_token = token_resp.refresh_token
        .ok_or_else(|| "No refresh token received".to_string())?;

    let email = fetch_user_email(&token_resp.access_token).await?;

    Ok(crate::token::TokenData {
        access_token: token_resp.access_token,
        refresh_token,
        expires_at: now + token_resp.expires_in,
        email,
    })
}

#[derive(serde::Deserialize)]
struct UserinfoResponse {
    email: String,
}

async fn fetch_user_email(access_token: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(GOOGLE_USERINFO_ENDPOINT)
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| format!("Network error fetching userinfo: {}", e))?;

    if !response.status().is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Userinfo request failed: {}", body));
    }

    let info: UserinfoResponse = response
        .json()
        .await
        .map_err(|e| format!("Parse error for userinfo: {}", e))?;

    Ok(info.email)
}

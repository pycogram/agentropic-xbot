use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;

pub struct TwitterClient {
    client: Client,
    consumer_key: String,
    consumer_secret: String,
    access_token: String,
    access_token_secret: String,
}

#[derive(Serialize)]
struct TweetRequest {
    text: String,
}

#[derive(Deserialize, Debug)]
pub struct TweetResponse {
    pub data: TweetData,
}

#[derive(Deserialize, Debug)]
pub struct TweetData {
    pub id: String,
    #[allow(dead_code)]
    pub text: String,
}

/// RFC 3986 percent-encoding for OAuth 1.0a
/// Encodes everything EXCEPT unreserved characters: A-Z, a-z, 0-9, '-', '.', '_', '~'
fn percent_encode(input: &str) -> String {
    let mut encoded = String::new();
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                encoded.push(byte as char);
            }
            _ => {
                encoded.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    encoded
}

impl TwitterClient {
    pub fn new() -> Result<Self> {
        let consumer_key = env::var("TWITTER_CONSUMER_KEY")
            .map_err(|_| anyhow::anyhow!("TWITTER_CONSUMER_KEY not set"))?;
        let consumer_secret = env::var("TWITTER_CONSUMER_SECRET")
            .map_err(|_| anyhow::anyhow!("TWITTER_CONSUMER_SECRET not set"))?;
        let access_token = env::var("TWITTER_ACCESS_TOKEN")
            .map_err(|_| anyhow::anyhow!("TWITTER_ACCESS_TOKEN not set"))?;
        let access_token_secret = env::var("TWITTER_ACCESS_TOKEN_SECRET")
            .map_err(|_| anyhow::anyhow!("TWITTER_ACCESS_TOKEN_SECRET not set"))?;

        Ok(Self {
            client: Client::new(),
            consumer_key,
            consumer_secret,
            access_token,
            access_token_secret,
        })
    }

    pub async fn post_tweet(&self, text: &str) -> Result<TweetResponse> {
        let url = "https://api.x.com/2/tweets";

        let tweet_request = TweetRequest {
            text: text.to_string(),
        };

        let auth_header = self.create_oauth_header("POST", url)?;

        let response = self
            .client
            .post(url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .json(&tweet_request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            anyhow::bail!("Twitter API error ({}): {}", status, error_text);
        }

        let tweet_response = response.json::<TweetResponse>().await?;
        Ok(tweet_response)
    }

    fn create_oauth_header(&self, method: &str, url: &str) -> Result<String> {
        use base64::{engine::general_purpose, Engine as _};
        use chrono::Utc;
        use hmac::{Hmac, Mac};
        use sha1::Sha1;

        type HmacSha1 = Hmac<Sha1>;

        let timestamp = Utc::now().timestamp().to_string();
        let nonce: String = rand::random::<u64>().to_string();

        // Collect all OAuth parameters (BTreeMap keeps them sorted)
        let mut params = BTreeMap::new();
        params.insert("oauth_consumer_key", self.consumer_key.as_str());
        params.insert("oauth_nonce", nonce.as_str());
        params.insert("oauth_signature_method", "HMAC-SHA1");
        params.insert("oauth_timestamp", timestamp.as_str());
        params.insert("oauth_token", self.access_token.as_str());
        params.insert("oauth_version", "1.0");

        // Create parameter string using RFC 3986 encoding
        let param_string = params
            .iter()
            .map(|(k, v)| {
                format!("{}={}", percent_encode(k), percent_encode(v))
            })
            .collect::<Vec<_>>()
            .join("&");

        // Signature base string
        let signature_base = format!(
            "{}&{}&{}",
            method,
            percent_encode(url),
            percent_encode(&param_string)
        );

        // Signing key
        let signing_key = format!(
            "{}&{}",
            percent_encode(&self.consumer_secret),
            percent_encode(&self.access_token_secret)
        );

        // HMAC-SHA1 signature
        let mut mac = HmacSha1::new_from_slice(signing_key.as_bytes())
            .map_err(|e| anyhow::anyhow!("HMAC error: {}", e))?;
        mac.update(signature_base.as_bytes());
        let signature = general_purpose::STANDARD.encode(mac.finalize().into_bytes());

        // Build authorization header
        let auth_header = format!(
            r#"OAuth oauth_consumer_key="{}", oauth_nonce="{}", oauth_signature="{}", oauth_signature_method="HMAC-SHA1", oauth_timestamp="{}", oauth_token="{}", oauth_version="1.0""#,
            percent_encode(&self.consumer_key),
            percent_encode(&nonce),
            percent_encode(&signature),
            timestamp,
            percent_encode(&self.access_token)
        );

        Ok(auth_header)
    }
}
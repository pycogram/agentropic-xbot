use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::collections::BTreeMap;

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

impl TwitterClient {
    pub fn new() -> Result<Self> {
        let consumer_key = env::var("TWITTER_CONSUMER_KEY")?;
        let consumer_secret = env::var("TWITTER_CONSUMER_SECRET")?;
        let access_token = env::var("TWITTER_ACCESS_TOKEN")?;
        let access_token_secret = env::var("TWITTER_ACCESS_TOKEN_SECRET")?;
        
        Ok(Self {
            client: Client::new(),
            consumer_key,
            consumer_secret,
            access_token,
            access_token_secret,
        })
    }

    pub async fn post_tweet(&self, text: &str) -> Result<TweetResponse> {
        let url = "https://api.twitter.com/2/tweets";
        
        let tweet_request = TweetRequest {
            text: text.to_string(),
        };

        // Create OAuth 1.0a authorization header
        let auth_header = self.create_oauth_header("POST", url)?;

        let response = self.client
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
        use hmac::{Hmac, Mac};
        use sha1::Sha1;
        use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
        use chrono::Utc;
        use base64::{Engine as _, engine::general_purpose};
        
        type HmacSha1 = Hmac<Sha1>;

        // OAuth parameters
        let timestamp = Utc::now().timestamp().to_string();
        let nonce: String = rand::random::<u64>().to_string();

        // Collect all OAuth parameters
        let mut params = BTreeMap::new();
        params.insert("oauth_consumer_key", self.consumer_key.as_str());
        params.insert("oauth_nonce", nonce.as_str());
        params.insert("oauth_signature_method", "HMAC-SHA1");
        params.insert("oauth_timestamp", timestamp.as_str());
        params.insert("oauth_token", self.access_token.as_str());
        params.insert("oauth_version", "1.0");

        // Create parameter string (sorted alphabetically)
        let param_string = params
            .iter()
            .map(|(k, v)| format!("{}={}", 
                utf8_percent_encode(k, NON_ALPHANUMERIC),
                utf8_percent_encode(v, NON_ALPHANUMERIC)))
            .collect::<Vec<_>>()
            .join("&");

        // Create signature base string
        let signature_base = format!(
            "{}&{}&{}",
            method,
            utf8_percent_encode(url, NON_ALPHANUMERIC),
            utf8_percent_encode(&param_string, NON_ALPHANUMERIC)
        );

        // Create signing key
        let signing_key = format!(
            "{}&{}",
            utf8_percent_encode(&self.consumer_secret, NON_ALPHANUMERIC),
            utf8_percent_encode(&self.access_token_secret, NON_ALPHANUMERIC)
        );

        // Generate signature
        let mut mac = HmacSha1::new_from_slice(signing_key.as_bytes())
            .map_err(|e| anyhow::anyhow!("HMAC error: {}", e))?;
        mac.update(signature_base.as_bytes());
        let signature = general_purpose::STANDARD.encode(mac.finalize().into_bytes());

        // Build authorization header
        let auth_header = format!(
            r#"OAuth oauth_consumer_key="{}", oauth_nonce="{}", oauth_signature="{}", oauth_signature_method="HMAC-SHA1", oauth_timestamp="{}", oauth_token="{}", oauth_version="1.0""#,
            utf8_percent_encode(&self.consumer_key, NON_ALPHANUMERIC),
            utf8_percent_encode(&nonce, NON_ALPHANUMERIC),
            utf8_percent_encode(&signature, NON_ALPHANUMERIC),
            timestamp,
            utf8_percent_encode(&self.access_token, NON_ALPHANUMERIC)
        );

        Ok(auth_header)
    }
}

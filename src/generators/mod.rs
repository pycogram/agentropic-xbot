use crate::templates::TweetTemplates;
use crate::config::{BotConfig, ContentCategory};
use rand::seq::SliceRandom;
use tracing::warn;

/// Maximum tweet length (Twitter limit)
const MAX_TWEET_LENGTH: usize = 280;

pub struct TweetGenerator;

impl TweetGenerator {
    /// Generate a random bull post based on enabled categories
    pub fn generate(config: &BotConfig) -> String {
        let enabled_categories = config.get_enabled_categories();

        if enabled_categories.is_empty() {
            return TweetTemplates::random_bull_tweet();
        }

        // Pick random category from enabled ones
        let category = enabled_categories
            .choose(&mut rand::thread_rng())
            .unwrap();

        match category {
            ContentCategory::AI => TweetTemplates::random_ai_tweet(),
            ContentCategory::Agentropic => TweetTemplates::random_agentropic_tweet(),
            ContentCategory::Crypto => TweetTemplates::random_crypto_tweet(),
            ContentCategory::Meme => TweetTemplates::random_meme_tweet(),
        }
    }

    /// Add bot signature, but only if the result fits within 280 chars
    pub fn add_signature(tweet: String, username: &str) -> String {
        let signature = format!("\n\n-- Auto-posted by {}", username);
        let with_sig = format!("{}{}", tweet, signature);

        if with_sig.len() <= MAX_TWEET_LENGTH {
            with_sig
        } else {
            // Skip signature to stay within limit
            warn!(
                "Tweet is {} chars; skipping signature to stay within {} limit",
                with_sig.len(),
                MAX_TWEET_LENGTH
            );
            tweet
        }
    }

    /// Generate and prepare tweet for posting
    pub fn create_tweet(config: &BotConfig) -> String {
        let base_tweet = Self::generate(config);
        Self::add_signature(base_tweet, &config.bot_username)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_produces_content() {
        let config = BotConfig {
            bot_username: "test".to_string(),
            post_interval_hours: 6,
            max_posts_per_day: 4,
            enable_crypto: true,
            enable_meme: true,
            enable_ai: true,
            enable_agentropic: true,
        };

        let tweet = TweetGenerator::generate(&config);
        assert!(!tweet.is_empty());
    }

    #[test]
    fn test_signature_respects_length() {
        let long_tweet = "a".repeat(270);
        let result = TweetGenerator::add_signature(long_tweet.clone(), "testbot");
        // Should skip signature since 270 + signature > 280
        assert_eq!(result, long_tweet);
    }

    #[test]
    fn test_create_tweet_within_limit() {
        let config = BotConfig {
            bot_username: "test".to_string(),
            post_interval_hours: 6,
            max_posts_per_day: 4,
            enable_crypto: true,
            enable_meme: true,
            enable_ai: true,
            enable_agentropic: true,
        };

        let tweet = TweetGenerator::create_tweet(&config);
        assert!(tweet.len() <= 280);
    }
}
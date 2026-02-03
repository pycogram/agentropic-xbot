use crate::templates::TweetTemplates;
use crate::config::{BotConfig, ContentCategory};
use rand::seq::SliceRandom;

pub struct TweetGenerator;

impl TweetGenerator {
    /// Generate a random bull post based on enabled categories
    pub fn generate(config: &BotConfig) -> String {
        let enabled_categories = config.get_enabled_categories();
        
        if enabled_categories.is_empty() {
            // Fallback to general if nothing enabled
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

    /// Add bot signature
    pub fn add_signature(tweet: String, username: &str) -> String {
        format!("{}\n\n🤖 Auto-posted by {}", tweet, username)
    }

    /// Generate and prepare tweet for posting
    pub fn create_tweet(config: &BotConfig) -> String {
        let base_tweet = Self::generate(config);
        Self::add_signature(base_tweet, &config.bot_username)
    }
}

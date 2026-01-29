use crate::templates::TweetTemplates;
use rand::Rng;

pub struct TweetGenerator;

impl TweetGenerator {
    /// Generate a random bull post
    pub fn generate() -> String {
        let mut rng = rand::thread_rng();
        let category = rng.gen_range(0..5);

        match category {
            0 => TweetTemplates::random_ai_tweet(),
            1 => TweetTemplates::random_agentropic_tweet(),
            2 => TweetTemplates::random_crypto_tweet(),
            3 => TweetTemplates::random_meme_tweet(),
            _ => TweetTemplates::random_bull_tweet(),
        }
    }

    /// Add bot signature
    pub fn add_signature(tweet: String) -> String {
        format!("{}\n\n Auto-posted by AgentropicAI", tweet)
    }

    /// Generate and prepare tweet for posting
    pub fn create_tweet() -> String {
        let base_tweet = Self::generate();
        Self::add_signature(base_tweet)
    }
}

use anyhow::Result;
use std::env;

#[derive(Debug, Clone)]
pub struct BotConfig {
    pub bot_username: String,
    pub post_interval_hours: u64,
    pub max_posts_per_day: u32,
    pub enable_crypto: bool,
    pub enable_meme: bool,
    pub enable_ai: bool,
    pub enable_agentropic: bool,
}

impl BotConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            bot_username: env::var("BOT_USERNAME")
                .unwrap_or_else(|_| "agentropic".to_string()),
            post_interval_hours: env::var("POST_INTERVAL_HOURS")
                .unwrap_or_else(|_| "6".to_string())
                .parse()?,
            max_posts_per_day: env::var("MAX_POSTS_PER_DAY")
                .unwrap_or_else(|_| "4".to_string())
                .parse()?,
            enable_crypto: env::var("ENABLE_CRYPTO_CONTENT")
                .unwrap_or_else(|_| "true".to_string())
                .to_lowercase() == "true",
            enable_meme: env::var("ENABLE_MEME_CONTENT")
                .unwrap_or_else(|_| "true".to_string())
                .to_lowercase() == "true",
            enable_ai: env::var("ENABLE_AI_CONTENT")
                .unwrap_or_else(|_| "true".to_string())
                .to_lowercase() == "true",
            enable_agentropic: env::var("ENABLE_AGENTROPIC_CONTENT")
                .unwrap_or_else(|_| "true".to_string())
                .to_lowercase() == "true",
        })
    }

    pub fn get_cron_expression(&self) -> String {
        format!("0 0 */{} * * *", self.post_interval_hours)
    }

    pub fn validate(&self) -> Result<()> {
        if self.post_interval_hours == 0 {
            anyhow::bail!("POST_INTERVAL_HOURS must be greater than 0");
        }

        if self.max_posts_per_day == 0 {
            anyhow::bail!("MAX_POSTS_PER_DAY must be greater than 0");
        }

        if !self.enable_crypto && !self.enable_meme && !self.enable_ai && !self.enable_agentropic {
            anyhow::bail!("At least one content type must be enabled");
        }

        Ok(())
    }

    pub fn get_enabled_categories(&self) -> Vec<ContentCategory> {
        let mut categories = Vec::new();
        
        if self.enable_ai {
            categories.push(ContentCategory::AI);
        }
        if self.enable_agentropic {
            categories.push(ContentCategory::Agentropic);
        }
        if self.enable_crypto {
            categories.push(ContentCategory::Crypto);
        }
        if self.enable_meme {
            categories.push(ContentCategory::Meme);
        }
        
        categories
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ContentCategory {
    AI,
    Agentropic,
    Crypto,
    Meme,
}

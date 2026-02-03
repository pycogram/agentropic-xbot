mod generators;
mod filters;
mod templates;
mod config;
use anyhow::Result;
use dotenv::dotenv;
use egg_mode::{KeyPair, Token, tweet::DraftTweet};
use std::env;
use tokio_cron_scheduler::{JobScheduler, Job};
use tracing::{info, error};
use generators::TweetGenerator;
use filters::ContentFilter;
use config::BotConfig;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize
    dotenv().ok();
    tracing_subscriber::fmt::init();
    
    info!("🤖 AgentropicAI Bot starting...");
    
    // Load configuration
    let config = BotConfig::from_env()?;
    config.validate()?;
    
    info!("📋 Bot Configuration:");
    info!("  Username: {}", config.bot_username);
    info!("  Post Interval: {} hours", config.post_interval_hours);
    info!("  Max Posts/Day: {}", config.max_posts_per_day);
    info!("  AI Content: {}", config.enable_ai);
    info!("  Agentropic Content: {}", config.enable_agentropic);
    info!("  Crypto Content: {}", config.enable_crypto);
    info!("  Meme Content: {}", config.enable_meme);
    
    // Load Twitter credentials
    let token = load_twitter_token()?;
    
    // Create scheduler
    let scheduler = JobScheduler::new().await?;
    
    // Get cron expression from config
    let cron_expr = config.get_cron_expression();
    info!("⏰ Cron schedule: {}", cron_expr);
    
    // Schedule tweets based on config
    let config_clone = config.clone();
    let token_clone = token.clone();
    
    // FIX 1: Change &cron_expr to cron_expr.as_str()
    let tweet_job = Job::new_async(cron_expr.as_str(), move |_uuid, _lock| {
        let token_inner = token_clone.clone();
        let config_inner = config_clone.clone();
        
        Box::pin(async move {
            if let Err(e) = post_tweet(token_inner, &config_inner).await {
                error!("Failed to post tweet: {}", e);
            }
        })
    })?;
    
    scheduler.add(tweet_job).await?;
    
    info!("✅ Scheduler started - posting every {} hours", config.post_interval_hours);
    info!("📊 Expected: {} tweets per day", config.max_posts_per_day);
    
    // Post one immediately on startup
    info!("🚀 Posting initial tweet...");
    post_tweet(token.clone(), &config).await?;
    
    // Start scheduler
    scheduler.start().await?;
    
    // Keep running
    info!("✅ Bot is now running. Press Ctrl+C to stop.");
    tokio::signal::ctrl_c().await?;
    info!("🛑 Shutting down...");
    
    Ok(())
}

fn load_twitter_token() -> Result<Token> {
    let consumer_key = env::var("TWITTER_API_KEY")?;
    let consumer_secret = env::var("TWITTER_API_SECRET")?;
    let access_key = env::var("TWITTER_ACCESS_TOKEN")?;
    let access_secret = env::var("TWITTER_ACCESS_SECRET")?;
    
    let con_token = KeyPair::new(consumer_key, consumer_secret);
    let access_token = KeyPair::new(access_key, access_secret);
    
    Ok(Token::Access {
        consumer: con_token,
        access: access_token,
    })
}

async fn post_tweet(token: Token, config: &BotConfig) -> Result<()> {
    info!("🎲 Generating tweet...");
    
    // Generate tweet based on config
    let tweet = TweetGenerator::create_tweet(config);
    
    // Validate
    let validated_tweet = match ContentFilter::validate(tweet) {
        Some(t) => t,
        None => {
            error!("❌ Tweet failed validation, skipping");
            return Ok(());
        }
    };
    
    // Safe preview (handles unicode/emojis properly)
    let preview = validated_tweet.chars().take(50).collect::<String>();
    info!("📝 Tweet preview: {}...", preview);
    
    // FIX 2: Use validated_tweet directly instead of &validated_tweet
    let draft = DraftTweet::new(validated_tweet);
    draft.send(&token).await?;
    
    info!("✅ Tweet posted successfully!");
    
    Ok(())
}
mod generators;
mod filters;
mod templates;
mod config;
mod twitter;

use anyhow::Result;
use dotenv::dotenv;
use tokio_cron_scheduler::{JobScheduler, Job};
use tracing::{info, error};

use generators::TweetGenerator;
use filters::ContentFilter;
use config::BotConfig;
use twitter::TwitterClient;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize
    dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("AgentropicAI Bot starting...");

    // Load configuration
    let config = BotConfig::from_env()?;
    config.validate()?;

    info!("Bot Configuration:");
    info!("  Username: {}", config.bot_username);
    info!("  Post Interval: {} hours", config.post_interval_hours);
    info!("  Max Posts/Day: {}", config.max_posts_per_day);
    info!("  AI Content: {}", config.enable_ai);
    info!("  Agentropic Content: {}", config.enable_agentropic);
    info!("  Crypto Content: {}", config.enable_crypto);
    info!("  Meme Content: {}", config.enable_meme);

    // Create Twitter client
    let twitter_client = TwitterClient::new()?;
    info!("Twitter client initialized");

    // Create scheduler
    let scheduler = JobScheduler::new().await?;

    // Get cron expression from config
    let cron_expr = config.get_cron_expression();
    info!("Cron schedule: {}", cron_expr);

    // Schedule tweets based on config
    let config_clone = config.clone();
    let tweet_job = Job::new_async(cron_expr.as_str(), move |_uuid, _lock| {
        let config_inner = config_clone.clone();
        Box::pin(async move {
            let client = match TwitterClient::new() {
                Ok(c) => c,
                Err(e) => {
                    error!("Failed to create Twitter client: {}", e);
                    return;
                }
            };
            
            if let Err(e) = post_tweet(&client, &config_inner).await {
                error!("Failed to post tweet: {}", e);
            }
        })
    })?;

    scheduler.add(tweet_job).await?;

    info!("Scheduler started - posting every {} hours", config.post_interval_hours);
    info!("Expected: {} tweets per day", config.max_posts_per_day);

    // Post one immediately on startup
    info!("🚀 Posting initial tweet...");
    post_tweet(&twitter_client, &config).await?;

    // Start scheduler
    scheduler.start().await?;

    // Keep running
    info!("Bot is now running. Press Ctrl+C to stop.");
    tokio::signal::ctrl_c().await?;
    info!("Shutting down...");

    Ok(())
}

async fn post_tweet(client: &TwitterClient, config: &BotConfig) -> Result<()> {
    info!("Generating tweet...");

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

    let preview = validated_tweet.chars().take(50).collect::<String>();
    info!("Tweet preview: {}...", preview);

    // Post to Twitter
    let response = client.post_tweet(&validated_tweet).await?;

    info!("Tweet posted successfully! ID: {}", response.data.id);

    Ok(())
}

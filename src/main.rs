mod generators;
mod filters;
mod templates;
mod config;
mod twitter;

use anyhow::Result;
use dotenv::dotenv;
use tokio_cron_scheduler::{JobScheduler, Job};
use tracing::{info, warn, error};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{Utc, Datelike};

use generators::TweetGenerator;
use filters::ContentFilter;
use config::BotConfig;
use twitter::TwitterClient;

/// Tracks daily post count and resets each day
struct PostTracker {
    count: u32,
    day: u32,
    max_per_day: u32,
}

impl PostTracker {
    fn new(max_per_day: u32) -> Self {
        Self {
            count: 0,
            day: Utc::now().ordinal(),
            max_per_day,
        }
    }

    /// Returns true if we can post, false if daily limit reached
    fn try_post(&mut self) -> bool {
        let today = Utc::now().ordinal();

        // Reset counter on new day
        if today != self.day {
            self.count = 0;
            self.day = today;
        }

        if self.count >= self.max_per_day {
            return false;
        }

        self.count += 1;
        true
    }
}

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

    // Create Twitter client (shared via Arc)
    let twitter_client = Arc::new(TwitterClient::new()?);
    info!("Twitter client initialized");

    // Create post tracker (shared via Arc<Mutex>)
    let tracker = Arc::new(Mutex::new(PostTracker::new(config.max_posts_per_day)));

    // Create scheduler
    let scheduler = JobScheduler::new().await?;

    // Get cron expression from config
    let cron_expr = config.get_cron_expression();
    info!("Cron schedule: {}", cron_expr);

    // Schedule tweets based on config
    let config_clone = config.clone();
    let client_clone = Arc::clone(&twitter_client);
    let tracker_clone = Arc::clone(&tracker);

    let tweet_job = Job::new_async(cron_expr.as_str(), move |_uuid, _lock| {
        let config_inner = config_clone.clone();
        let client_inner = Arc::clone(&client_clone);
        let tracker_inner = Arc::clone(&tracker_clone);
        Box::pin(async move {
            if let Err(e) = post_tweet(&client_inner, &config_inner, &tracker_inner).await {
                error!("Failed to post tweet: {}", e);
            }
        })
    })?;

    scheduler.add(tweet_job).await?;

    info!("Scheduler started - posting every {} hours", config.post_interval_hours);
    info!("Max: {} tweets per day", config.max_posts_per_day);

    // Post one immediately on startup
    info!("Posting initial tweet...");
    post_tweet(&twitter_client, &config, &tracker).await?;

    // Start scheduler
    scheduler.start().await?;

    // Keep running
    info!("Bot is now running. Press Ctrl+C to stop.");
    tokio::signal::ctrl_c().await?;
    info!("Shutting down...");

    Ok(())
}

async fn post_tweet(
    client: &TwitterClient,
    config: &BotConfig,
    tracker: &Arc<Mutex<PostTracker>>,
) -> Result<()> {
    // Check daily limit
    {
        let mut t = tracker.lock().await;
        if !t.try_post() {
            warn!("Daily post limit ({}) reached, skipping", config.max_posts_per_day);
            return Ok(());
        }
        info!("Post {}/{} for today", t.count, t.max_per_day);
    }

    info!("Generating tweet...");

    // Generate tweet based on config
    let tweet = TweetGenerator::create_tweet(config);

    // Validate
    let validated_tweet = match ContentFilter::validate(tweet) {
        Some(t) => t,
        None => {
            error!("Tweet failed validation, skipping");
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
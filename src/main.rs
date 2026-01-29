mod generators;
mod filters;
mod templates;

use anyhow::Result;
use dotenv::dotenv;
use egg_mode::{KeyPair, Token, tweet::DraftTweet};
use std::env;
use tokio_cron_scheduler::{JobScheduler, Job};
use tracing::{info, error};

use generators::TweetGenerator;
use filters::ContentFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize
    dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("🤖 AgentropicAI Bot starting...");

    // Load Twitter credentials
    let token = load_twitter_token()?;

    // Create scheduler
    let scheduler = JobScheduler::new().await?;

    // Schedule tweets every 6 hours (4 per day)
    let tweet_job = Job::new_async("0 0 */6 * * *", move |_uuid, _lock| {
        let token_clone = token.clone();
        Box::pin(async move {
            if let Err(e) = post_tweet(token_clone).await {
                error!("Failed to post tweet: {}", e);
            }
        })
    })?;

    scheduler.add(tweet_job).await?;

    info!("Scheduler started - posting every 6 hours");
    info!("Expected: 4 tweets per day");

    // Post one immediately on startup
    post_tweet(token.clone()).await?;

    // Start scheduler
    scheduler.start().await?;

    // Keep running
    tokio::signal::ctrl_c().await?;
    info!("Shutting down...");

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

async fn post_tweet(token: Token) -> Result<()> {
    info!("Generating tweet...");

    // Generate tweet
    let tweet = TweetGenerator::create_tweet();

    // Validate
    let validated_tweet = match ContentFilter::validate(tweet) {
        Some(t) => t,
        None => {
            error!("❌ Tweet failed validation, skipping");
            return Ok(());
        }
    };

    info!("Tweet preview: {}", &validated_tweet[..50.min(validated_tweet.len())]);

    // Post to Twitter
    let draft = DraftTweet::new(&validated_tweet);
    draft.send(&token).await?;

    info!("Tweet posted successfully!");

    Ok(())
}

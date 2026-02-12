use tracing::warn;

/// Maximum tweet length (Twitter limit)
const MAX_TWEET_LENGTH: usize = 280;

pub struct ContentFilter;

impl ContentFilter {
    /// List of blocked words/phrases
    fn blocked_terms() -> Vec<&'static str> {
        vec![
            // Scam / spam keywords
            "guaranteed returns",
            "get rich quick",
            "send me crypto",
            "free money",
            "100x guaranteed",
            "not financial advice but buy",
            "pump and dump",
            // Regulatory-risky terms
            "investment advice",
            "guaranteed profit",
            "securities",
            "insider info",
            // Offensive terms (add as needed)
        ]
    }

    /// Check if tweet is safe to post
    pub fn is_safe(tweet: &str) -> bool {
        let lowercase = tweet.to_lowercase();

        // Check for blocked terms
        for term in Self::blocked_terms() {
            if lowercase.contains(term) {
                warn!("Tweet blocked: contains term '{}'", term);
                return false;
            }
        }

        // Check length (Twitter limit)
        if tweet.len() > MAX_TWEET_LENGTH {
            warn!(
                "Tweet blocked: {} chars exceeds {} limit",
                tweet.len(),
                MAX_TWEET_LENGTH
            );
            return false;
        }

        // Must have content
        if tweet.trim().is_empty() {
            warn!("Tweet blocked: empty content");
            return false;
        }

        true
    }

    /// Validate and clean tweet
    pub fn validate(tweet: String) -> Option<String> {
        if Self::is_safe(&tweet) {
            Some(tweet)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_tweet() {
        let tweet = "AI agents are the future!";
        assert!(ContentFilter::is_safe(tweet));
    }

    #[test]
    fn test_too_long() {
        let tweet = "a".repeat(300);
        assert!(!ContentFilter::is_safe(&tweet));
    }

    #[test]
    fn test_empty() {
        assert!(!ContentFilter::is_safe(""));
        assert!(!ContentFilter::is_safe("   "));
    }

    #[test]
    fn test_blocked_scam_terms() {
        assert!(!ContentFilter::is_safe("This is guaranteed returns on your investment"));
        assert!(!ContentFilter::is_safe("FREE MONEY just send me crypto"));
        assert!(!ContentFilter::is_safe("100x guaranteed gains"));
    }

    #[test]
    fn test_blocked_regulatory_terms() {
        assert!(!ContentFilter::is_safe("Here's my investment advice"));
        assert!(!ContentFilter::is_safe("Guaranteed profit if you buy now"));
    }

    #[test]
    fn test_exactly_280_chars() {
        let tweet = "a".repeat(280);
        assert!(ContentFilter::is_safe(&tweet));
    }
}
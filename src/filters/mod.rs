pub struct ContentFilter;

impl ContentFilter {
    /// List of blocked words/phrases
    fn blocked_terms() -> Vec<&'static str> {
        vec![
            // Add offensive terms here
            // Add scam keywords
            // Add regulatory-risky terms
        ]
    }

    /// Check if tweet is safe to post
    pub fn is_safe(tweet: &str) -> bool {
        let lowercase = tweet.to_lowercase();

        // Check for blocked terms
        for term in Self::blocked_terms() {
            if lowercase.contains(term) {
                return false;
            }
        }

        // Check length (Twitter limit: 280 chars)
        if tweet.len() > 280 {
            return false;
        }

        // Must have content
        if tweet.trim().is_empty() {
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
    }
}

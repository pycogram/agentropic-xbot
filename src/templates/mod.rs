use rand::seq::SliceRandom;

pub struct TweetTemplates;

// All templates must be ≤250 chars to leave room for the ~30 char signature.
// The generator will skip the signature if needed, but we aim to always fit.

impl TweetTemplates {
    /// AI-focused posts
    pub fn ai_templates() -> Vec<&'static str> {
        vec![
            "AI agents are evolving faster than most realize. The future is autonomous systems working together.\n\n#AI #Agents #MachineLearning",
            "The real revolution isn't chatbots — it's multi-agent systems. Agents coordinating decisions and action. Pure alpha.\n\n#AI #MultiAgent",
            "Neural networks were just the beginning. Agent swarms are the endgame.\n\n#AI #SwarmIntelligence",
            "AGI won't be one model. It'll be thousands of specialized agents in perfect coordination.\n\n#AGI #Agents",
            "While everyone's playing with prompts, smart money is building autonomous agent systems.\n\n#AI #Automation",
        ]
    }

    /// Agentropic-specific posts
    pub fn agentropic_templates() -> Vec<&'static str> {
        vec![
            "Production-ready multi-agent systems in Rust. BDI architecture, swarm coordination, fault tolerance — batteries included.\n\n#Rust #Agentropic",
            "Your agents deserve Rust's safety and performance. No GC pauses. No Python spaghetti. Just speed.\n\n#Rust #Agentropic",
            "8 org patterns for multi-agent systems: Hierarchy, Swarm, Market, Coalition, Team, Holarchy, Federation, Blackboard. All in Rust.\n\n#Agentropic",
            "FIPA messaging. BDI cognition. Fault-tolerant runtime. Swarm intelligence. All open source.\n\n#Rust #Agentropic #Agents",
            "While others figure out agent basics, Agentropic devs are deploying production swarms. Different game.\n\n#Agentropic #MultiAgent",
        ]
    }

    /// Crypto + AI hybrid posts
    pub fn crypto_ai_templates() -> Vec<&'static str> {
        vec![
            "Blockchain + AI agents: on-chain coordination, autonomous execution, trustless cooperation. The future of DeFi is agentic.\n\n#DeFi #AIAgents",
            "Smart contracts are cool. AI agents executing them autonomously? That's next level.\n\n#AI #Blockchain #DeFi",
            "MEV but it's AI agents competing in milliseconds. That's the meta.\n\n#MEV #AIAgents #Crypto",
            "Every major protocol will have AI agents soon. The ones sleeping on this will regret it.\n\n#DeFi #AI #Agents",
            "Algo trading → AI trading agents → Agent swarms coordinating trades. We're entering the swarm era.\n\n#Crypto #AIAgents",
        ]
    }

    /// Meme coin + AI posts
    pub fn meme_ai_templates() -> Vec<&'static str> {
        vec![
            "AI agent tokens are the new meta. Utility + memes = unstoppable force.\n\n#AI #MemeCoins #Crypto",
            "Imagine: AI agents shitposting their own meme coins into existence. Bullish.\n\n#AIAgents #Memes",
            "Doge had a dog. We have autonomous agents. Different era, same energy.\n\n#AI #MemeCoins",
            "AI tokens aren't just memes. They're infrastructure for autonomous economies. (Also they're memes.)\n\n#AI #Crypto",
            "The best performing asset next year will be an AI agent token nobody's heard of yet. Screenshot this.\n\n#Crypto #AI",
        ]
    }

    /// General posts
    pub fn general_bull_templates() -> Vec<&'static str> {
        vec![
            "Agent economies are coming. Agents trading, coordinating value, building wealth. Humans? Optional.\n\n#AI #Agents #Future",
            "Going from single AI models to multi-agent systems is like going from single-player to MMO. We're going multiplayer.\n\n#AI #Agents",
            "Your next coworker won't be human. It'll be a swarm of specialized AI agents. Get ready.\n\n#AI #FutureOfWork",
            "AI agents don't sleep, don't take breaks, and scale infinitely. The workforce shift is already here.\n\n#AI #Automation",
            "Building AI agents right now is like building websites in 1995. Early. Weird. Massively underpriced.\n\n#AI #Agents #Tech",
        ]
    }

    pub fn random_ai_tweet() -> String {
        Self::ai_templates()
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string()
    }

    pub fn random_agentropic_tweet() -> String {
        Self::agentropic_templates()
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string()
    }

    pub fn random_crypto_tweet() -> String {
        Self::crypto_ai_templates()
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string()
    }

    pub fn random_meme_tweet() -> String {
        Self::meme_ai_templates()
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string()
    }

    pub fn random_bull_tweet() -> String {
        Self::general_bull_templates()
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify every template fits within 250 chars (leaves room for signature)
    #[test]
    fn test_all_templates_under_limit() {
        let max_len = 250;
        let all_templates: Vec<(&str, Vec<&str>)> = vec![
            ("ai", TweetTemplates::ai_templates()),
            ("agentropic", TweetTemplates::agentropic_templates()),
            ("crypto", TweetTemplates::crypto_ai_templates()),
            ("meme", TweetTemplates::meme_ai_templates()),
            ("general", TweetTemplates::general_bull_templates()),
        ];

        for (category, templates) in all_templates {
            for (i, template) in templates.iter().enumerate() {
                assert!(
                    template.len() <= max_len,
                    "Template {}[{}] is {} chars (max {}): {:?}",
                    category,
                    i,
                    template.len(),
                    max_len,
                    &template[..50.min(template.len())]
                );
            }
        }
    }
}
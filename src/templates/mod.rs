use rand::seq::SliceRandom;

pub struct TweetTemplates;

impl TweetTemplates {
    /// AI-focused bull posts
    pub fn ai_templates() -> Vec<&'static str> {
        vec![
            "AI agents are evolving faster than most people realize. The future is autonomous systems working together. \n\n#AI #Agents #MachineLearning",
            "Forget ChatGPT, the real revolution is multi-agent systems. Agents talking to agents, making decisions, coordinating action. Pure alpha. \n\n#AI #MultiAgent",
            "Neural networks were just the beginning. Agent swarms are the endgame. \n\n#AI #SwarmIntelligence",
            "AGI won't be one model. It'll be thousands of specialized agents working in perfect coordination. That's the play. \n\n#AGI #Agents",
            "While everyone's playing with prompts, smart money is building autonomous agent systems. NGMI if you're not watching this space. \n\n#AI #Automation",
        ]
    }

    /// Agentropic-specific posts
    pub fn agentropic_templates() -> Vec<&'static str> {
        vec![
            "Building production-ready multi-agent systems in Rust? @AgentropicAI has entered the chat. \n\nBDI architecture, swarm coordination, fault tolerance - all batteries included.\n\n#Rust #Agentropic #Agents",
            "Agentropic: Because your agents deserve Rust's safety guarantees and performance. \n\nno more Python spaghetti\nno more garbage collection\njust pure speed\n\n#Rust #Agentropic",
            "8 organizational patterns for multi-agent systems:\n• Hierarchy\n• Swarm\n• Market\n• Coalition\n• Team\n• Holarchy\n• Federation  \n• Blackboard\n\nAll production-ready. All in Rust. \n\n#Agentropic",
            "FIPA-compliant agent messaging? \nBDI cognitive architecture? \nFault-tolerant runtime? \nSwarm intelligence? \n\nAgentropic has it all. And it's open source. \n\n#Rust #Agents",
            "While others are still figuring out agent basics, Agentropic devs are deploying production swarms. \n\nDifferent levels. Different game. \n\n#Agentropic #MultiAgent",
        ]
    }

    /// Crypto + AI hybrid posts
    pub fn crypto_ai_templates() -> Vec<&'static str> {
        vec![
            "Blockchain + AI agents = the ultimate combo\n\nOn-chain coordination\nAutonomous execution\nTrustless cooperation\n\nThe future of DeFi is agentic. \n\n#DeFi #AIAgents #Crypto",
            "Smart contracts are cool. AI agents executing smart contracts autonomously? That's god-tier. \n\n#AI #Blockchain #DeFi",
            "MEV but it's AI agents competing in milliseconds. That's the meta. \n\n#MEV #AIAgents #Crypto",
            "Every major protocol will have AI agents by 2026. Those sleeping on this ngmi. \n\n#DeFi #AI #Agents",
            "Algorithmic trading → AI trading agents → Agent swarms coordinating trades\n\nWe're entering the swarm era. \n\n#Crypto #AIAgents",
        ]
    }

    /// Meme coin + AI posts  
    pub fn meme_ai_templates() -> Vec<&'static str> {
        vec![
            "AI agent tokens are the new meta. Utility + memes = unstoppable force. \n\n#AI #MemeCoins #Crypto",
            "Imagine: AI agents shitposting their own meme coins into existence. Bullish. \n\n#AIAgents #Memes",
            "Doge had a dog. We have autonomous agents. Different era, same energy. \n\n#AI #MemeCoins",
            "AI tokens aren't memes. They're the infrastructure for autonomous economies. (Also they're absolutely memes) \n\n#AI #Crypto",
            "The best performing asset of 2025 will be an AI agent token you've never heard of. Screenshot this. \n\n#Crypto #AI",
        ]
    }

    /// General bull posts
    pub fn general_bull_templates() -> Vec<&'static str> {
        vec![
            "Agent economies are coming. Agents trading with agents. Agents coordinating value. Agents building wealth.\n\nHumans? Optional. \n\n#AI #Agents #Future",
            "The transition from single AI models to multi-agent systems is like going from single-player to MMO.\n\nWe're going massively multiplayer. \n\n#AI #Agents",
            "Your next coworker won't be human. It'll be a swarm of specialized AI agents. Get ready. \n\n#AI #FutureOfWork",
            "AI agents don't sleep. They don't take breaks. They scale infinitely. \n\nThis is the workforce replacement everyone's worried about. And it's already here. \n\n#AI #Automation",
            "Building in AI agents right now is like building websites in 1995. Early. Weird. Massively underpriced opportunity. \n\n#AI #Agents #Tech",
        ]
    }

    /// Get random tweet from a category
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

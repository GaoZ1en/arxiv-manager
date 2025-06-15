// AI Assistant Core Engine
// Main AI assistant functionality inspired by GitHub Copilot

use async_trait::async_trait;
use chrono::{Utc, Datelike};
use std::collections::HashMap;
use uuid::Uuid;

use crate::ai::ai_models::*;
use crate::core::models::ArxivPaper;
use crate::utils::Result;

#[derive(Clone, Debug)]
pub struct AiAssistant {
    context: AiContextWindow,
    suggestions_cache: HashMap<String, Vec<AiSuggestion>>,
    chat_history: Vec<AiChatMessage>,
    active_session_id: Option<String>,
}

impl AiAssistant {
    pub fn new() -> Self {
        Self {
            context: AiContextWindow {
                selected_papers: Vec::new(),
                current_search: None,
                research_goals: Vec::new(),
                user_preferences: AiUserPreferences::default(),
            },
            suggestions_cache: HashMap::new(),
            chat_history: Vec::new(),
            active_session_id: None,
        }
    }

    pub fn start_session(&mut self) -> String {
        let session_id = Uuid::new_v4().to_string();
        self.active_session_id = Some(session_id.clone());
        self.chat_history.clear();
        
        // Add system message
        self.add_system_message("AI Assistant activated. I can help you with research, paper analysis, and code suggestions.".to_string());
        
        session_id
    }

    pub fn add_paper_to_context(&mut self, paper: ArxivPaper) {
        if !self.context.selected_papers.iter().any(|p| p.id == paper.id) {
            self.context.selected_papers.push(paper);
            self.invalidate_suggestions_cache();
        }
    }

    pub fn remove_paper_from_context(&mut self, paper_id: &str) {
        self.context.selected_papers.retain(|p| p.id != paper_id);
        self.invalidate_suggestions_cache();
    }

    pub fn update_search_context(&mut self, search_query: String) {
        self.context.current_search = Some(search_query);
        self.invalidate_suggestions_cache();
    }

    pub fn add_research_goal(&mut self, goal: String) {
        if !self.context.research_goals.contains(&goal) {
            self.context.research_goals.push(goal);
        }
    }

    pub fn generate_suggestions(&mut self) -> Vec<AiSuggestion> {
        let cache_key = self.generate_cache_key();
        
        if let Some(cached) = self.suggestions_cache.get(&cache_key) {
            return cached.clone();
        }

        let mut suggestions = Vec::new();

        // Generate different types of suggestions based on context
        suggestions.extend(self.generate_search_suggestions());
        suggestions.extend(self.generate_related_paper_suggestions());
        suggestions.extend(self.generate_code_suggestions());
        suggestions.extend(self.generate_trend_suggestions());

        // Cache the suggestions
        self.suggestions_cache.insert(cache_key, suggestions.clone());
        
        suggestions
    }

    pub async fn chat(&mut self, user_message: String) -> Result<String> {
        // Add user message to history
        let user_msg = AiChatMessage {
            id: Uuid::new_v4().to_string(),
            role: MessageRole::User,
            content: user_message.clone(),
            timestamp: Utc::now(),
            paper_context: self.context.selected_papers.first().cloned(),
        };
        self.chat_history.push(user_msg);

        // Generate AI response based on context
        let response = self.generate_chat_response(&user_message).await?;

        // Add AI response to history
        let ai_msg = AiChatMessage {
            id: Uuid::new_v4().to_string(),
            role: MessageRole::Assistant,
            content: response.clone(),
            timestamp: Utc::now(),
            paper_context: None,
        };
        self.chat_history.push(ai_msg);

        Ok(response)
    }

    pub fn get_chat_history(&self) -> &[AiChatMessage] {
        &self.chat_history
    }

    pub fn clear_chat_history(&mut self) {
        self.chat_history.clear();
    }

    pub fn analyze_paper(&self, paper: &ArxivPaper) -> AiAnalysisResult {
        AiAnalysisResult {
            paper_id: paper.id.clone(),
            summary: self.generate_paper_summary(paper),
            key_points: self.extract_key_points(paper),
            methodology: self.extract_methodology(paper),
            code_availability: self.detect_code_availability(paper),
            dataset_info: self.extract_dataset_info(paper),
            related_topics: self.extract_related_topics(paper),
            complexity_score: self.calculate_complexity_score(paper),
            research_impact: self.estimate_research_impact(paper),
        }
    }

    pub fn suggest_citations(&self, paper: &ArxivPaper) -> Vec<String> {
        let style = &self.context.user_preferences.citation_style;
        vec![self.format_citation(paper, style)]
    }

    pub fn get_context(&self) -> &AiContextWindow {
        &self.context
    }

    pub fn update_preferences(&mut self, preferences: AiUserPreferences) {
        self.context.user_preferences = preferences;
        self.invalidate_suggestions_cache();
    }

    // Private helper methods
    fn add_system_message(&mut self, content: String) {
        let msg = AiChatMessage {
            id: Uuid::new_v4().to_string(),
            role: MessageRole::System,
            content,
            timestamp: Utc::now(),
            paper_context: None,
        };
        self.chat_history.push(msg);
    }

    fn generate_cache_key(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        self.context.selected_papers.len().hash(&mut hasher);
        self.context.current_search.hash(&mut hasher);
        self.context.research_goals.hash(&mut hasher);
        
        format!("cache_{}", hasher.finish())
    }

    fn invalidate_suggestions_cache(&mut self) {
        self.suggestions_cache.clear();
    }

    fn generate_search_suggestions(&self) -> Vec<AiSuggestion> {
        let mut suggestions = Vec::new();

        if let Some(current_search) = &self.context.current_search {
            // Generate enhanced search queries
            let enhanced_queries = vec![
                format!("{} AND machine learning", current_search),
                format!("{} AND deep learning", current_search),
                format!("{} AND neural networks", current_search),
                format!("{} survey", current_search),
                format!("{} recent advances", current_search),
            ];

            for query in enhanced_queries {
                suggestions.push(AiSuggestion {
                    id: Uuid::new_v4().to_string(),
                    suggestion_type: SuggestionType::SearchQuery,
                    title: format!("Enhanced Search: {}", query),
                    description: format!("Try this refined search query for better results"),
                    confidence: 0.8,
                    context: current_search.clone(),
                    created_at: Utc::now(),
                    paper_ids: Vec::new(),
                });
            }
        }

        suggestions
    }

    fn generate_related_paper_suggestions(&self) -> Vec<AiSuggestion> {
        let mut suggestions = Vec::new();

        for paper in &self.context.selected_papers {
            suggestions.push(AiSuggestion {
                id: Uuid::new_v4().to_string(),
                suggestion_type: SuggestionType::RelatedPapers,
                title: format!("Papers related to: {}", paper.title),
                description: "Find papers with similar topics and methodologies".to_string(),
                confidence: 0.9,
                context: paper.title.clone(),
                created_at: Utc::now(),
                paper_ids: vec![paper.id.clone()],
            });
        }

        suggestions
    }

    fn generate_code_suggestions(&self) -> Vec<AiSuggestion> {
        let mut suggestions = Vec::new();

        for paper in &self.context.selected_papers {
            if self.detect_code_availability(paper) {
                suggestions.push(AiSuggestion {
                    id: Uuid::new_v4().to_string(),
                    suggestion_type: SuggestionType::CodeExample,
                    title: format!("Implementation for: {}", paper.title),
                    description: "Generate code examples based on this paper's methodology".to_string(),
                    confidence: 0.7,
                    context: paper.title.clone(),
                    created_at: Utc::now(),
                    paper_ids: vec![paper.id.clone()],
                });
            }
        }

        suggestions
    }

    fn generate_trend_suggestions(&self) -> Vec<AiSuggestion> {
        let mut suggestions = Vec::new();

        // Analyze research trends from selected papers
        let topics = self.extract_common_topics();
        
        for topic in topics {
            suggestions.push(AiSuggestion {
                id: Uuid::new_v4().to_string(),
                suggestion_type: SuggestionType::ResearchTrend,
                title: format!("Trending in: {}", topic),
                description: "Explore the latest developments in this research area".to_string(),
                confidence: 0.6,
                context: topic.clone(),
                created_at: Utc::now(),
                paper_ids: Vec::new(),
            });
        }

        suggestions
    }

    async fn generate_chat_response(&self, user_message: &str) -> Result<String> {
        // This is a simplified response generator
        // In a real implementation, you would integrate with an AI service like OpenAI
        
        let response = if user_message.to_lowercase().contains("summary") {
            self.generate_context_summary()
        } else if user_message.to_lowercase().contains("code") {
            "I can help you generate code examples. Please select a paper you'd like to implement.".to_string()
        } else if user_message.to_lowercase().contains("search") {
            "I can suggest better search queries. What specific topic are you researching?".to_string()
        } else {
            "I'm here to help with your research! You can ask me about paper summaries, code examples, or search suggestions.".to_string()
        };

        Ok(response)
    }

    fn generate_context_summary(&self) -> String {
        if self.context.selected_papers.is_empty() {
            return "No papers selected for analysis.".to_string();
        }

        let paper_count = self.context.selected_papers.len();
        let topics = self.extract_common_topics();
        
        format!(
            "Context Summary: You have {} papers selected. Common topics include: {}. {}",
            paper_count,
            topics.join(", "),
            if let Some(search) = &self.context.current_search {
                format!("Current search: '{}'", search)
            } else {
                "No active search query.".to_string()
            }
        )
    }

    fn generate_paper_summary(&self, paper: &ArxivPaper) -> String {
        // Simplified summary generation
        format!(
            "This paper titled '{}' by {} focuses on {}. Published in {}.",
            paper.title,
            paper.authors.join(", "),
            paper.abstract_text.chars().take(200).collect::<String>(),
            paper.published
        )
    }

    fn extract_key_points(&self, paper: &ArxivPaper) -> Vec<String> {
        // Simplified key point extraction
        vec![
            "Novel methodology presented".to_string(),
            "Experimental validation provided".to_string(),
            "Performance improvements demonstrated".to_string(),
        ]
    }

    fn extract_methodology(&self, paper: &ArxivPaper) -> Option<String> {
        if paper.abstract_text.to_lowercase().contains("method") {
            Some("Machine learning methodology identified".to_string())
        } else {
            None
        }
    }

    fn detect_code_availability(&self, paper: &ArxivPaper) -> bool {
        let indicators = ["github", "code", "implementation", "repository", "open source"];
        indicators.iter().any(|&indicator| {
            paper.abstract_text.to_lowercase().contains(indicator) ||
            paper.title.to_lowercase().contains(indicator)
        })
    }

    fn extract_dataset_info(&self, paper: &ArxivPaper) -> Option<String> {
        if paper.abstract_text.to_lowercase().contains("dataset") {
            Some("Dataset information available".to_string())
        } else {
            None
        }
    }

    fn extract_related_topics(&self, paper: &ArxivPaper) -> Vec<String> {
        // Simplified topic extraction
        vec!["Machine Learning".to_string(), "Deep Learning".to_string()]
    }

    fn calculate_complexity_score(&self, paper: &ArxivPaper) -> f32 {
        // Simplified complexity calculation
        let complexity_indicators = ["complex", "advanced", "novel", "state-of-the-art"];
        let score = complexity_indicators.iter()
            .map(|&indicator| if paper.abstract_text.to_lowercase().contains(indicator) { 0.25 } else { 0.0 })
            .sum::<f32>();
        score.min(1.0)
    }

    fn estimate_research_impact(&self, _paper: &ArxivPaper) -> f32 {
        // Simplified impact estimation
        0.7 // Default impact score
    }

    fn format_citation(&self, paper: &ArxivPaper, style: &CitationStyle) -> String {
        match style {
            CitationStyle::APA => {
                format!(
                    "{}. ({}). {}. arXiv preprint arXiv:{}.",
                    paper.authors.join(", "),
                    paper.published,
                    paper.title,
                    paper.id
                )
            },
            CitationStyle::IEEE => {
                format!(
                    "{}, \"{}\", arXiv preprint arXiv:{}, {}.",
                    paper.authors.join(", "),
                    paper.title,
                    paper.id,
                    paper.published
                )
            },
            _ => {
                format!(
                    "{}. {}. arXiv:{} ({})",
                    paper.authors.join(", "),
                    paper.title,
                    paper.id,
                    paper.published
                )
            }
        }
    }

    fn extract_common_topics(&self) -> Vec<String> {
        // Simplified topic extraction from selected papers
        let mut topics = Vec::new();
        
        for paper in &self.context.selected_papers {
            if paper.abstract_text.to_lowercase().contains("machine learning") {
                topics.push("Machine Learning".to_string());
            }
            if paper.abstract_text.to_lowercase().contains("deep learning") {
                topics.push("Deep Learning".to_string());
            }
            if paper.abstract_text.to_lowercase().contains("neural network") {
                topics.push("Neural Networks".to_string());
            }
        }
        
        topics.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect()
    }
}

impl Default for AiAssistant {
    fn default() -> Self {
        Self::new()
    }
}

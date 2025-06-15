// AI Assistant Data Models
// Defines structures for AI assistance functionality

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::core::models::ArxivPaper;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSuggestion {
    pub id: String,
    pub suggestion_type: SuggestionType,
    pub title: String,
    pub description: String,
    pub confidence: f32,
    pub context: String,
    pub created_at: DateTime<Utc>,
    pub paper_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionType {
    SearchQuery,
    RelatedPapers,
    ResearchTrend,
    CodeExample,
    Summary,
    Citation,
    Collaboration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAnalysisResult {
    pub paper_id: String,
    pub summary: String,
    pub key_points: Vec<String>,
    pub methodology: Option<String>,
    pub code_availability: bool,
    pub dataset_info: Option<String>,
    pub related_topics: Vec<String>,
    pub complexity_score: f32,
    pub research_impact: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiChatMessage {
    pub id: String,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub paper_context: Option<ArxivPaper>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiContextWindow {
    pub selected_papers: Vec<ArxivPaper>,
    pub current_search: Option<String>,
    pub research_goals: Vec<String>,
    pub user_preferences: AiUserPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiUserPreferences {
    pub research_areas: Vec<String>,
    pub preferred_complexity: ComplexityLevel,
    pub language_preference: String,
    pub citation_style: CitationStyle,
    pub auto_suggestions: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CitationStyle {
    APA,
    MLA,
    IEEE,
    Nature,
    Science,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSuggestion {
    pub id: String,
    pub language: String,
    pub code_snippet: String,
    pub explanation: String,
    pub related_paper: Option<String>,
    pub dependencies: Vec<String>,
    pub usage_example: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchTrend {
    pub topic: String,
    pub trend_score: f32,
    pub paper_count: u32,
    pub recent_papers: Vec<String>,
    pub key_researchers: Vec<String>,
    pub emerging_keywords: Vec<String>,
}

impl Default for AiUserPreferences {
    fn default() -> Self {
        Self {
            research_areas: vec!["Machine Learning".to_string(), "Computer Science".to_string()],
            preferred_complexity: ComplexityLevel::Intermediate,
            language_preference: "English".to_string(),
            citation_style: CitationStyle::APA,
            auto_suggestions: true,
        }
    }
}

impl std::fmt::Display for SuggestionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SuggestionType::SearchQuery => write!(f, "Search Query"),
            SuggestionType::RelatedPapers => write!(f, "Related Papers"),
            SuggestionType::ResearchTrend => write!(f, "Research Trend"),
            SuggestionType::CodeExample => write!(f, "Code Example"),
            SuggestionType::Summary => write!(f, "Summary"),
            SuggestionType::Citation => write!(f, "Citation"),
            SuggestionType::Collaboration => write!(f, "Collaboration"),
        }
    }
}

impl std::fmt::Display for ComplexityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComplexityLevel::Beginner => write!(f, "Beginner"),
            ComplexityLevel::Intermediate => write!(f, "Intermediate"),
            ComplexityLevel::Advanced => write!(f, "Advanced"),
            ComplexityLevel::Expert => write!(f, "Expert"),
        }
    }
}

impl std::fmt::Display for CitationStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CitationStyle::APA => write!(f, "APA"),
            CitationStyle::MLA => write!(f, "MLA"),
            CitationStyle::IEEE => write!(f, "IEEE"),
            CitationStyle::Nature => write!(f, "Nature"),
            CitationStyle::Science => write!(f, "Science"),
        }
    }
}

// AI Handler - Message processing and state management for AI features
// Integrates AI capabilities into the main application flow

use async_trait::async_trait;
use std::collections::HashMap;

use crate::ai::{AiAssistant, AiSuggestion, AiAnalysisResult, AiChatMessage, AiUserPreferences};
use crate::core::models::ArxivPaper;
use crate::utils::Result;

#[derive(Debug, Clone)]
pub enum AiMessage {
    // Chat interactions
    StartChatSession,
    SendChatMessage(String),
    ChatResponseReceived(std::result::Result<String, String>),
    ClearChatHistory,
    
    // Suggestions
    GenerateSuggestions,
    SuggestionsGenerated(Vec<AiSuggestion>),
    ApplySuggestion(String), // suggestion_id
    RateSuggestion { id: String, rating: f32 },
    
    // Paper analysis
    AnalyzePaper(ArxivPaper),
    AnalysisCompleted(AiAnalysisResult),
    AnalyzeSelectedPapers,
    
    // Context management
    AddPaperToContext(ArxivPaper),
    RemovePaperFromContext(String),
    UpdateSearchContext(String),
    AddResearchGoal(String),
    
    // Preferences
    UpdateAiPreferences(AiUserPreferences),
    
    // AI Assistant toggle
    ToggleAiAssistant,
    AiAssistantVisibilityChanged(bool),
    
    // Code generation
    GenerateCodeForPaper(String), // paper_id
    CodeGenerated { paper_id: String, code: String },
    
    // Research insights
    GenerateResearchInsights,
    ResearchInsightsGenerated(String),
}

#[derive(Debug, Clone)]
pub struct AiState {
    pub assistant: AiAssistant,
    pub is_visible: bool,
    pub current_suggestions: Vec<AiSuggestion>,
    pub chat_messages: Vec<AiChatMessage>,
    pub analysis_results: HashMap<String, AiAnalysisResult>,
    pub is_generating: bool,
    pub active_session_id: Option<String>,
    pub generated_code: HashMap<String, String>, // paper_id -> code
}

impl AiState {
    pub fn new() -> Self {
        Self {
            assistant: AiAssistant::new(),
            is_visible: false,
            current_suggestions: Vec::new(),
            chat_messages: Vec::new(),
            analysis_results: HashMap::new(),
            is_generating: false,
            active_session_id: None,
            generated_code: HashMap::new(),
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
        if self.is_visible && self.active_session_id.is_none() {
            self.active_session_id = Some(self.assistant.start_session());
        }
    }

    pub fn add_paper_to_context(&mut self, paper: ArxivPaper) {
        self.assistant.add_paper_to_context(paper);
        // Regenerate suggestions when context changes
        self.current_suggestions = self.assistant.generate_suggestions();
    }

    pub fn remove_paper_from_context(&mut self, paper_id: &str) {
        self.assistant.remove_paper_from_context(paper_id);
        self.current_suggestions = self.assistant.generate_suggestions();
    }

    pub fn update_search_context(&mut self, search_query: String) {
        self.assistant.update_search_context(search_query);
        self.current_suggestions = self.assistant.generate_suggestions();
    }

    pub async fn send_chat_message(&mut self, message: String) -> Result<String> {
        self.is_generating = true;
        let response = self.assistant.chat(message).await;
        self.is_generating = false;
        
        // Update chat messages from assistant
        self.chat_messages = self.assistant.get_chat_history().to_vec();
        
        response
    }

    pub fn generate_suggestions(&mut self) {
        self.current_suggestions = self.assistant.generate_suggestions();
    }

    pub fn analyze_paper(&mut self, paper: &ArxivPaper) -> AiAnalysisResult {
        let analysis = self.assistant.analyze_paper(paper);
        self.analysis_results.insert(paper.id.clone(), analysis.clone());
        analysis
    }

    pub fn apply_suggestion(&mut self, suggestion_id: &str) -> Option<AiMessage> {
        if let Some(suggestion) = self.current_suggestions.iter().find(|s| s.id == suggestion_id) {
            match suggestion.suggestion_type {
                crate::ai::SuggestionType::SearchQuery => {
                    Some(AiMessage::UpdateSearchContext(suggestion.context.clone()))
                },
                crate::ai::SuggestionType::CodeExample => {
                    if let Some(paper_id) = suggestion.paper_ids.first() {
                        Some(AiMessage::GenerateCodeForPaper(paper_id.clone()))
                    } else {
                        None
                    }
                },
                _ => None
            }
        } else {
            None
        }
    }

    pub fn generate_code_for_paper(&mut self, paper_id: &str) -> Option<String> {
        // This would typically call an AI service to generate code
        // For now, we'll return a template
        let code = format!(
            r#"# Implementation for paper: {}
# This is an AI-generated code template

import numpy as np
import matplotlib.pyplot as plt

class PaperImplementation:
    def __init__(self):
        """Initialize the implementation based on the paper's methodology."""
        pass
    
    def train(self, data):
        """Train the model using the described approach."""
        pass
    
    def predict(self, input_data):
        """Make predictions using the trained model."""
        pass
    
    def evaluate(self, test_data):
        """Evaluate the model performance."""
        pass

# Example usage
if __name__ == "__main__":
    model = PaperImplementation()
    # Add your implementation here
    print("Implementation ready!")
"#,
            paper_id
        );
        
        self.generated_code.insert(paper_id.to_string(), code.clone());
        Some(code)
    }

    pub fn clear_chat_history(&mut self) {
        self.assistant.clear_chat_history();
        self.chat_messages.clear();
    }

    pub fn update_preferences(&mut self, preferences: AiUserPreferences) {
        self.assistant.update_preferences(preferences);
        // Regenerate suggestions with new preferences
        self.current_suggestions = self.assistant.generate_suggestions();
    }

    pub fn get_context_summary(&self) -> String {
        let context = self.assistant.get_context();
        format!(
            "Selected papers: {}, Research goals: {}, Current search: {}",
            context.selected_papers.len(),
            context.research_goals.len(),
            context.current_search.as_deref().unwrap_or("None")
        )
    }
}

impl Default for AiState {
    fn default() -> Self {
        Self::new()
    }
}

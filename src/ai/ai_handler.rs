// AI Handler - Message processing and state management for AI features
// Integrates AI capabilities into the main application flow

use std::collections::HashMap;

use crate::ai::{AiAssistant, AiSuggestion, AiAnalysisResult, AiChatMessage, AiUserPreferences};
use crate::core::models::ArxivPaper;
use crate::utils::Result;

#[derive(Debug, Clone)]
pub enum AiMessage {
    // Chat interactions
    StartChatSession,
    UpdateChatInput(String),  // 新增：更新输入内容，不发送消息
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
    
    // Smart suggestions and Copilot-like features
    GenerateInputSuggestions(String), // 根据输入生成建议
    InputSuggestionsGenerated(Vec<String>), // 生成的输入建议
    ApplyInputSuggestion(String), // 应用输入建议
    
    // Copilot-like intelligent assistance
    GenerateSmartCompletion(String), // 智能补全
    SmartCompletionGenerated(String), // 生成的智能补全
    
    // Research workflow assistance
    GenerateWorkflowSuggestions, // 生成工作流建议
    WorkflowSuggestionsGenerated(Vec<AiSuggestion>), // 工作流建议
    
    // Research insights generation
    GenerateResearchInsights, // 生成研究洞察
    ResearchInsightsGenerated(Vec<String>), // 研究洞察结果
    
    // GitHub Copilot Integration
    InitializeCopilot, // 初始化 GitHub Copilot
    CopilotInitialized(std::result::Result<(), String>), // Copilot 初始化结果
    CopilotAuthStatus(crate::ai::CopilotAuth), // Copilot 认证状态
    OpenDocumentInCopilot { uri: String, content: String, language: String }, // 在 Copilot 中打开文档
    UpdateDocumentInCopilot { content: String, version: i32 }, // 更新 Copilot 文档
    GetCopilotCompletions(lsp_types::Position), // 获取 Copilot 补全
    GetCopilotInlineCompletions(lsp_types::Position), // 获取 Copilot 内联补全
    CopilotCompletionsReceived(Vec<crate::ai::CopilotSuggestion>), // Copilot 补全结果
    ApplyCopilotSuggestion(String), // 应用 Copilot 建议
    CopilotSignIn, // Copilot 登录
    CopilotSignOut, // Copilot 登出
}

#[derive(Debug, Clone)]
pub struct AiState {
    pub assistant: AiAssistant,
    pub is_visible: bool,
    pub current_input: String,  // 新增：当前输入的文本
    pub current_suggestions: Vec<AiSuggestion>,
    pub chat_messages: Vec<AiChatMessage>,
    pub analysis_results: HashMap<String, AiAnalysisResult>,
    pub is_generating: bool,
    pub active_session_id: Option<String>,
    pub generated_code: HashMap<String, String>, // paper_id -> code
    
    // GitHub Copilot integration
    pub copilot_suggestions: Vec<crate::ai::CopilotSuggestion>,
    pub copilot_auth_status: Option<crate::ai::CopilotAuth>,
    pub copilot_enabled: bool,
    pub current_document_version: i32,
}

impl AiState {
    pub fn new() -> Self {
        Self {
            assistant: AiAssistant::new(),
            is_visible: false,
            current_input: String::new(),  // 初始化为空字符串
            current_suggestions: Vec::new(),
            chat_messages: Vec::new(),
            analysis_results: HashMap::new(),
            is_generating: false,
            active_session_id: None,
            generated_code: HashMap::new(),
            
            // GitHub Copilot fields
            copilot_suggestions: Vec::new(),
            copilot_auth_status: None,
            copilot_enabled: false,
            current_document_version: 0,
        }
    }

    pub fn toggle_visibility(&mut self) {
        self.is_visible = !self.is_visible;
        if self.is_visible && self.active_session_id.is_none() {
            self.active_session_id = Some(self.assistant.start_session());
        }
    }

    pub fn update_input(&mut self, input: String) {
        self.current_input = input;
    }

    pub fn clear_input(&mut self) {
        self.current_input.clear();
    }

    pub fn add_paper_to_context(&mut self, paper: ArxivPaper) {
        self.assistant.add_paper_to_context(paper);
        // Note: suggestions will be regenerated async when needed
    }

    pub fn remove_paper_from_context(&mut self, paper_id: &str) {
        self.assistant.remove_paper_from_context(paper_id);
        // Note: suggestions will be regenerated async when needed
    }

    pub fn update_search_context(&mut self, search_query: String) {
        self.assistant.update_search_context(search_query);
        // Note: suggestions will be regenerated async when needed
    }

    pub async fn send_chat_message(&mut self, message: String) -> Result<String> {
        self.is_generating = true;
        let response = self.assistant.chat(message).await;
        self.is_generating = false;
        
        // Update chat messages from assistant
        self.chat_messages = self.assistant.get_chat_history().to_vec();
        
        response
    }

    pub async fn generate_suggestions(&mut self) {
        match self.assistant.generate_suggestions().await {
            suggestions => self.current_suggestions = suggestions,
        }
    }

    pub async fn analyze_paper(&mut self, paper: &ArxivPaper) -> AiAnalysisResult {
        let analysis = self.assistant.analyze_paper(paper).await;
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

    pub async fn generate_code_for_paper(&mut self, paper_id: &str) -> Option<String> {
        // Find the paper in context
        if let Some(paper) = self.assistant.get_context().selected_papers.iter().find(|p| p.id == paper_id) {
            // Generate code using AI service
            match self.assistant.generate_code_example(paper, "python").await {
                Ok(code) => {
                    self.generated_code.insert(paper_id.to_string(), code.clone());
                    Some(code)
                },
                Err(_) => {
                    // Fallback code generation
                    let fallback_code = format!(
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
                        paper.title
                    );
                    self.generated_code.insert(paper_id.to_string(), fallback_code.clone());
                    Some(fallback_code)
                }
            }
        } else {
            None
        }
    }

    pub fn clear_chat_history(&mut self) {
        self.assistant.clear_chat_history();
        self.chat_messages.clear();
    }

    pub fn update_preferences(&mut self, preferences: AiUserPreferences) {
        self.assistant.update_preferences(preferences);
        // Note: suggestions will be regenerated async when needed
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

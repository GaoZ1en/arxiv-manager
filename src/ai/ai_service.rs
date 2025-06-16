// AI Service - Real AI integration for intelligent responses
// Integrates with OpenAI API and other AI services

use async_openai::{
    types::{ChatCompletionRequestMessage, CreateChatCompletionRequest, Role, ChatCompletionRequestUserMessage},
    Client,
};
use anyhow::{anyhow, Result as AnyhowResult};
use std::env;

use crate::ai::ai_models::*;
use crate::core::models::ArxivPaper;

#[derive(Clone, Debug)]
pub struct AiService {
    client: Option<Client<async_openai::config::OpenAIConfig>>,
    model: String,
    is_enabled: bool,
}

impl AiService {
    pub fn new() -> Self {
        let api_key = env::var("OPENAI_API_KEY").ok();
        let is_enabled = api_key.as_ref().map_or(false, |key| !key.is_empty());
        let client = if let Some(key) = api_key {
            if !key.is_empty() {
                Some(Client::with_config(async_openai::config::OpenAIConfig::new().with_api_key(key)))
            } else {
                None
            }
        } else {
            None
        };

        Self {
            client,
            model: "gpt-4".to_string(),
            is_enabled,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    pub async fn generate_chat_response(
        &self,
        user_message: &str,
        chat_history: &[AiChatMessage],
        context: &AiContextWindow,
    ) -> AnyhowResult<String> {
        if !self.is_enabled {
            return Ok(self.generate_fallback_response(user_message, context));
        }

        let client = self.client.as_ref().unwrap();

        // Build context-aware system prompt
        let system_prompt = self.build_system_prompt(context);
        
        // Convert chat history to OpenAI format
        let mut messages = vec![
            ChatCompletionRequestMessage::System(
                async_openai::types::ChatCompletionRequestSystemMessage {
                    content: system_prompt.into(),
                    name: None,
                }
            )
        ];

        // Add recent chat history (last 10 messages to stay within token limits)
        for msg in chat_history.iter().rev().take(10).rev() {
            let role = match msg.role {
                MessageRole::User => Role::User,
                MessageRole::Assistant => Role::Assistant,
                MessageRole::System => Role::System,
            };
            
            match role {
                Role::User => {
                    messages.push(ChatCompletionRequestMessage::User(
                        ChatCompletionRequestUserMessage {
                            content: msg.content.clone().into(),
                            name: None,
                        }
                    ));
                },
                Role::Assistant => {
                    messages.push(ChatCompletionRequestMessage::Assistant(
                        async_openai::types::ChatCompletionRequestAssistantMessage {
                            content: Some(async_openai::types::ChatCompletionRequestAssistantMessageContent::Text(msg.content.clone())),
                            name: None,
                            tool_calls: None,
                            function_call: None,
                            audio: None,
                            refusal: None,
                        }
                    ));
                },
                Role::System => {
                    messages.push(ChatCompletionRequestMessage::System(
                        async_openai::types::ChatCompletionRequestSystemMessage {
                            content: msg.content.clone().into(),
                            name: None,
                        }
                    ));
                },
                _ => {
                    // Handle other roles like Tool and Function if needed
                    // For now, just skip them
                }
            }
        }

        // Add current user message
        messages.push(ChatCompletionRequestMessage::User(
            ChatCompletionRequestUserMessage {
                content: user_message.to_string().into(),
                name: None,
            }
        ));

        let request = CreateChatCompletionRequest {
            model: self.model.clone(),
            messages,
            temperature: Some(0.7),
            max_completion_tokens: Some(500),
            ..Default::default()
        };

        let response = client.chat().create(request).await
            .map_err(|e| anyhow!("OpenAI API error: {}", e))?;

        let content = response.choices.first()
            .and_then(|choice| choice.message.content.as_ref())
            .ok_or_else(|| anyhow!("No response content from AI"))?;

        Ok(content.clone())
    }

    pub async fn analyze_paper(&self, paper: &ArxivPaper) -> AnyhowResult<AiAnalysisResult> {
        let analysis_prompt = format!(
            "Analyze this research paper and provide detailed insights:\n\n\
            Title: {}\n\
            Authors: {}\n\
            Abstract: {}\n\n\
            Please provide:\n\
            1. A concise summary (2-3 sentences)\n\
            2. Key contributions and findings\n\
            3. Methodology used\n\
            4. Potential impact and applications\n\
            5. Any code/dataset availability mentioned\n\
            6. Related research areas",
            paper.title,
            paper.authors.join(", "),
            paper.abstract_text
        );

        let response = if self.is_enabled {
            self.generate_ai_analysis(&analysis_prompt).await?
        } else {
            self.generate_fallback_analysis(paper)
        };

        Ok(self.parse_analysis_response(paper, &response))
    }

    pub async fn generate_suggestions(&self, context: &AiContextWindow) -> AnyhowResult<Vec<AiSuggestion>> {
        if context.selected_papers.is_empty() {
            return Ok(self.generate_general_suggestions());
        }

        let suggestions_prompt = self.build_suggestions_prompt(context);
        
        let response = if self.is_enabled {
            self.generate_ai_suggestions(&suggestions_prompt).await?
        } else {
            self.generate_fallback_suggestions(context)
        };

        Ok(self.parse_suggestions_response(&response))
    }

    pub async fn generate_code_example(&self, paper: &ArxivPaper, language: &str) -> AnyhowResult<String> {
        let code_prompt = format!(
            "Generate a practical code example implementing the main algorithm or concept from this paper:\n\n\
            Title: {}\n\
            Abstract: {}\n\n\
            Please provide a {language} implementation with:\n\
            1. Clear comments explaining the approach\n\
            2. Realistic example usage\n\
            3. Any necessary imports/dependencies\n\
            4. Error handling where appropriate",
            paper.title,
            paper.abstract_text,
            language = language
        );

        if self.is_enabled {
            self.generate_ai_code(&code_prompt).await
        } else {
            Ok(self.generate_fallback_code(paper, language))
        }
    }

    // Private helper methods

    fn build_system_prompt(&self, context: &AiContextWindow) -> String {
        let mut prompt = "You are an intelligent AI research assistant specializing in academic papers and scientific research. You help researchers understand papers, find relevant work, and generate useful code examples.\n\n".to_string();

        if !context.selected_papers.is_empty() {
            prompt.push_str("Current context:\n");
            for paper in &context.selected_papers {
                prompt.push_str(&format!("- Paper: {} by {}\n", paper.title, paper.authors.join(", ")));
            }
            prompt.push('\n');
        }

        if let Some(search) = &context.current_search {
            prompt.push_str(&format!("Current research focus: {}\n\n", search));
        }

        if !context.research_goals.is_empty() {
            prompt.push_str("Research goals:\n");
            for goal in &context.research_goals {
                prompt.push_str(&format!("- {}\n", goal));
            }
            prompt.push('\n');
        }

        prompt.push_str("Please provide helpful, accurate, and contextually relevant responses.");
        prompt
    }

    fn build_suggestions_prompt(&self, context: &AiContextWindow) -> String {
        format!(
            "Based on the following research context, generate 5 intelligent suggestions:\n\n\
            Selected papers: {}\n\
            Current search: {}\n\
            Research goals: {}\n\n\
            Provide suggestions for: search queries, related papers to find, code implementations, and research directions.",
            context.selected_papers.iter().map(|p| p.title.as_str()).collect::<Vec<_>>().join(", "),
            context.current_search.as_deref().unwrap_or("None"),
            context.research_goals.join(", ")
        )
    }

    async fn generate_ai_analysis(&self, prompt: &str) -> AnyhowResult<String> {
        if let Some(client) = &self.client {
            let request = CreateChatCompletionRequest {
                model: self.model.clone(),
                messages: vec![
                    ChatCompletionRequestMessage::User(
                        ChatCompletionRequestUserMessage {
                            content: prompt.to_string().into(),
                            name: None,
                        }
                    )
                ],
                temperature: Some(0.3),
                max_completion_tokens: Some(800),
                ..Default::default()
            };

            let response = client.chat().create(request).await
                .map_err(|e| anyhow!("OpenAI API error: {}", e))?;

            response.choices.first()
                .and_then(|choice| choice.message.content.as_ref())
                .cloned()
                .ok_or_else(|| anyhow!("No analysis content from AI"))
        } else {
            Err(anyhow!("AI client not available"))
        }
    }

    async fn generate_ai_suggestions(&self, prompt: &str) -> AnyhowResult<String> {
        if let Some(client) = &self.client {
            let request = CreateChatCompletionRequest {
                model: self.model.clone(),
                messages: vec![
                    ChatCompletionRequestMessage::User(
                        ChatCompletionRequestUserMessage {
                            content: prompt.to_string().into(),
                            name: None,
                        }
                    )
                ],
                temperature: Some(0.5),
                max_completion_tokens: Some(600),
                ..Default::default()
            };

            let response = client.chat().create(request).await
                .map_err(|e| anyhow!("OpenAI API error: {}", e))?;

            response.choices.first()
                .and_then(|choice| choice.message.content.as_ref())
                .cloned()
                .ok_or_else(|| anyhow!("No suggestions content from AI"))
        } else {
            Err(anyhow!("AI client not available"))
        }
    }

    async fn generate_ai_code(&self, prompt: &str) -> AnyhowResult<String> {
        if let Some(client) = &self.client {
            let request = CreateChatCompletionRequest {
                model: self.model.clone(),
                messages: vec![
                    ChatCompletionRequestMessage::User(
                        ChatCompletionRequestUserMessage {
                            content: prompt.to_string().into(),
                            name: None,
                        }
                    )
                ],
                temperature: Some(0.2),
                max_completion_tokens: Some(1000),
                ..Default::default()
            };

            let response = client.chat().create(request).await
                .map_err(|e| anyhow!("OpenAI API error: {}", e))?;

            response.choices.first()
                .and_then(|choice| choice.message.content.as_ref())
                .cloned()
                .ok_or_else(|| anyhow!("No code content from AI"))
        } else {
            Err(anyhow!("AI client not available"))
        }
    }

    // Fallback methods for when AI service is not available

    fn generate_fallback_response(&self, user_message: &str, context: &AiContextWindow) -> String {
        let msg_lower = user_message.to_lowercase();
        
        if msg_lower.contains("summary") || msg_lower.contains("summarize") {
            if !context.selected_papers.is_empty() {
                format!("Based on your selected papers, here's a summary: The main topics appear to focus on {}. Consider exploring related work in these areas.", 
                    context.selected_papers.iter().map(|p| p.title.clone()).collect::<Vec<_>>().join(", "))
            } else {
                "Please select some papers first, and I can provide a summary of their content and connections.".to_string()
            }
        } else if msg_lower.contains("code") || msg_lower.contains("implement") {
            "I can help generate code examples! Please select a specific paper, and I'll create implementation examples based on the methodology described.".to_string()
        } else if msg_lower.contains("search") || msg_lower.contains("find") {
            "For better search results, try specific technical terms from your field. I can suggest related keywords based on papers you've already selected.".to_string()
        } else if msg_lower.contains("related") || msg_lower.contains("similar") {
            "To find related papers, I can analyze the authors, keywords, and citations from your current selection. Add more papers to improve recommendations.".to_string()
        } else {
            format!("I'm here to help with your research! I can assist with:\n\
                    • Paper summaries and analysis\n\
                    • Code implementation examples\n\
                    • Search suggestions\n\
                    • Finding related work\n\n\
                    Currently you have {} papers selected. What would you like to explore?", 
                    context.selected_papers.len())
        }
    }

    fn generate_fallback_analysis(&self, paper: &ArxivPaper) -> String {
        format!(
            "## Analysis Summary\n\
            This paper titled \"{}\" by {} presents research in the field indicated by its abstract. \
            \n\n## Key Insights\n\
            - The methodology appears to focus on the approaches mentioned in the abstract\n\
            - This work contributes to the research area by advancing understanding\n\
            - Consider reviewing the references for related work\n\
            - The findings may have applications in related domains\n\n\
            Note: Enable AI service for detailed analysis.",
            paper.title,
            paper.authors.join(", ")
        )
    }

    fn generate_general_suggestions(&self) -> Vec<AiSuggestion> {
        vec![
            AiSuggestion {
                id: uuid::Uuid::new_v4().to_string(),
                title: "Start with a focused search".to_string(),
                description: "Begin by searching for papers in your specific research area".to_string(),
                suggestion_type: SuggestionType::SearchQuery,
                confidence: 0.9,
                context: "general_search".to_string(),
                created_at: chrono::Utc::now(),
                paper_ids: Vec::new(),
            },
            AiSuggestion {
                id: uuid::Uuid::new_v4().to_string(),
                title: "Explore trending topics".to_string(),
                description: "Check out recent papers in popular research areas".to_string(),
                suggestion_type: SuggestionType::ResearchTrend,
                confidence: 0.8,
                context: "trending_topics".to_string(),
                created_at: chrono::Utc::now(),
                paper_ids: Vec::new(),
            },
        ]
    }

    fn generate_fallback_suggestions(&self, context: &AiContextWindow) -> String {
        format!("Based on your {} selected papers, here are some suggestions:\n\
                1. Search for more recent work by the same authors\n\
                2. Look for papers citing these works\n\
                3. Explore related methodologies\n\
                4. Consider implementation challenges\n\
                5. Review dataset availability",
                context.selected_papers.len())
    }

    fn generate_fallback_code(&self, paper: &ArxivPaper, language: &str) -> String {
        format!(
            "// Code example for: {}\n\
            // Based on the methodology described in the paper\n\
            // Language: {}\n\n\
            // TODO: This is a template - enable AI service for actual implementation\n\
            \n\
            fn main() {{\n\
                println!(\"Implementation of: {}\");\n\
                // Add specific algorithm implementation here\n\
            }}",
            paper.title,
            language,
            paper.title
        )
    }

    fn parse_analysis_response(&self, paper: &ArxivPaper, response: &str) -> AiAnalysisResult {
        // Simple parsing - in real implementation, could use more sophisticated parsing
        let lines: Vec<&str> = response.lines().collect();
        
        AiAnalysisResult {
            paper_id: paper.id.clone(),
            summary: lines.iter().take(3).map(|s| s.to_string()).collect::<Vec<_>>().join(" "),
            key_points: vec![
                "Key insight 1".to_string(),
                "Key insight 2".to_string(),
                "Key insight 3".to_string(),
            ],
            methodology: Some("Methodology extracted from analysis".to_string()),
            code_availability: response.to_lowercase().contains("code") || response.to_lowercase().contains("github"),
            dataset_info: if response.to_lowercase().contains("dataset") { 
                Some("Dataset information available".to_string()) 
            } else { 
                None 
            },
            related_topics: vec!["Related topic 1".to_string(), "Related topic 2".to_string()],
            complexity_score: 0.7,
            research_impact: 0.8,
        }
    }

    fn parse_suggestions_response(&self, response: &str) -> Vec<AiSuggestion> {
        // Simple parsing - could be more sophisticated
        response.lines()
            .filter(|line| !line.trim().is_empty())
            .take(5)
            .enumerate()
            .map(|(i, line)| AiSuggestion {
                id: uuid::Uuid::new_v4().to_string(),
                title: format!("Suggestion {}", i + 1),
                description: line.trim().to_string(),
                suggestion_type: if line.to_lowercase().contains("search") {
                    SuggestionType::SearchQuery
                } else if line.to_lowercase().contains("code") {
                    SuggestionType::CodeExample
                } else {
                    SuggestionType::ResearchTrend
                },
                confidence: 0.8 - (i as f32 * 0.1),
                context: line.trim().to_string(),
                created_at: chrono::Utc::now(),
                paper_ids: Vec::new(),
            })
            .collect()
    }
}

impl Default for AiService {
    fn default() -> Self {
        Self::new()
    }
}

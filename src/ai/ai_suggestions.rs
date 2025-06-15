// AI Suggestions Engine
// Advanced suggestion algorithms inspired by GitHub Copilot

use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use chrono::Utc;

use crate::ai::ai_models::*;
use crate::core::models::ArxivPaper;

pub struct AiSuggestionsEngine {
    suggestion_history: Vec<AiSuggestion>,
    user_interactions: HashMap<String, f32>, // suggestion_id -> user_rating
    learned_patterns: HashMap<String, f32>,  // pattern -> relevance_score
}

impl AiSuggestionsEngine {
    pub fn new() -> Self {
        Self {
            suggestion_history: Vec::new(),
            user_interactions: HashMap::new(),
            learned_patterns: HashMap::new(),
        }
    }

    pub fn generate_smart_suggestions(&mut self, context: &AiContextWindow) -> Vec<AiSuggestion> {
        let mut suggestions = Vec::new();

        // Generate context-aware suggestions
        suggestions.extend(self.suggest_search_improvements(context));
        suggestions.extend(self.suggest_paper_connections(context));
        suggestions.extend(self.suggest_research_directions(context));
        suggestions.extend(self.suggest_code_implementations(context));
        suggestions.extend(self.suggest_collaboration_opportunities(context));

        // Sort by confidence and relevance
        suggestions.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        
        // Learn from suggestions for future improvements
        self.learn_from_context(context);

        suggestions
    }

    pub fn rate_suggestion(&mut self, suggestion_id: &str, rating: f32) {
        self.user_interactions.insert(suggestion_id.to_string(), rating);
        self.update_learned_patterns(suggestion_id, rating);
    }

    pub fn get_personalized_suggestions(&self, user_prefs: &AiUserPreferences) -> Vec<AiSuggestion> {
        let mut suggestions = Vec::new();

        // Generate suggestions based on user preferences
        for area in &user_prefs.research_areas {
            suggestions.push(AiSuggestion {
                id: Uuid::new_v4().to_string(),
                suggestion_type: SuggestionType::SearchQuery,
                title: format!("Explore latest in {}", area),
                description: format!("Discover recent papers in {}", area),
                confidence: 0.8,
                context: area.clone(),
                created_at: Utc::now(),
                paper_ids: Vec::new(),
            });
        }

        suggestions
    }

    // Smart search query suggestions
    fn suggest_search_improvements(&self, context: &AiContextWindow) -> Vec<AiSuggestion> {
        let mut suggestions = Vec::new();

        if let Some(current_search) = &context.current_search {
            // Suggest query expansions
            let expansions = self.generate_query_expansions(current_search);
            
            for expansion in expansions {
                suggestions.push(AiSuggestion {
                    id: Uuid::new_v4().to_string(),
                    suggestion_type: SuggestionType::SearchQuery,
                    title: format!("Try: \"{}\"", expansion),
                    description: "Enhanced search query for better results".to_string(),
                    confidence: self.calculate_query_confidence(&expansion),
                    context: current_search.clone(),
                    created_at: Utc::now(),
                    paper_ids: Vec::new(),
                });
            }

            // Suggest alternative formulations
            let alternatives = self.generate_query_alternatives(current_search);
            
            for alt in alternatives {
                suggestions.push(AiSuggestion {
                    id: Uuid::new_v4().to_string(),
                    suggestion_type: SuggestionType::SearchQuery,
                    title: format!("Alternative: \"{}\"", alt),
                    description: "Different approach to find relevant papers".to_string(),
                    confidence: 0.6,
                    context: current_search.clone(),
                    created_at: Utc::now(),
                    paper_ids: Vec::new(),
                });
            }
        }

        suggestions
    }

    // Paper connection suggestions
    fn suggest_paper_connections(&self, context: &AiContextWindow) -> Vec<AiSuggestion> {
        let mut suggestions = Vec::new();

        if context.selected_papers.len() >= 2 {
            let connections = self.find_paper_connections(&context.selected_papers);
            
            for connection in connections {
                suggestions.push(AiSuggestion {
                    id: Uuid::new_v4().to_string(),
                    suggestion_type: SuggestionType::RelatedPapers,
                    title: connection.title,
                    description: connection.description,
                    confidence: connection.confidence,
                    context: "paper_analysis".to_string(),
                    created_at: Utc::now(),
                    paper_ids: connection.paper_ids,
                });
            }
        }

        suggestions
    }

    // Research direction suggestions
    fn suggest_research_directions(&self, context: &AiContextWindow) -> Vec<AiSuggestion> {
        let mut suggestions = Vec::new();

        let research_gaps = self.identify_research_gaps(context);
        
        for gap in research_gaps {
            suggestions.push(AiSuggestion {
                id: Uuid::new_v4().to_string(),
                suggestion_type: SuggestionType::ResearchTrend,
                title: format!("Research opportunity: {}", gap.topic),
                description: gap.description,
                confidence: gap.confidence,
                context: "research_analysis".to_string(),
                created_at: Utc::now(),
                paper_ids: Vec::new(),
            });
        }

        suggestions
    }

    // Code implementation suggestions
    fn suggest_code_implementations(&self, context: &AiContextWindow) -> Vec<AiSuggestion> {
        let mut suggestions = Vec::new();

        for paper in &context.selected_papers {
            if self.is_implementable(paper) {
                let implementation_types = self.suggest_implementation_approaches(paper);
                
                for impl_type in implementation_types {
                    suggestions.push(AiSuggestion {
                        id: Uuid::new_v4().to_string(),
                        suggestion_type: SuggestionType::CodeExample,
                        title: format!("Implement {} from {}", impl_type.name, paper.title),
                        description: impl_type.description,
                        confidence: impl_type.confidence,
                        context: paper.id.clone(),
                        created_at: Utc::now(),
                        paper_ids: vec![paper.id.clone()],
                    });
                }
            }
        }

        suggestions
    }

    // Collaboration suggestions
    fn suggest_collaboration_opportunities(&self, context: &AiContextWindow) -> Vec<AiSuggestion> {
        let mut suggestions = Vec::new();

        let authors = self.extract_potential_collaborators(context);
        
        for author in authors {
            suggestions.push(AiSuggestion {
                id: Uuid::new_v4().to_string(),
                suggestion_type: SuggestionType::Collaboration,
                title: format!("Potential collaborator: {}", author.name),
                description: format!("Research overlap in {}", author.expertise.join(", ")),
                confidence: author.relevance_score,
                context: "collaboration".to_string(),
                created_at: Utc::now(),
                paper_ids: author.paper_ids,
            });
        }

        suggestions
    }

    // Helper methods
    fn generate_query_expansions(&self, query: &str) -> Vec<String> {
        let mut expansions = Vec::new();
        
        // Add domain-specific expansions
        let ml_terms = ["machine learning", "deep learning", "neural networks", "AI"];
        let cs_terms = ["computer science", "algorithm", "computational"];
        let math_terms = ["mathematical", "statistics", "optimization"];
        
        for &term in &ml_terms {
            if !query.to_lowercase().contains(term) {
                expansions.push(format!("{} AND {}", query, term));
            }
        }
        
        // Add temporal expansions
        expansions.push(format!("{} AND recent", query));
        expansions.push(format!("{} AND 2023..2024", query));
        expansions.push(format!("{} AND survey", query));
        
        expansions
    }

    fn generate_query_alternatives(&self, query: &str) -> Vec<String> {
        let mut alternatives = Vec::new();
        
        // Synonym replacements
        let synonyms = HashMap::from([
            ("learning", vec!["training", "optimization", "adaptation"]),
            ("network", vec!["architecture", "model", "system"]),
            ("deep", vec!["hierarchical", "multilayer", "complex"]),
        ]);
        
        for (word, syns) in synonyms {
            if query.to_lowercase().contains(word) {
                for syn in syns {
                    alternatives.push(query.to_lowercase().replace(word, syn));
                }
            }
        }
        
        alternatives
    }

    fn find_paper_connections(&self, papers: &[ArxivPaper]) -> Vec<ConnectionSuggestion> {
        let mut connections = Vec::new();
        
        for i in 0..papers.len() {
            for j in i+1..papers.len() {
                let paper1 = &papers[i];
                let paper2 = &papers[j];
                
                let connection_strength = self.calculate_connection_strength(paper1, paper2);
                
                if connection_strength > 0.5 {
                    connections.push(ConnectionSuggestion {
                        title: format!("Connection found between '{}' and '{}'", 
                                     paper1.title.chars().take(30).collect::<String>(),
                                     paper2.title.chars().take(30).collect::<String>()),
                        description: "These papers share similar methodologies and could be combined for deeper insights".to_string(),
                        confidence: connection_strength,
                        paper_ids: vec![paper1.id.clone(), paper2.id.clone()],
                    });
                }
            }
        }
        
        connections
    }

    fn identify_research_gaps(&self, context: &AiContextWindow) -> Vec<ResearchGap> {
        let mut gaps = Vec::new();
        
        // Analyze selected papers for missing elements
        let topics = self.extract_topics_from_papers(&context.selected_papers);
        let methodologies = self.extract_methodologies_from_papers(&context.selected_papers);
        
        // Suggest unexplored combinations
        for topic in &topics {
            for methodology in &methodologies {
                if !self.combination_exists(&context.selected_papers, topic, methodology) {
                    gaps.push(ResearchGap {
                        topic: format!("{} using {}", topic, methodology),
                        description: format!("Opportunity to apply {} to {} problems", methodology, topic),
                        confidence: 0.7,
                    });
                }
            }
        }
        
        gaps
    }

    fn is_implementable(&self, paper: &ArxivPaper) -> bool {
        let implementation_indicators = [
            "algorithm", "method", "implementation", "code", "github",
            "experiment", "evaluation", "benchmark", "dataset"
        ];
        
        implementation_indicators.iter().any(|&indicator| {
            paper.abstract_text.to_lowercase().contains(indicator) ||
            paper.title.to_lowercase().contains(indicator)
        })
    }

    fn suggest_implementation_approaches(&self, paper: &ArxivPaper) -> Vec<ImplementationType> {
        let mut approaches = Vec::new();
        
        // Determine implementation complexity and approaches
        if paper.abstract_text.to_lowercase().contains("neural") {
            approaches.push(ImplementationType {
                name: "Neural Network Implementation".to_string(),
                description: "Implement the neural architecture described in the paper".to_string(),
                confidence: 0.8,
            });
        }
        
        if paper.abstract_text.to_lowercase().contains("algorithm") {
            approaches.push(ImplementationType {
                name: "Algorithm Implementation".to_string(),
                description: "Code the core algorithm from scratch".to_string(),
                confidence: 0.9,
            });
        }
        
        approaches
    }

    fn extract_potential_collaborators(&self, context: &AiContextWindow) -> Vec<CollaboratorSuggestion> {
        let mut collaborators = Vec::new();
        
        // Extract authors from selected papers
        let mut author_expertise: HashMap<String, Vec<String>> = HashMap::new();
        let mut author_papers: HashMap<String, Vec<String>> = HashMap::new();
        
        for paper in &context.selected_papers {
            for author in &paper.authors {
                let topics = self.extract_topics_from_title(&paper.title);
                author_expertise.entry(author.clone()).or_default().extend(topics);
                author_papers.entry(author.clone()).or_default().push(paper.id.clone());
            }
        }
        
        for (author, expertise) in author_expertise {
            if expertise.len() >= 2 { // Author has diverse expertise
                collaborators.push(CollaboratorSuggestion {
                    name: author.clone(),
                    expertise: expertise.into_iter().collect::<HashSet<_>>().into_iter().collect(),
                    relevance_score: 0.8,
                    paper_ids: author_papers.get(&author).cloned().unwrap_or_default(),
                });
            }
        }
        
        collaborators
    }

    fn calculate_query_confidence(&self, query: &str) -> f32 {
        // Simple confidence calculation based on query complexity
        let word_count = query.split_whitespace().count();
        let has_operators = query.contains("AND") || query.contains("OR");
        
        let base_confidence = 0.6;
        let complexity_bonus = (word_count as f32 * 0.1).min(0.3);
        let operator_bonus = if has_operators { 0.1 } else { 0.0 };
        
        (base_confidence + complexity_bonus + operator_bonus).min(1.0)
    }

    fn calculate_connection_strength(&self, paper1: &ArxivPaper, paper2: &ArxivPaper) -> f32 {
        let mut strength = 0.0;
        
        // Check for common authors
        let common_authors = paper1.authors.iter()
            .filter(|author| paper2.authors.contains(author))
            .count();
        strength += common_authors as f32 * 0.3;
        
        // Check for common keywords in titles
        let title1_words: HashSet<&str> = paper1.title.split_whitespace().collect();
        let title2_words: HashSet<&str> = paper2.title.split_whitespace().collect();
        let common_title_words = title1_words.intersection(&title2_words).count();
        strength += common_title_words as f32 * 0.1;
        
        // Check for similar topics in summaries
        let summary_similarity = self.calculate_text_similarity(&paper1.abstract_text, &paper2.abstract_text);
        strength += summary_similarity * 0.6;
        
        strength.min(1.0)
    }

    fn calculate_text_similarity(&self, text1: &str, text2: &str) -> f32 {
        // Simple Jaccard similarity
        let words1: HashSet<&str> = text1.split_whitespace().collect();
        let words2: HashSet<&str> = text2.split_whitespace().collect();
        
        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();
        
        if union == 0 { 0.0 } else { intersection as f32 / union as f32 }
    }

    fn extract_topics_from_papers(&self, papers: &[ArxivPaper]) -> Vec<String> {
        let mut topics = HashSet::new();
        
        for paper in papers {
            topics.extend(self.extract_topics_from_title(&paper.title));
        }
        
        topics.into_iter().collect()
    }

    fn extract_topics_from_title(&self, title: &str) -> Vec<String> {
        let keywords = ["learning", "neural", "deep", "machine", "algorithm", "optimization", "classification", "regression"];
        let mut topics = Vec::new();
        
        for &keyword in &keywords {
            if title.to_lowercase().contains(keyword) {
                topics.push(keyword.to_string());
            }
        }
        
        topics
    }

    fn extract_methodologies_from_papers(&self, papers: &[ArxivPaper]) -> Vec<String> {
        let mut methodologies = HashSet::new();
        
        let method_keywords = ["CNN", "RNN", "LSTM", "GAN", "VAE", "Transformer", "SVM", "Random Forest"];
        
        for paper in papers {
            for &method in &method_keywords {
                if paper.abstract_text.contains(method) || paper.title.contains(method) {
                    methodologies.insert(method.to_string());
                }
            }
        }
        
        methodologies.into_iter().collect()
    }

    fn combination_exists(&self, papers: &[ArxivPaper], topic: &str, methodology: &str) -> bool {
        papers.iter().any(|paper| {
            (paper.title.to_lowercase().contains(&topic.to_lowercase()) ||
             paper.abstract_text.to_lowercase().contains(&topic.to_lowercase())) &&
            (paper.title.contains(methodology) || paper.abstract_text.contains(methodology))
        })
    }

    fn learn_from_context(&mut self, context: &AiContextWindow) {
        // Simple learning mechanism
        for paper in &context.selected_papers {
            let topics = self.extract_topics_from_title(&paper.title);
            for topic in topics {
                *self.learned_patterns.entry(topic).or_insert(0.0) += 0.1;
            }
        }
    }

    fn update_learned_patterns(&mut self, suggestion_id: &str, rating: f32) {
        // Find the suggestion and update related patterns
        if let Some(suggestion) = self.suggestion_history.iter().find(|s| s.id == suggestion_id) {
            let pattern_key = format!("{}:{}", suggestion.suggestion_type, suggestion.context);
            *self.learned_patterns.entry(pattern_key).or_insert(0.0) += rating * 0.1;
        }
    }
}

// Helper structs for internal use
#[derive(Debug, Clone)]
struct ConnectionSuggestion {
    title: String,
    description: String,
    confidence: f32,
    paper_ids: Vec<String>,
}

#[derive(Debug, Clone)]
struct ResearchGap {
    topic: String,
    description: String,
    confidence: f32,
}

#[derive(Debug, Clone)]
struct ImplementationType {
    name: String,
    description: String,
    confidence: f32,
}

#[derive(Debug, Clone)]
struct CollaboratorSuggestion {
    name: String,
    expertise: Vec<String>,
    relevance_score: f32,
    paper_ids: Vec<String>,
}

impl Default for AiSuggestionsEngine {
    fn default() -> Self {
        Self::new()
    }
}

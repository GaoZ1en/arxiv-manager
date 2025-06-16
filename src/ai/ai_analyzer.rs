// AI Research Analyzer
// Advanced paper analysis and research insights

use std::collections::{HashMap, HashSet};

use crate::ai::ai_models::*;
use crate::core::models::ArxivPaper;

pub struct AiResearchAnalyzer {
    analysis_cache: HashMap<String, AiAnalysisResult>,
    topic_keywords: HashMap<String, Vec<String>>,
    methodology_patterns: HashMap<String, Vec<String>>,
    complexity_indicators: Vec<String>,
}

impl AiResearchAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            analysis_cache: HashMap::new(),
            topic_keywords: HashMap::new(),
            methodology_patterns: HashMap::new(),
            complexity_indicators: Vec::new(),
        };
        
        analyzer.initialize_knowledge_base();
        analyzer
    }

    pub fn analyze_paper_comprehensive(&mut self, paper: &ArxivPaper) -> AiAnalysisResult {
        // Check cache first
        if let Some(cached) = self.analysis_cache.get(&paper.id) {
            return cached.clone();
        }

        let analysis = AiAnalysisResult {
            paper_id: paper.id.clone(),
            summary: self.generate_intelligent_summary(paper),
            key_points: self.extract_intelligent_key_points(paper),
            methodology: self.identify_methodology(paper),
            code_availability: self.assess_code_availability(paper),
            dataset_info: self.extract_dataset_information(paper),
            related_topics: self.identify_related_topics(paper),
            complexity_score: self.calculate_complexity_score(paper),
            research_impact: self.estimate_research_impact(paper),
        };

        // Cache the analysis
        self.analysis_cache.insert(paper.id.clone(), analysis.clone());
        analysis
    }

    pub fn analyze_research_trends(&self, papers: &[ArxivPaper]) -> Vec<ResearchTrend> {
        let mut trends = Vec::new();
        let topic_frequency = self.calculate_topic_frequency(papers);
        let temporal_analysis = self.analyze_temporal_trends(papers);
        
        for (topic, frequency) in topic_frequency {
            if frequency >= 2 { // Only consider topics with multiple papers
                let trend = ResearchTrend {
                    topic: topic.clone(),
                    trend_score: self.calculate_trend_score(&topic, &temporal_analysis),
                    paper_count: frequency,
                    recent_papers: self.get_recent_papers_for_topic(&topic, papers),
                    key_researchers: self.identify_key_researchers(&topic, papers),
                    emerging_keywords: self.identify_emerging_keywords(&topic, papers),
                };
                trends.push(trend);
            }
        }

        // Sort by trend score
        trends.sort_by(|a, b| b.trend_score.partial_cmp(&a.trend_score).unwrap());
        trends
    }

    pub fn suggest_research_directions(&self, papers: &[ArxivPaper]) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        // Analyze gaps in current research
        let covered_topics = self.extract_covered_topics(papers);
        let methodologies = self.extract_methodologies(papers);
        
        // Suggest cross-domain applications
        for topic in &covered_topics {
            for methodology in &methodologies {
                if !self.combination_exists(papers, topic, methodology) {
                    suggestions.push(format!(
                        "Apply {} methodology to {} problems",
                        methodology, topic
                    ));
                }
            }
        }

        // Suggest emerging technology applications
        let emerging_techs = vec![
            "Transformer architecture", "Graph Neural Networks", 
            "Federated Learning", "Quantum Computing", "Edge AI"
        ];
        
        for tech in emerging_techs {
            for topic in &covered_topics {
                suggestions.push(format!(
                    "Investigate {} applications in {}",
                    tech, topic
                ));
            }
        }

        suggestions
    }

    // Private helper methods
    fn initialize_knowledge_base(&mut self) {
        // Initialize topic keywords
        self.topic_keywords.insert("machine_learning".to_string(), vec![
            "neural network".to_string(), "deep learning".to_string(), 
            "classification".to_string(), "regression".to_string(),
            "supervised".to_string(), "unsupervised".to_string()
        ]);
        
        self.topic_keywords.insert("computer_vision".to_string(), vec![
            "image".to_string(), "visual".to_string(), "CNN".to_string(),
            "object detection".to_string(), "segmentation".to_string()
        ]);

        self.topic_keywords.insert("nlp".to_string(), vec![
            "natural language".to_string(), "text".to_string(), "language model".to_string(),
            "transformer".to_string(), "BERT".to_string(), "GPT".to_string()
        ]);

        // Initialize methodology patterns
        self.methodology_patterns.insert("deep_learning".to_string(), vec![
            "neural network".to_string(), "gradient descent".to_string(),
            "backpropagation".to_string(), "layer".to_string()
        ]);

        self.methodology_patterns.insert("reinforcement_learning".to_string(), vec![
            "reward".to_string(), "policy".to_string(), "agent".to_string(),
            "environment".to_string(), "Q-learning".to_string()
        ]);

        // Initialize complexity indicators
        self.complexity_indicators = vec![
            "novel".to_string(), "complex".to_string(), "advanced".to_string(),
            "sophisticated".to_string(), "state-of-the-art".to_string(),
            "breakthrough".to_string(), "innovative".to_string()
        ];
    }

    fn generate_intelligent_summary(&self, paper: &ArxivPaper) -> String {
        let word_limit = 200;
        let summary_words: Vec<&str> = paper.abstract_text.split_whitespace().collect();
        
        // Extract key sentences using simple heuristics
        let sentences: Vec<&str> = paper.abstract_text.split('.').collect();
        let mut key_sentences = Vec::new();
        
        for sentence in sentences {
            if self.is_key_sentence(sentence) {
                key_sentences.push(sentence);
            }
        }

        if key_sentences.is_empty() {
            // Fallback to truncated original summary
            let truncated: String = summary_words.iter()
                .take(word_limit)
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            return format!("{}...", truncated);
        }

        // Combine key sentences with word limit
        let mut result = String::new();
        let mut word_count = 0;
        
        for sentence in key_sentences {
            let sentence_words = sentence.split_whitespace().count();
            if word_count + sentence_words <= word_limit {
                result.push_str(sentence);
                result.push('.');
                word_count += sentence_words;
            } else {
                break;
            }
        }

        if result.is_empty() {
            format!("Analysis of {}", paper.title)
        } else {
            result
        }
    }

    fn is_key_sentence(&self, sentence: &str) -> bool {
        let key_indicators = [
            "propose", "introduce", "demonstrate", "show", "achieve",
            "novel", "new", "first", "significant", "improve"
        ];
        
        let sentence_lower = sentence.to_lowercase();
        key_indicators.iter().any(|&indicator| sentence_lower.contains(indicator))
    }

    fn extract_intelligent_key_points(&self, paper: &ArxivPaper) -> Vec<String> {
        let mut key_points = Vec::new();
        
        // Extract methodology
        if let Some(methodology) = self.identify_methodology(paper) {
            key_points.push(format!("Methodology: {}", methodology));
        }

        // Extract performance claims
        if paper.abstract_text.to_lowercase().contains("accuracy") ||
           paper.abstract_text.to_lowercase().contains("performance") {
            key_points.push("Performance improvements reported".to_string());
        }

        // Extract novelty claims
        if paper.abstract_text.to_lowercase().contains("novel") ||
           paper.abstract_text.to_lowercase().contains("new") {
            key_points.push("Novel approach presented".to_string());
        }

        // Extract experimental validation
        if paper.abstract_text.to_lowercase().contains("experiment") ||
           paper.abstract_text.to_lowercase().contains("evaluation") {
            key_points.push("Experimental validation provided".to_string());
        }

        if key_points.is_empty() {
            key_points.push("Research contribution in the field".to_string());
        }

        key_points
    }

    fn identify_methodology(&self, paper: &ArxivPaper) -> Option<String> {
        let text = format!("{} {}", paper.title, paper.abstract_text).to_lowercase();
        
        for (methodology, patterns) in &self.methodology_patterns {
            for pattern in patterns {
                if text.contains(pattern) {
                    return Some(methodology.replace("_", " "));
                }
            }
        }

        // Fallback to common methodology detection
        if text.contains("neural network") || text.contains("deep learning") {
            Some("Deep Learning".to_string())
        } else if text.contains("machine learning") {
            Some("Machine Learning".to_string())
        } else if text.contains("algorithm") {
            Some("Algorithmic Approach".to_string())
        } else {
            None
        }
    }

    fn assess_code_availability(&self, paper: &ArxivPaper) -> bool {
        let indicators = [
            "github", "code", "implementation", "repository", 
            "source code", "available", "open source", "software"
        ];
        
        let text = format!("{} {}", paper.title, paper.abstract_text).to_lowercase();
        indicators.iter().any(|&indicator| text.contains(indicator))
    }

    fn extract_dataset_information(&self, paper: &ArxivPaper) -> Option<String> {
        let text = format!("{} {}", paper.title, paper.abstract_text).to_lowercase();
        
        let dataset_names = [
            "imagenet", "cifar", "mnist", "coco", "pascal voc",
            "squad", "glue", "bert", "openai", "common crawl"
        ];
        
        for &dataset in &dataset_names {
            if text.contains(dataset) {
                return Some(format!("Uses {} dataset", dataset));
            }
        }

        if text.contains("dataset") {
            Some("Custom dataset used".to_string())
        } else {
            None
        }
    }

    fn identify_related_topics(&self, paper: &ArxivPaper) -> Vec<String> {
        let mut topics = Vec::new();
        let text = format!("{} {}", paper.title, paper.abstract_text).to_lowercase();
        
        for (topic, keywords) in &self.topic_keywords {
            for keyword in keywords {
                if text.contains(keyword) {
                    topics.push(topic.replace("_", " "));
                    break;
                }
            }
        }

        if topics.is_empty() {
            topics.push("Computer Science".to_string());
        }

        topics.into_iter().collect::<HashSet<_>>().into_iter().collect()
    }

    fn calculate_complexity_score(&self, paper: &ArxivPaper) -> f32 {
        let text = format!("{} {}", paper.title, paper.abstract_text).to_lowercase();
        let mut score = 0.0_f32;
        
        // Check for complexity indicators
        for indicator in &self.complexity_indicators {
            if text.contains(indicator) {
                score += 0.2;
            }
        }

        // Check for mathematical complexity
        if text.contains("theorem") || text.contains("proof") || text.contains("lemma") {
            score += 0.3;
        }

        // Check for implementation complexity
        if text.contains("architecture") || text.contains("framework") {
            score += 0.2;
        }

        score.min(1.0_f32)
    }

    fn estimate_research_impact(&self, paper: &ArxivPaper) -> f32 {
        let mut impact = 0.5_f32; // Base impact score
        
        // Check for high-impact indicators
        let text = format!("{} {}", paper.title, paper.abstract_text).to_lowercase();
        
        if text.contains("breakthrough") || text.contains("state-of-the-art") {
            impact += 0.3;
        }
        
        if text.contains("significant") || text.contains("substantial") {
            impact += 0.2;
        }

        // Consider publication recency (more recent = potentially higher impact)
        // For simplicity, assume recent papers (we can enhance this later)
        impact += 0.1;

        impact.min(1.0)
    }

    // Additional helper methods...
    fn calculate_topic_frequency(&self, papers: &[ArxivPaper]) -> HashMap<String, u32> {
        let mut frequency = HashMap::new();
        
        for paper in papers {
            let topics = self.identify_related_topics(paper);
            for topic in topics {
                *frequency.entry(topic).or_insert(0) += 1;
            }
        }
        
        frequency
    }

    fn analyze_temporal_trends(&self, papers: &[ArxivPaper]) -> HashMap<String, Vec<String>> {
        let mut temporal_data = HashMap::new();
        
        for paper in papers {
            let topics = self.identify_related_topics(paper);
            for topic in topics {
                temporal_data.entry(topic).or_insert_with(Vec::new).push(paper.published.clone());
            }
        }
        
        temporal_data
    }

    fn calculate_trend_score(&self, _topic: &str, temporal_data: &HashMap<String, Vec<String>>) -> f32 {
        // Simple trend calculation based on publication frequency
        if let Some(dates) = temporal_data.get(_topic) {
            // For simplicity, assume higher frequency means higher trend
            (dates.len() as f32 / 10.0).min(1.0)
        } else {
            0.0
        }
    }

    fn get_recent_papers_for_topic(&self, topic: &str, papers: &[ArxivPaper]) -> Vec<String> {
        papers.iter()
            .filter(|paper| self.identify_related_topics(paper).contains(&topic.to_string()))
            .take(5)
            .map(|paper| paper.id.clone())
            .collect()
    }

    fn identify_key_researchers(&self, topic: &str, papers: &[ArxivPaper]) -> Vec<String> {
        let mut author_count = HashMap::new();
        
        for paper in papers {
            if self.identify_related_topics(paper).contains(&topic.to_string()) {
                for author in &paper.authors {
                    *author_count.entry(author.clone()).or_insert(0) += 1;
                }
            }
        }

        let mut researchers: Vec<_> = author_count.into_iter().collect();
        researchers.sort_by(|a, b| b.1.cmp(&a.1));
        researchers.into_iter().take(3).map(|(author, _)| author).collect()
    }

    fn identify_emerging_keywords(&self, _topic: &str, _papers: &[ArxivPaper]) -> Vec<String> {
        // Simplified implementation
        vec!["emerging".to_string(), "novel".to_string(), "advanced".to_string()]
    }

    fn extract_covered_topics(&self, papers: &[ArxivPaper]) -> Vec<String> {
        let mut topics = HashSet::new();
        for paper in papers {
            topics.extend(self.identify_related_topics(paper));
        }
        topics.into_iter().collect()
    }

    fn extract_methodologies(&self, papers: &[ArxivPaper]) -> Vec<String> {
        let mut methodologies = HashSet::new();
        for paper in papers {
            if let Some(methodology) = self.identify_methodology(paper) {
                methodologies.insert(methodology);
            }
        }
        methodologies.into_iter().collect()
    }

    fn combination_exists(&self, papers: &[ArxivPaper], topic: &str, methodology: &str) -> bool {
        papers.iter().any(|paper| {
            let paper_topics = self.identify_related_topics(paper);
            let paper_methodology = self.identify_methodology(paper);
            
            paper_topics.contains(&topic.to_string()) &&
            paper_methodology.as_deref() == Some(methodology)
        })
    }
}

impl Default for AiResearchAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

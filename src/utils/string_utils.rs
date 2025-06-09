use super::{UtilError, UtilResult};
use regex::Regex;
use std::collections::HashMap;

/// String processing utility functions
pub struct StringUtils;

impl StringUtils {
    /// Clean and normalize text for indexing
    pub fn normalize_text(text: &str) -> String {
        text.chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c.is_whitespace() {
                    c.to_lowercase().collect::<String>()
                } else {
                    " ".to_string()
                }
            })
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }
    
    /// Extract arXiv ID from various formats
    pub fn extract_arxiv_id(text: &str) -> Option<String> {
        // arXiv ID patterns
        let patterns = vec![
            r"(?i)arxiv:([0-9]{4}\.[0-9]{4,5}(?:v[0-9]+)?)",
            r"(?i)([0-9]{4}\.[0-9]{4,5}(?:v[0-9]+)?)",
            r"(?i)arxiv\.org/abs/([0-9]{4}\.[0-9]{4,5}(?:v[0-9]+)?)",
            r"(?i)arxiv\.org/pdf/([0-9]{4}\.[0-9]{4,5}(?:v[0-9]+)?)",
        ];
        
        for pattern in patterns {
            if let Ok(re) = Regex::new(pattern) {
                if let Some(caps) = re.captures(text) {
                    if let Some(id) = caps.get(1) {
                        return Some(id.as_str().to_string());
                    }
                }
            }
        }
        
        None
    }
    
    /// Extract DOI from text
    pub fn extract_doi(text: &str) -> Option<String> {
        let pattern = r"(?i)(?:doi:?\s*)?10\.\d{4,}/[^\s\]]+";
        
        if let Ok(re) = Regex::new(pattern) {
            if let Some(m) = re.find(text) {
                let doi = m.as_str();
                // Clean up the DOI
                let cleaned = doi
                    .trim_start_matches("doi:")
                    .trim_start_matches("DOI:")
                    .trim();
                return Some(cleaned.to_string());
            }
        }
        
        None
    }
    
    /// Parse author names from various formats
    pub fn parse_authors(authors_str: &str) -> Vec<String> {
        // Handle different separators and formats
        let separators = vec![",", ";", " and ", " & ", "\n"];
        let mut authors = vec![authors_str.to_string()];
        
        for separator in separators {
            let mut new_authors = Vec::new();
            for author in authors {
                new_authors.extend(
                    author.split(separator)
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                );
            }
            authors = new_authors;
        }
        
        // Clean up author names
        authors.into_iter()
            .map(|author| Self::clean_author_name(&author))
            .filter(|author| !author.is_empty())
            .collect()
    }
    
    /// Clean individual author name
    fn clean_author_name(name: &str) -> String {
        // Remove common prefixes and suffixes
        let cleaned = name
            .trim()
            .trim_start_matches("and ")
            .trim_start_matches("And ")
            .trim_end_matches(" et al.")
            .trim_end_matches(" et al")
            .trim();
        
        // Remove extra whitespace
        cleaned.split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }
    
    /// Extract categories from arXiv string
    pub fn parse_categories(categories_str: &str) -> Vec<String> {
        categories_str
            .split_whitespace()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
    
    /// Truncate text to specified length with ellipsis
    pub fn truncate_text(text: &str, max_length: usize) -> String {
        if text.len() <= max_length {
            text.to_string()
        } else {
            let mut truncated = text.chars().take(max_length.saturating_sub(3)).collect::<String>();
            truncated.push_str("...");
            truncated
        }
    }
    
    /// Clean text for display (remove extra whitespace, control characters)
    pub fn clean_display_text(text: &str) -> String {
        text.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
            .chars()
            .filter(|c| !c.is_control() || c.is_whitespace())
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }
    
    /// Calculate similarity between two strings using Levenshtein distance
    pub fn similarity_score(a: &str, b: &str) -> f64 {
        let a_normalized = Self::normalize_text(a);
        let b_normalized = Self::normalize_text(b);
        
        let distance = Self::levenshtein_distance(&a_normalized, &b_normalized);
        let max_len = a_normalized.len().max(b_normalized.len());
        
        if max_len == 0 {
            1.0
        } else {
            1.0 - (distance as f64 / max_len as f64)
        }
    }
    
    /// Calculate Levenshtein distance between two strings
    fn levenshtein_distance(a: &str, b: &str) -> usize {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        let a_len = a_chars.len();
        let b_len = b_chars.len();
        
        if a_len == 0 {
            return b_len;
        }
        if b_len == 0 {
            return a_len;
        }
        
        let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];
        
        // Initialize first row and column
        for i in 0..=a_len {
            matrix[i][0] = i;
        }
        for j in 0..=b_len {
            matrix[0][j] = j;
        }
        
        // Fill the matrix
        for i in 1..=a_len {
            for j in 1..=b_len {
                let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
                matrix[i][j] = (matrix[i - 1][j] + 1)
                    .min(matrix[i][j - 1] + 1)
                    .min(matrix[i - 1][j - 1] + cost);
            }
        }
        
        matrix[a_len][b_len]
    }
    
    /// Extract keywords from text using simple frequency analysis
    pub fn extract_keywords(text: &str, min_length: usize, max_count: usize) -> Vec<String> {
        let normalized = Self::normalize_text(text);
        let mut word_counts: HashMap<String, usize> = HashMap::new();
        
        for word in normalized.split_whitespace() {
            if word.len() >= min_length && !Self::is_stop_word(word) {
                *word_counts.entry(word.to_string()).or_insert(0) += 1;
            }
        }
        
        let mut keywords: Vec<(String, usize)> = word_counts.into_iter().collect();
        keywords.sort_by(|a, b| b.1.cmp(&a.1));
        
        keywords.into_iter()
            .take(max_count)
            .map(|(word, _)| word)
            .collect()
    }
    
    /// Check if a word is a common stop word
    fn is_stop_word(word: &str) -> bool {
        const STOP_WORDS: &[&str] = &[
            "the", "be", "to", "of", "and", "a", "in", "that", "have",
            "i", "it", "for", "not", "on", "with", "he", "as", "you",
            "do", "at", "this", "but", "his", "by", "from", "they",
            "we", "say", "her", "she", "or", "an", "will", "my",
            "one", "all", "would", "there", "their", "what", "so",
            "up", "out", "if", "about", "who", "get", "which", "go", "me",
            "when", "make", "can", "like", "time", "no", "just", "him",
            "know", "take", "people", "into", "year", "your", "good",
            "some", "could", "them", "see", "other", "than", "then",
            "now", "look", "only", "come", "its", "over", "think", "also",
            "back", "after", "use", "two", "how", "our", "work", "first",
            "well", "way", "even", "new", "want", "because", "any", "these",
            "give", "day", "most", "us", "is", "was", "are", "been", "has",
            "had", "were", "said", "each", "which", "their", "time", "will"
        ];
        
        STOP_WORDS.contains(&word.to_lowercase().as_str())
    }
    
    /// Generate a slug from text (for URLs or filenames)
    pub fn slugify(text: &str) -> String {
        text.to_lowercase()
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() {
                    c
                } else if c.is_whitespace() || c == '-' || c == '_' {
                    '-'
                } else {
                    ' '
                }
            })
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("-")
            .trim_matches('-')
            .to_string()
    }
    
    /// Highlight search terms in text
    pub fn highlight_search_terms(text: &str, terms: &[String]) -> String {
        let mut result = text.to_string();
        
        for term in terms {
            if !term.is_empty() {
                let pattern = format!("(?i){}", regex::escape(term));
                if let Ok(re) = Regex::new(&pattern) {
                    result = re.replace_all(&result, |caps: &regex::Captures| {
                        format!("**{}**", &caps[0])
                    }).to_string();
                }
            }
        }
        
        result
    }
    
    /// Count words in text
    pub fn word_count(text: &str) -> usize {
        text.split_whitespace().count()
    }
    
    /// Estimate reading time in minutes
    pub fn estimate_reading_time(text: &str, words_per_minute: usize) -> usize {
        let word_count = Self::word_count(text);
        (word_count + words_per_minute - 1) / words_per_minute // Round up
    }
    
    /// Check if text looks like a valid email
    pub fn is_valid_email(email: &str) -> bool {
        let pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";
        if let Ok(re) = Regex::new(pattern) {
            re.is_match(email)
        } else {
            false
        }
    }
    
    /// Check if text looks like a valid URL
    pub fn is_valid_url(url: &str) -> bool {
        let pattern = r"^https?://[^\s/$.?#].[^\s]*$";
        if let Ok(re) = Regex::new(pattern) {
            re.is_match(url)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_text() {
        let text = "Hello, World! This is a TEST.";
        let normalized = StringUtils::normalize_text(text);
        assert_eq!(normalized, "hello world this is a test");
    }

    #[test]
    fn test_extract_arxiv_id() {
        assert_eq!(StringUtils::extract_arxiv_id("arXiv:2301.1234"), Some("2301.1234".to_string()));
        assert_eq!(StringUtils::extract_arxiv_id("https://arxiv.org/abs/2301.1234"), Some("2301.1234".to_string()));
        assert_eq!(StringUtils::extract_arxiv_id("2301.1234v2"), Some("2301.1234v2".to_string()));
        assert_eq!(StringUtils::extract_arxiv_id("not an arxiv id"), None);
    }

    #[test]
    fn test_extract_doi() {
        assert_eq!(StringUtils::extract_doi("DOI: 10.1234/example"), Some("10.1234/example".to_string()));
        assert_eq!(StringUtils::extract_doi("doi:10.5678/test"), Some("10.5678/test".to_string()));
        assert_eq!(StringUtils::extract_doi("no doi here"), None);
    }

    #[test]
    fn test_parse_authors() {
        let authors = StringUtils::parse_authors("John Doe, Jane Smith; Bob Johnson and Alice Brown");
        assert_eq!(authors.len(), 4);
        assert!(authors.contains(&"John Doe".to_string()));
        assert!(authors.contains(&"Jane Smith".to_string()));
        assert!(authors.contains(&"Bob Johnson".to_string()));
        assert!(authors.contains(&"Alice Brown".to_string()));
    }

    #[test]
    fn test_truncate_text() {
        assert_eq!(StringUtils::truncate_text("short", 10), "short");
        assert_eq!(StringUtils::truncate_text("this is a long text", 10), "this is...");
    }

    #[test]
    fn test_similarity_score() {
        assert!((StringUtils::similarity_score("hello", "hello") - 1.0).abs() < 0.01);
        assert!(StringUtils::similarity_score("hello", "world") < 0.5);
        assert!(StringUtils::similarity_score("hello", "helo") > 0.8);
    }

    #[test]
    fn test_slugify() {
        assert_eq!(StringUtils::slugify("Hello World!"), "hello-world");
        assert_eq!(StringUtils::slugify("Test_File Name.pdf"), "test-file-name-pdf");
    }

    #[test]
    fn test_highlight_search_terms() {
        let text = "This is a test document";
        let terms = vec!["test".to_string()];
        let highlighted = StringUtils::highlight_search_terms(text, &terms);
        assert!(highlighted.contains("**test**"));
    }

    #[test]
    fn test_word_count() {
        assert_eq!(StringUtils::word_count("hello world"), 2);
        assert_eq!(StringUtils::word_count("  one   two   three  "), 3);
        assert_eq!(StringUtils::word_count(""), 0);
    }

    #[test]
    fn test_estimate_reading_time() {
        // Assuming 200 words per minute
        assert_eq!(StringUtils::estimate_reading_time("word ".repeat(200).trim(), 200), 1);
        assert_eq!(StringUtils::estimate_reading_time("word ".repeat(300).trim(), 200), 2);
    }

    #[test]
    fn test_email_validation() {
        assert!(StringUtils::is_valid_email("test@example.com"));
        assert!(StringUtils::is_valid_email("user.name+tag@domain.co.uk"));
        assert!(!StringUtils::is_valid_email("invalid.email"));
        assert!(!StringUtils::is_valid_email("@domain.com"));
    }

    #[test]
    fn test_url_validation() {
        assert!(StringUtils::is_valid_url("https://example.com"));
        assert!(StringUtils::is_valid_url("http://test.org/path"));
        assert!(!StringUtils::is_valid_url("not-a-url"));
        assert!(!StringUtils::is_valid_url("ftp://example.com"));
    }
}

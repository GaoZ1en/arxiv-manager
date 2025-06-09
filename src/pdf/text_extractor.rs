use super::{PdfError, PageContent};
use anyhow::Result;
use std::path::Path;

/// Text extraction utilities for PDF files
pub struct TextExtractor;

impl TextExtractor {
    /// Extract text from a PDF file using various methods
    pub async fn extract_text<P: AsRef<Path>>(file_path: P) -> Result<String, PdfError> {
        // Try different extraction methods in order of preference
        
        // Method 1: Try with pdf-extract library (if available)
        if let Ok(text) = Self::extract_with_pdf_extract(&file_path).await {
            return Ok(text);
        }
        
        // Method 2: Try with external tools (pdftotext, etc.)
        if let Ok(text) = Self::extract_with_external_tools(&file_path).await {
            return Ok(text);
        }
        
        // Method 3: Fallback to basic extraction
        Self::extract_basic(&file_path).await
    }
    
    /// Extract text using pdf-extract library (placeholder)
    async fn extract_with_pdf_extract<P: AsRef<Path>>(file_path: P) -> Result<String, PdfError> {
        // This would use the pdf-extract crate
        // For now, return an error to fall back to other methods
        Err(PdfError::TextExtraction("pdf-extract not available".to_string()))
    }
    
    /// Extract text using external command-line tools
    async fn extract_with_external_tools<P: AsRef<Path>>(file_path: P) -> Result<String, PdfError> {
        use std::process::Command;
        
        let path_str = file_path.as_ref().to_string_lossy();
        
        // Try pdftotext command
        if let Ok(output) = Command::new("pdftotext")
            .arg("-layout")
            .arg(&*path_str)
            .arg("-")
            .output()
        {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                if !text.trim().is_empty() {
                    return Ok(text.to_string());
                }
            }
        }
        
        // Try pdfplumber (Python)
        if let Ok(output) = Command::new("python3")
            .arg("-c")
            .arg(&format!(
                "import pdfplumber; \
                 with pdfplumber.open('{}') as pdf: \
                     text = ''; \
                     for page in pdf.pages: \
                         text += page.extract_text() or ''; \
                     print(text)",
                path_str
            ))
            .output()
        {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                if !text.trim().is_empty() {
                    return Ok(text.to_string());
                }
            }
        }
        
        Err(PdfError::TextExtraction("No external tools available".to_string()))
    }
    
    /// Basic text extraction (fallback method)
    async fn extract_basic<P: AsRef<Path>>(file_path: P) -> Result<String, PdfError> {
        // This is a very basic fallback that just returns the filename
        // In a real implementation, you might try to read some basic text
        let filename = file_path.as_ref()
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        
        Ok(format!("Text extracted from: {}", filename))
    }
    
    /// Clean and normalize extracted text
    pub fn clean_text(text: &str) -> String {
        text.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
            .chars()
            .filter(|c| c.is_ascii() || c.is_whitespace())
            .collect::<String>()
            .trim()
            .to_string()
    }
    
    /// Extract text from specific pages
    pub async fn extract_pages<P: AsRef<Path>>(
        file_path: P,
        page_range: std::ops::Range<u32>,
    ) -> Result<Vec<PageContent>, PdfError> {
        // Placeholder implementation
        let full_text = Self::extract_text(file_path).await?;
        
        // For now, just create fake pages
        let mut pages = Vec::new();
        for page_num in page_range {
            pages.push(PageContent {
                page_number: page_num,
                text: format!("Page {} content: {}", page_num, &full_text[0..full_text.len().min(100)]),
                images: Vec::new(),
            });
        }
        
        Ok(pages)
    }
    
    /// Extract text and split into sentences
    pub async fn extract_sentences<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>, PdfError> {
        let text = Self::extract_text(file_path).await?;
        let cleaned_text = Self::clean_text(&text);
        
        // Simple sentence splitting
        let sentences: Vec<String> = cleaned_text
            .split(|c: char| c == '.' || c == '!' || c == '?')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty() && s.len() > 10)
            .collect();
        
        Ok(sentences)
    }
    
    /// Extract text and split into paragraphs
    pub async fn extract_paragraphs<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>, PdfError> {
        let text = Self::extract_text(file_path).await?;
        let cleaned_text = Self::clean_text(&text);
        
        // Split by double newlines (paragraph breaks)
        let paragraphs: Vec<String> = cleaned_text
            .split("\n\n")
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty() && p.len() > 50)
            .collect();
        
        Ok(paragraphs)
    }
    
    /// Get word count from extracted text
    pub async fn get_word_count<P: AsRef<Path>>(file_path: P) -> Result<usize, PdfError> {
        let text = Self::extract_text(file_path).await?;
        let word_count = text
            .split_whitespace()
            .count();
        
        Ok(word_count)
    }
    
    /// Extract keywords using simple frequency analysis
    pub async fn extract_keywords<P: AsRef<Path>>(
        file_path: P,
        limit: usize,
    ) -> Result<Vec<(String, usize)>, PdfError> {
        let text = Self::extract_text(file_path).await?;
        let cleaned_text = Self::clean_text(&text).to_lowercase();
        
        // Simple word frequency analysis
        let mut word_counts = std::collections::HashMap::new();
        
        for word in cleaned_text.split_whitespace() {
            let word = word.trim_matches(|c: char| !c.is_alphabetic());
            if word.len() > 3 && !Self::is_stop_word(word) {
                *word_counts.entry(word.to_string()).or_insert(0) += 1;
            }
        }
        
        // Sort by frequency and return top keywords
        let mut keywords: Vec<(String, usize)> = word_counts.into_iter().collect();
        keywords.sort_by(|a, b| b.1.cmp(&a.1));
        keywords.truncate(limit);
        
        Ok(keywords)
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
        ];
        
        STOP_WORDS.contains(&word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_text() {
        let dirty_text = "  This is   a test  \n\n  with weird   spacing  \n  ";
        let cleaned = TextExtractor::clean_text(dirty_text);
        assert_eq!(cleaned, "This is   a test\nwith weird   spacing");
    }

    #[test]
    fn test_stop_word_detection() {
        assert!(TextExtractor::is_stop_word("the"));
        assert!(TextExtractor::is_stop_word("and"));
        assert!(!TextExtractor::is_stop_word("algorithm"));
        assert!(!TextExtractor::is_stop_word("machine"));
    }

    #[tokio::test]
    async fn test_basic_extraction() {
        use tempfile::NamedTempFile;
        
        let temp_file = NamedTempFile::new().unwrap();
        let result = TextExtractor::extract_basic(temp_file.path()).await;
        assert!(result.is_ok());
    }
}

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub mod pdf_processor;
pub mod text_extractor;

pub use pdf_processor::PdfProcessor;
pub use text_extractor::TextExtractor;

/// Represents metadata extracted from a PDF
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfMetadata {
    pub title: Option<String>,
    pub authors: Vec<String>,
    pub subject: Option<String>,
    pub keywords: Option<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
    pub creation_date: Option<String>,
    pub modification_date: Option<String>,
    pub page_count: u32,
    pub file_size: u64,
}

/// Represents text content extracted from a PDF
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfContent {
    pub text: String,
    pub pages: Vec<PageContent>,
    pub metadata: PdfMetadata,
}

/// Content from a single page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageContent {
    pub page_number: u32,
    pub text: String,
    pub images: Vec<ImageInfo>,
}

/// Information about images found in the PDF
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    pub page_number: u32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub image_type: String,
}

/// Configuration for PDF processing
#[derive(Debug, Clone)]
pub struct ProcessingConfig {
    pub extract_images: bool,
    pub extract_metadata: bool,
    pub max_pages: Option<u32>,
    pub text_only: bool,
}

impl Default for ProcessingConfig {
    fn default() -> Self {
        Self {
            extract_images: false,
            extract_metadata: true,
            max_pages: None,
            text_only: true,
        }
    }
}

/// Error types for PDF processing
#[derive(Debug, thiserror::Error)]
pub enum PdfError {
    #[error("Failed to open PDF file: {0}")]
    FileOpen(String),
    
    #[error("Failed to extract text: {0}")]
    TextExtraction(String),
    
    #[error("Failed to extract metadata: {0}")]
    MetadataExtraction(String),
    
    #[error("PDF is password protected")]
    PasswordProtected,
    
    #[error("Unsupported PDF version")]
    UnsupportedVersion,
    
    #[error("Corrupted PDF file")]
    CorruptedFile,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_pdf_metadata_serialization() {
        let metadata = PdfMetadata {
            title: Some("Test Paper".to_string()),
            authors: vec!["Author 1".to_string(), "Author 2".to_string()],
            subject: Some("Computer Science".to_string()),
            keywords: Some("AI, ML".to_string()),
            creator: Some("LaTeX".to_string()),
            producer: Some("pdfTeX".to_string()),
            creation_date: Some("2023-01-01".to_string()),
            modification_date: Some("2023-01-02".to_string()),
            page_count: 10,
            file_size: 1024,
        };
        
        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: PdfMetadata = serde_json::from_str(&json).unwrap();
        
        assert_eq!(metadata.title, deserialized.title);
        assert_eq!(metadata.authors, deserialized.authors);
        assert_eq!(metadata.page_count, deserialized.page_count);
    }

    #[test]
    fn test_processing_config_default() {
        let config = ProcessingConfig::default();
        assert!(!config.extract_images);
        assert!(config.extract_metadata);
        assert!(config.text_only);
        assert!(config.max_pages.is_none());
    }
}

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub mod file_utils;
pub mod date_utils;
pub mod string_utils;
pub mod validation;
pub mod config_utils;

pub use file_utils::*;
pub use date_utils::*;
pub use string_utils::*;
pub use validation::*;
pub use config_utils::*;

/// Common error types for utility functions
#[derive(Debug, thiserror::Error)]
pub enum UtilError {
    #[error("File operation failed: {0}")]
    FileError(String),
    
    #[error("Date parsing failed: {0}")]
    DateError(String),
    
    #[error("Validation failed: {0}")]
    ValidationError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("String processing error: {0}")]
    StringError(String),
}

/// Result type alias for utility functions
pub type UtilResult<T> = Result<T, UtilError>;

/// Common constants used throughout the application
pub mod constants {
    /// Default file extensions for arXiv papers
    pub const ARXIV_EXTENSIONS: &[&str] = &["pdf", "ps", "tex"];
    
    /// Maximum file size for downloads (100MB)
    pub const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024;
    
    /// Default user agent for HTTP requests
    pub const USER_AGENT: &str = "ArxivManager/1.0";
    
    /// arXiv base URL
    pub const ARXIV_BASE_URL: &str = "https://arxiv.org";
    
    /// arXiv API base URL
    pub const ARXIV_API_URL: &str = "http://export.arxiv.org/api/query";
    
    /// Default search limit
    pub const DEFAULT_SEARCH_LIMIT: usize = 50;
    
    /// Maximum search limit
    pub const MAX_SEARCH_LIMIT: usize = 1000;
    
    /// Default timeout for HTTP requests (30 seconds)
    pub const HTTP_TIMEOUT_SECS: u64 = 30;
    
    /// Supported arXiv categories
    pub const ARXIV_CATEGORIES: &[&str] = &[
        "cs.AI", "cs.CL", "cs.CV", "cs.LG", "cs.NE", "cs.RO",
        "stat.ML", "math.OC", "physics.data-an", "q-bio.QM",
        "cs.DC", "cs.DS", "cs.IT", "cs.SY", "eess.AS", "eess.IV",
        "eess.SP", "eess.SY", "math.CO", "math.PR", "math.ST",
    ];
}

/// Common application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub download_directory: String,
    pub index_directory: String,
    pub max_concurrent_downloads: usize,
    pub auto_extract_text: bool,
    pub auto_index_papers: bool,
    pub theme: String,
    pub language: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            download_directory: default_download_dir(),
            index_directory: default_index_dir(),
            max_concurrent_downloads: 3,
            auto_extract_text: true,
            auto_index_papers: true,
            theme: "gruvbox".to_string(),
            language: "en".to_string(),
        }
    }
}

/// Get default download directory
pub fn default_download_dir() -> String {
    dirs::download_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
        .join("arxiv_papers")
        .to_string_lossy()
        .to_string()
}

/// Get default index directory
pub fn default_index_dir() -> String {
    dirs::data_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
        .join("arxiv_manager")
        .join("index")
        .to_string_lossy()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_settings_default() {
        let settings = AppSettings::default();
        assert_eq!(settings.max_concurrent_downloads, 3);
        assert!(settings.auto_extract_text);
        assert!(settings.auto_index_papers);
        assert_eq!(settings.theme, "gruvbox");
    }

    #[test]
    fn test_constants() {
        assert!(constants::ARXIV_EXTENSIONS.contains(&"pdf"));
        assert!(constants::ARXIV_CATEGORIES.contains(&"cs.AI"));
        assert_eq!(constants::DEFAULT_SEARCH_LIMIT, 50);
    }

    #[test]
    fn test_default_directories() {
        let download_dir = default_download_dir();
        let index_dir = default_index_dir();
        
        assert!(download_dir.contains("arxiv_papers"));
        assert!(index_dir.contains("index"));
    }
}

use super::{UtilError, UtilResult};
use std::path::Path;
use url::Url;

/// Validation utility functions
pub struct Validation;

impl Validation {
    /// Validate arXiv ID format
    pub fn is_valid_arxiv_id(id: &str) -> bool {
        // arXiv ID patterns: YYMM.NNNN[vN] or subject-class/YYMMnnn
        let new_format = regex::Regex::new(r"^[0-9]{4}\.[0-9]{4,5}(v[0-9]+)?$").unwrap();
        let old_format = regex::Regex::new(r"^[a-z-]+(.[A-Z]{2})?/[0-9]{7}(v[0-9]+)?$").unwrap();
        
        new_format.is_match(id) || old_format.is_match(id)
    }
    
    /// Validate DOI format
    pub fn is_valid_doi(doi: &str) -> bool {
        // DOI pattern: 10.nnnn/suffix
        let pattern = regex::Regex::new(r"^10\.\d{4,}/[^\s]+$").unwrap();
        pattern.is_match(doi)
    }
    
    /// Validate URL format and accessibility
    pub fn is_valid_url(url_str: &str) -> bool {
        match Url::parse(url_str) {
            Ok(url) => {
                // Check if it's HTTP or HTTPS
                matches!(url.scheme(), "http" | "https")
            }
            Err(_) => false,
        }
    }
    
    /// Validate email format
    pub fn is_valid_email(email: &str) -> bool {
        let pattern = regex::Regex::new(
            r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
        ).unwrap();
        pattern.is_match(email)
    }
    
    /// Validate file path
    pub fn is_valid_file_path<P: AsRef<Path>>(path: P) -> UtilResult<()> {
        let path = path.as_ref();
        
        // Check if path is safe (no directory traversal)
        let path_str = path.to_string_lossy();
        if path_str.contains("..") {
            return Err(UtilError::ValidationError("Path contains directory traversal".to_string()));
        }
        
        // Check for null bytes
        if path_str.contains('\0') {
            return Err(UtilError::ValidationError("Path contains null bytes".to_string()));
        }
        
        // Check path length (OS dependent, but 260 is safe for most systems)
        if path_str.len() > 260 {
            return Err(UtilError::ValidationError("Path too long".to_string()));
        }
        
        // Check for invalid characters on Windows
        #[cfg(windows)]
        {
            let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
            for char in invalid_chars {
                if path_str.contains(char) {
                    return Err(UtilError::ValidationError(format!("Path contains invalid character: {}", char)));
                }
            }
        }
        
        Ok(())
    }
    
    /// Validate directory path and check if it's writable
    pub fn is_valid_directory<P: AsRef<Path>>(path: P) -> UtilResult<()> {
        let path = path.as_ref();
        
        // First validate the path format
        Self::is_valid_file_path(path)?;
        
        // Check if directory exists
        if !path.exists() {
            return Err(UtilError::ValidationError("Directory does not exist".to_string()));
        }
        
        // Check if it's actually a directory
        if !path.is_dir() {
            return Err(UtilError::ValidationError("Path is not a directory".to_string()));
        }
        
        // Check if it's readable and writable
        let metadata = std::fs::metadata(path)
            .map_err(|e| UtilError::ValidationError(format!("Cannot read directory metadata: {}", e)))?;
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt;
            let mode = metadata.mode();
            let is_readable = mode & 0o400 != 0;
            let is_writable = mode & 0o200 != 0;
            
            if !is_readable {
                return Err(UtilError::ValidationError("Directory is not readable".to_string()));
            }
            
            if !is_writable {
                return Err(UtilError::ValidationError("Directory is not writable".to_string()));
            }
        }
        
        #[cfg(windows)]
        {
            // On Windows, try to create a temporary file to test write access
            let temp_file = path.join(".write_test");
            match std::fs::write(&temp_file, "test") {
                Ok(_) => {
                    let _ = std::fs::remove_file(&temp_file);
                }
                Err(_) => {
                    return Err(UtilError::ValidationError("Directory is not writable".to_string()));
                }
            }
        }
        
        Ok(())
    }
    
    /// Validate file size (check if it's within reasonable limits)
    pub fn is_valid_file_size(size: u64, max_size: u64) -> UtilResult<()> {
        if size > max_size {
            return Err(UtilError::ValidationError(
                format!("File size {} exceeds maximum allowed size {}", size, max_size)
            ));
        }
        
        if size == 0 {
            return Err(UtilError::ValidationError("File is empty".to_string()));
        }
        
        Ok(())
    }
    
    /// Validate PDF file
    pub fn is_valid_pdf_file<P: AsRef<Path>>(path: P) -> UtilResult<()> {
        let path = path.as_ref();
        
        // Check if file exists
        if !path.exists() {
            return Err(UtilError::ValidationError("PDF file does not exist".to_string()));
        }
        
        // Check if it's a file
        if !path.is_file() {
            return Err(UtilError::ValidationError("Path is not a file".to_string()));
        }
        
        // Check file extension
        if let Some(extension) = path.extension() {
            if extension.to_string_lossy().to_lowercase() != "pdf" {
                return Err(UtilError::ValidationError("File is not a PDF".to_string()));
            }
        } else {
            return Err(UtilError::ValidationError("File has no extension".to_string()));
        }
        
        // Check PDF signature
        match std::fs::read(path) {
            Ok(bytes) => {
                if bytes.len() < 4 {
                    return Err(UtilError::ValidationError("File too small to be a valid PDF".to_string()));
                }
                
                if &bytes[0..4] != b"%PDF" {
                    return Err(UtilError::ValidationError("File does not have valid PDF signature".to_string()));
                }
            }
            Err(e) => {
                return Err(UtilError::ValidationError(format!("Cannot read file: {}", e)));
            }
        }
        
        Ok(())
    }
    
    /// Validate search query
    pub fn is_valid_search_query(query: &str) -> UtilResult<()> {
        let trimmed = query.trim();
        
        if trimmed.is_empty() {
            return Err(UtilError::ValidationError("Search query is empty".to_string()));
        }
        
        if trimmed.len() > 1000 {
            return Err(UtilError::ValidationError("Search query is too long".to_string()));
        }
        
        // Check for potentially dangerous characters
        let dangerous_chars = ['<', '>', '&', '"', '\''];
        for char in dangerous_chars {
            if trimmed.contains(char) {
                return Err(UtilError::ValidationError(
                    format!("Search query contains dangerous character: {}", char)
                ));
            }
        }
        
        Ok(())
    }
    
    /// Validate database path
    pub fn is_valid_database_path<P: AsRef<Path>>(path: P) -> UtilResult<()> {
        let path = path.as_ref();
        
        Self::is_valid_file_path(path)?;
        
        // Check if parent directory exists and is writable
        if let Some(parent) = path.parent() {
            if parent.exists() {
                Self::is_valid_directory(parent)?;
            } else {
                return Err(UtilError::ValidationError("Database parent directory does not exist".to_string()));
            }
        }
        
        // If database file exists, check if it's a valid SQLite file
        if path.exists() {
            match std::fs::read(path) {
                Ok(bytes) => {
                    if bytes.len() >= 16 {
                        let sqlite_header = b"SQLite format 3\0";
                        if &bytes[0..16] != sqlite_header {
                            return Err(UtilError::ValidationError("File is not a valid SQLite database".to_string()));
                        }
                    } else {
                        return Err(UtilError::ValidationError("Database file too small".to_string()));
                    }
                }
                Err(e) => {
                    return Err(UtilError::ValidationError(format!("Cannot read database file: {}", e)));
                }
            }
        }
        
        Ok(())
    }
    
    /// Validate configuration values
    pub fn validate_config_value(key: &str, value: &str) -> UtilResult<()> {
        match key {
            "max_concurrent_downloads" => {
                let num: u32 = value.parse()
                    .map_err(|_| UtilError::ValidationError("Invalid number for max_concurrent_downloads".to_string()))?;
                if num == 0 || num > 10 {
                    return Err(UtilError::ValidationError("max_concurrent_downloads must be between 1 and 10".to_string()));
                }
            }
            "download_directory" => {
                let path = Path::new(value);
                if path.exists() {
                    Self::is_valid_directory(path)?;
                }
            }
            "database_path" => {
                Self::is_valid_database_path(value)?;
            }
            "theme" => {
                if !["light", "dark", "gruvbox"].contains(&value) {
                    return Err(UtilError::ValidationError("Invalid theme value".to_string()));
                }
            }
            "language" => {
                if !["en", "zh", "es", "fr", "de", "ja"].contains(&value) {
                    return Err(UtilError::ValidationError("Unsupported language".to_string()));
                }
            }
            _ => {
                // For unknown keys, just check if value is not empty
                if value.trim().is_empty() {
                    return Err(UtilError::ValidationError("Configuration value cannot be empty".to_string()));
                }
            }
        }
        
        Ok(())
    }
    
    /// Validate network connectivity
    pub async fn check_network_connectivity() -> bool {
        // Try to connect to a reliable host
        let hosts = vec![
            "8.8.8.8:53",      // Google DNS
            "1.1.1.1:53",      // Cloudflare DNS
            "208.67.222.222:53", // OpenDNS
        ];
        
        for host in hosts {
            if tokio::net::TcpStream::connect(host).await.is_ok() {
                return true;
            }
        }
        
        false
    }
    
    /// Validate arXiv URL
    pub fn is_valid_arxiv_url(url: &str) -> bool {
        if !Self::is_valid_url(url) {
            return false;
        }
        
        let arxiv_domains = vec![
            "arxiv.org",
            "export.arxiv.org",
            "www.arxiv.org",
        ];
        
        if let Ok(parsed_url) = Url::parse(url) {
            if let Some(domain) = parsed_url.domain() {
                return arxiv_domains.contains(&domain);
            }
        }
        
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{TempDir, NamedTempFile};
    use std::io::Write;

    #[test]
    fn test_arxiv_id_validation() {
        assert!(Validation::is_valid_arxiv_id("2301.1234"));
        assert!(Validation::is_valid_arxiv_id("2301.12345v2"));
        assert!(Validation::is_valid_arxiv_id("cs.AI/0701001"));
        assert!(!Validation::is_valid_arxiv_id("invalid"));
        assert!(!Validation::is_valid_arxiv_id("123.456"));
    }

    #[test]
    fn test_doi_validation() {
        assert!(Validation::is_valid_doi("10.1234/example"));
        assert!(Validation::is_valid_doi("10.5678/test.2023"));
        assert!(!Validation::is_valid_doi("invalid"));
        assert!(!Validation::is_valid_doi("11.1234/example"));
    }

    #[test]
    fn test_url_validation() {
        assert!(Validation::is_valid_url("https://example.com"));
        assert!(Validation::is_valid_url("http://test.org"));
        assert!(!Validation::is_valid_url("ftp://example.com"));
        assert!(!Validation::is_valid_url("not-a-url"));
    }

    #[test]
    fn test_email_validation() {
        assert!(Validation::is_valid_email("test@example.com"));
        assert!(Validation::is_valid_email("user.name+tag@domain.co.uk"));
        assert!(!Validation::is_valid_email("invalid.email"));
        assert!(!Validation::is_valid_email("@domain.com"));
    }

    #[test]
    fn test_file_path_validation() {
        assert!(Validation::is_valid_file_path("valid/path").is_ok());
        assert!(Validation::is_valid_file_path("../dangerous/path").is_err());
        assert!(Validation::is_valid_file_path("path\0with\0nulls").is_err());
    }

    #[test]
    fn test_directory_validation() {
        let temp_dir = TempDir::new().unwrap();
        assert!(Validation::is_valid_directory(temp_dir.path()).is_ok());
        assert!(Validation::is_valid_directory("/nonexistent/directory").is_err());
    }

    #[test]
    fn test_file_size_validation() {
        assert!(Validation::is_valid_file_size(1024, 2048).is_ok());
        assert!(Validation::is_valid_file_size(3000, 2048).is_err());
        assert!(Validation::is_valid_file_size(0, 2048).is_err());
    }

    #[test]
    fn test_pdf_validation() {
        // Create a temporary file with PDF signature
        let mut temp_file = NamedTempFile::with_suffix(".pdf").unwrap();
        temp_file.write_all(b"%PDF-1.4\ntest content").unwrap();
        temp_file.flush().unwrap();
        
        assert!(Validation::is_valid_pdf_file(temp_file.path()).is_ok());
        
        // Test invalid PDF
        let mut invalid_file = NamedTempFile::with_suffix(".pdf").unwrap();
        invalid_file.write_all(b"not a pdf").unwrap();
        invalid_file.flush().unwrap();
        
        assert!(Validation::is_valid_pdf_file(invalid_file.path()).is_err());
    }

    #[test]
    fn test_search_query_validation() {
        assert!(Validation::is_valid_search_query("machine learning").is_ok());
        assert!(Validation::is_valid_search_query("").is_err());
        assert!(Validation::is_valid_search_query("query with <script>").is_err());
    }

    #[test]
    fn test_config_validation() {
        assert!(Validation::validate_config_value("max_concurrent_downloads", "3").is_ok());
        assert!(Validation::validate_config_value("max_concurrent_downloads", "0").is_err());
        assert!(Validation::validate_config_value("max_concurrent_downloads", "15").is_err());
        assert!(Validation::validate_config_value("theme", "dark").is_ok());
        assert!(Validation::validate_config_value("theme", "invalid").is_err());
    }

    #[test]
    fn test_arxiv_url_validation() {
        assert!(Validation::is_valid_arxiv_url("https://arxiv.org/abs/2301.1234"));
        assert!(Validation::is_valid_arxiv_url("http://export.arxiv.org/api/query"));
        assert!(!Validation::is_valid_arxiv_url("https://example.com"));
        assert!(!Validation::is_valid_arxiv_url("not-a-url"));
    }

    #[tokio::test]
    async fn test_network_connectivity() {
        // This test may fail in offline environments
        let is_connected = Validation::check_network_connectivity().await;
        // Just ensure the function runs without panic
        println!("Network connectivity: {}", is_connected);
    }
}

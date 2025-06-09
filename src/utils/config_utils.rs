use super::{UtilError, UtilResult, AppSettings};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use crate::config::AppConfig;

/// Configuration utility functions
pub struct ConfigUtils;

impl ConfigUtils {
    /// Load application settings from file
    pub fn load_settings<P: AsRef<Path>>(config_path: P) -> UtilResult<AppSettings> {
        let path = config_path.as_ref();
        
        if !path.exists() {
            // Return default settings if config file doesn't exist
            return Ok(AppSettings::default());
        }
        
        let content = fs::read_to_string(path)
            .map_err(|e| UtilError::ConfigError(format!("Failed to read config file: {}", e)))?;
        
        let settings: AppSettings = toml::from_str(&content)
            .map_err(|e| UtilError::ConfigError(format!("Failed to parse config file: {}", e)))?;
        
        Ok(settings)
    }
    
    /// Save application settings to file
    pub fn save_settings<P: AsRef<Path>>(settings: &AppSettings, config_path: P) -> UtilResult<()> {
        let path = config_path.as_ref();
        
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| UtilError::ConfigError(format!("Failed to create config directory: {}", e)))?;
        }
        
        let content = toml::to_string_pretty(settings)
            .map_err(|e| UtilError::ConfigError(format!("Failed to serialize settings: {}", e)))?;
        
        fs::write(path, content)
            .map_err(|e| UtilError::ConfigError(format!("Failed to write config file: {}", e)))?;
        
        Ok(())
    }
    
    /// Get default configuration directory
    pub fn get_config_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
            .join("arxiv_manager")
    }
    
    /// Get configuration file path
    pub fn get_config_file_path() -> PathBuf {
        Self::get_config_dir().join("config.toml")
    }
    
    /// Initialize configuration with defaults
    pub fn init_default_config() -> UtilResult<AppSettings> {
        let config_path = Self::get_config_file_path();
        let settings = AppSettings::default();
        
        // Create necessary directories
        fs::create_dir_all(&settings.download_directory)
            .map_err(|e| UtilError::ConfigError(format!("Failed to create download directory: {}", e)))?;
        
        fs::create_dir_all(&settings.index_directory)
            .map_err(|e| UtilError::ConfigError(format!("Failed to create index directory: {}", e)))?;
        
        // Save default configuration
        Self::save_settings(&settings, &config_path)?;
        
        Ok(settings)
    }
    
    /// Update specific configuration value
    pub fn update_config_value<P: AsRef<Path>>(
        config_path: P,
        key: &str,
        value: &str,
    ) -> UtilResult<()> {
        let mut settings = Self::load_settings(&config_path)?;
        
        // Validate the value before updating
        crate::utils::validation::Validation::validate_config_value(key, value)?;
        
        // Update the specific field
        match key {
            "download_directory" => settings.download_directory = value.to_string(),
            "index_directory" => settings.index_directory = value.to_string(),
            "max_concurrent_downloads" => {
                settings.max_concurrent_downloads = value.parse()
                    .map_err(|e| UtilError::ConfigError(format!("Invalid number: {}", e)))?;
            }
            "auto_extract_text" => {
                settings.auto_extract_text = value.parse()
                    .map_err(|e| UtilError::ConfigError(format!("Invalid boolean: {}", e)))?;
            }
            "auto_index_papers" => {
                settings.auto_index_papers = value.parse()
                    .map_err(|e| UtilError::ConfigError(format!("Invalid boolean: {}", e)))?;
            }
            "theme" => settings.theme = value.to_string(),
            "language" => settings.language = value.to_string(),
            _ => return Err(UtilError::ConfigError(format!("Unknown configuration key: {}", key))),
        }
        
        Self::save_settings(&settings, config_path)?;
        Ok(())
    }
    
    /// Reset configuration to defaults
    pub fn reset_to_defaults<P: AsRef<Path>>(config_path: P) -> UtilResult<AppSettings> {
        let settings = AppSettings::default();
        Self::save_settings(&settings, config_path)?;
        Ok(settings)
    }
    
    /// Import configuration from another file
    pub fn import_config<P1: AsRef<Path>, P2: AsRef<Path>>(
        source_path: P1,
        target_path: P2,
    ) -> UtilResult<AppSettings> {
        let settings = Self::load_settings(source_path)?;
        Self::save_settings(&settings, target_path)?;
        Ok(settings)
    }
    
    /// Export configuration to a file
    pub fn export_config<P1: AsRef<Path>, P2: AsRef<Path>>(
        source_path: P1,
        target_path: P2,
    ) -> UtilResult<()> {
        let settings = Self::load_settings(source_path)?;
        Self::save_settings(&settings, target_path)?;
        Ok(())
    }
    
    /// Migrate old configuration format to new format
    pub fn migrate_config<P: AsRef<Path>>(config_path: P) -> UtilResult<AppSettings> {
        let path = config_path.as_ref();
        
        // Try to load as new format first
        match Self::load_settings(path) {
            Ok(settings) => return Ok(settings),
            Err(_) => {
                // If that fails, try to migrate from old format
                if let Ok(old_config) = Self::load_old_config_format(path) {
                    let new_settings = Self::convert_old_to_new_config(old_config)?;
                    Self::save_settings(&new_settings, path)?;
                    return Ok(new_settings);
                }
            }
        }
        
        // If all else fails, create default config
        Self::init_default_config()
    }
    
    /// Load old configuration format (for migration)
    fn load_old_config_format<P: AsRef<Path>>(config_path: P) -> UtilResult<OldConfigFormat> {
        let content = fs::read_to_string(config_path)
            .map_err(|e| UtilError::ConfigError(format!("Failed to read old config: {}", e)))?;
        
        // Try JSON format first
        if let Ok(config) = serde_json::from_str::<OldConfigFormat>(&content) {
            return Ok(config);
        }
        
        // Try TOML format
        toml::from_str::<OldConfigFormat>(&content)
            .map_err(|e| UtilError::ConfigError(format!("Failed to parse old config: {}", e)))
    }
    
    /// Convert old configuration format to new format
    fn convert_old_to_new_config(old_config: OldConfigFormat) -> UtilResult<AppSettings> {
        Ok(AppSettings {
            download_directory: old_config.download_dir.unwrap_or_else(super::default_download_dir),
            index_directory: old_config.index_dir.unwrap_or_else(super::default_index_dir),
            max_concurrent_downloads: old_config.max_downloads.unwrap_or(3),
            auto_extract_text: old_config.auto_extract.unwrap_or(true),
            auto_index_papers: old_config.auto_index.unwrap_or(true),
            theme: old_config.theme.unwrap_or_else(|| "gruvbox".to_string()),
            language: old_config.language.unwrap_or_else(|| "en".to_string()),
        })
    }
    
    /// Validate configuration file
    pub fn validate_config<P: AsRef<Path>>(config_path: P) -> UtilResult<()> {
        let settings = Self::load_settings(config_path)?;
        
        // Validate each setting
        if settings.max_concurrent_downloads == 0 || settings.max_concurrent_downloads > 10 {
            return Err(UtilError::ConfigError(
                "max_concurrent_downloads must be between 1 and 10".to_string()
            ));
        }
        
        // Validate directories exist or can be created
        if !Path::new(&settings.download_directory).exists() {
            fs::create_dir_all(&settings.download_directory)
                .map_err(|e| UtilError::ConfigError(
                    format!("Cannot create download directory: {}", e)
                ))?;
        }
        
        if !Path::new(&settings.index_directory).exists() {
            fs::create_dir_all(&settings.index_directory)
                .map_err(|e| UtilError::ConfigError(
                    format!("Cannot create index directory: {}", e)
                ))?;
        }
        
        // Validate theme
        if !["light", "dark", "gruvbox"].contains(&settings.theme.as_str()) {
            return Err(UtilError::ConfigError(
                format!("Unknown theme: {}", settings.theme)
            ));
        }
        
        // Validate language
        if !["en", "zh", "es", "fr", "de", "ja"].contains(&settings.language.as_str()) {
            return Err(UtilError::ConfigError(
                format!("Unsupported language: {}", settings.language)
            ));
        }
        
        Ok(())
    }
    
    /// Get configuration summary for display
    pub fn get_config_summary<P: AsRef<Path>>(config_path: P) -> UtilResult<ConfigSummary> {
        let config_path_ref = config_path.as_ref();
        let settings = Self::load_settings(&config_path)?;
        
        Ok(ConfigSummary {
            download_directory: settings.download_directory.clone(),
            index_directory: settings.index_directory.clone(),
            max_concurrent_downloads: settings.max_concurrent_downloads,
            auto_extract_text: settings.auto_extract_text,
            auto_index_papers: settings.auto_index_papers,
            theme: settings.theme.clone(),
            language: settings.language.clone(),
            config_file_size: Self::get_config_file_size(config_path_ref),
            last_modified: Self::get_config_last_modified(config_path_ref),
        })
    }
    
    /// Get configuration file size
    fn get_config_file_size(config_path: &Path) -> Option<u64> {
        fs::metadata(config_path).ok().map(|m| m.len())
    }
    
    /// Get configuration file last modified time
    fn get_config_last_modified(config_path: &Path) -> Option<String> {
        use crate::utils::date_utils::DateUtils;
        
        fs::metadata(config_path)
            .ok()
            .and_then(|m| m.modified().ok())
            .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|duration| {
                let datetime = chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0)
                    .unwrap_or_default();
                DateUtils::format_display_date(&datetime)
            })
    }
}

/// Old configuration format for migration
#[derive(Debug, Deserialize)]
struct OldConfigFormat {
    download_dir: Option<String>,
    index_dir: Option<String>,
    max_downloads: Option<usize>,
    auto_extract: Option<bool>,
    auto_index: Option<bool>,
    theme: Option<String>,
    language: Option<String>,
}

/// Configuration summary for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSummary {
    pub download_directory: String,
    pub index_directory: String,
    pub max_concurrent_downloads: usize,
    pub auto_extract_text: bool,
    pub auto_index_papers: bool,
    pub theme: String,
    pub language: String,
    pub config_file_size: Option<u64>,
    pub last_modified: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{TempDir, NamedTempFile};
    use std::io::Write;

    #[test]
    fn test_load_default_settings() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("nonexistent.toml");
        
        let settings = ConfigUtils::load_settings(&config_path).unwrap();
        assert_eq!(settings.max_concurrent_downloads, 3);
        assert!(settings.auto_extract_text);
    }

    #[test]
    fn test_save_and_load_settings() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");
        
        let original_settings = AppSettings {
            download_directory: "/tmp/downloads".to_string(),
            max_concurrent_downloads: 5,
            theme: "dark".to_string(),
            ..Default::default()
        };
        
        ConfigUtils::save_settings(&original_settings, &config_path).unwrap();
        let loaded_settings = ConfigUtils::load_settings(&config_path).unwrap();
        
        assert_eq!(loaded_settings.download_directory, "/tmp/downloads");
        assert_eq!(loaded_settings.max_concurrent_downloads, 5);
        assert_eq!(loaded_settings.theme, "dark");
    }

    #[test]
    fn test_update_config_value() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");
        
        // Create initial config
        let settings = AppSettings::default();
        ConfigUtils::save_settings(&settings, &config_path).unwrap();
        
        // Update a value
        ConfigUtils::update_config_value(&config_path, "max_concurrent_downloads", "7").unwrap();
        
        // Verify the update
        let updated_settings = ConfigUtils::load_settings(&config_path).unwrap();
        assert_eq!(updated_settings.max_concurrent_downloads, 7);
    }

    #[test]
    fn test_validate_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");
        
        let settings = AppSettings {
            download_directory: temp_dir.path().join("downloads").to_string_lossy().to_string(),
            index_directory: temp_dir.path().join("index").to_string_lossy().to_string(),
            ..Default::default()
        };
        
        ConfigUtils::save_settings(&settings, &config_path).unwrap();
        assert!(ConfigUtils::validate_config(&config_path).is_ok());
    }

    #[test]
    fn test_config_migration() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("old_config.json");
        
        // Create old format config
        let old_config = r#"{
            "download_dir": "/old/downloads",
            "max_downloads": 2,
            "theme": "light"
        }"#;
        
        std::fs::write(&config_path, old_config).unwrap();
        
        // Migrate
        let migrated_settings = ConfigUtils::migrate_config(&config_path).unwrap();
        assert_eq!(migrated_settings.download_directory, "/old/downloads");
        assert_eq!(migrated_settings.max_concurrent_downloads, 2);
        assert_eq!(migrated_settings.theme, "light");
    }

    #[test]
    fn test_config_summary() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");
        
        let settings = AppSettings::default();
        ConfigUtils::save_settings(&settings, &config_path).unwrap();
        
        let summary = ConfigUtils::get_config_summary(&config_path).unwrap();
        assert_eq!(summary.max_concurrent_downloads, 3);
        assert!(summary.auto_extract_text);
        assert!(summary.config_file_size.is_some());
    }

    #[test]
    fn test_export_import_config() {
        let temp_dir = TempDir::new().unwrap();
        let source_path = temp_dir.path().join("source.toml");
        let target_path = temp_dir.path().join("target.toml");
        
        let settings = AppSettings {
            theme: "dark".to_string(),
            max_concurrent_downloads: 8,
            ..Default::default()
        };
        
        ConfigUtils::save_settings(&settings, &source_path).unwrap();
        ConfigUtils::export_config(&source_path, &target_path).unwrap();
        
        let imported_settings = ConfigUtils::import_config(&target_path, &source_path).unwrap();
        assert_eq!(imported_settings.theme, "dark");
        assert_eq!(imported_settings.max_concurrent_downloads, 8);
    }
}

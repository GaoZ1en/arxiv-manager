use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::utils::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub download: DownloadConfig,
    pub ui: UiConfig,
    pub database: DatabaseConfig,
    pub search: SearchConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadConfig {
    pub download_dir: PathBuf,
    pub max_concurrent_downloads: usize,
    pub retry_attempts: u32,
    pub timeout_seconds: u64,
    pub auto_organize: bool,
    pub naming_pattern: String, // e.g., "{year}/{category}/{title}"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: Theme,
    pub language: Language,
    pub font_size: f32,
    pub window_width: u32,
    pub window_height: u32,
    pub vim_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub db_path: PathBuf,
    pub enable_backup: bool,
    pub backup_interval_hours: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub index_path: PathBuf,
    pub enable_fulltext_search: bool,
    pub max_search_results: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    GruvboxDark,
    GruvboxLight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Language {
    Chinese,
    English,
}

impl Default for Config {
    fn default() -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let app_dir = home_dir.join(".arxiv-manager");
        
        Self {
            download: DownloadConfig {
                download_dir: app_dir.join("papers"),
                max_concurrent_downloads: 3,
                retry_attempts: 3,
                timeout_seconds: 30,
                auto_organize: true,
                naming_pattern: "{year}/{category}/{title}".to_string(),
            },
            ui: UiConfig {
                theme: Theme::GruvboxDark,
                language: Language::Chinese,
                font_size: 14.0,
                window_width: 1200,
                window_height: 800,
                vim_mode: true,
            },
            database: DatabaseConfig {
                db_path: app_dir.join("arxiv.db"),
                enable_backup: true,
                backup_interval_hours: 24,
            },
            search: SearchConfig {
                index_path: app_dir.join("search_index"),
                enable_fulltext_search: true,
                max_search_results: 100,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path();
        
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)
                .map_err(|e| crate::utils::ArxivError::Config(e.to_string()))?;
            Ok(config)
        } else {
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }
    
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path();
        
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let content = toml::to_string_pretty(self)
            .map_err(|e| crate::utils::ArxivError::Config(e.to_string()))?;
        std::fs::write(&config_path, content)?;
        
        Ok(())
    }
    
    fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")))
            .join("arxiv-manager")
            .join("config.toml")
    }
    
    pub fn ensure_directories(&self) -> Result<()> {
        std::fs::create_dir_all(&self.download.download_dir)?;
        
        if let Some(parent) = self.database.db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        std::fs::create_dir_all(&self.search.index_path)?;
        
        Ok(())
    }
}

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 应用配置
    pub app: AppSettings,
    /// 下载配置
    pub download: DownloadSettings,
    /// UI 配置
    pub ui: UISettings,
    /// 数据库配置
    pub database: DatabaseSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// 应用数据目录
    pub data_dir: PathBuf,
    /// 语言设置
    pub language: String,
    /// 日志级别
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadSettings {
    /// 下载目录
    pub download_dir: PathBuf,
    /// 最大并发下载数
    pub max_concurrent_downloads: usize,
    /// 下载超时时间（秒）
    pub timeout_seconds: u64,
    /// 重试次数
    pub max_retries: usize,
    /// 下载速度限制（字节/秒，0表示无限制）
    pub speed_limit: u64,
    /// 文件命名规则
    pub naming_pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UISettings {
    /// 主题
    pub theme: String,
    /// 字体大小
    pub font_size: f32,
    /// 窗口大小
    pub window_size: (u32, u32),
    /// 侧边栏宽度
    pub sidebar_width: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    /// 数据库文件路径
    pub db_path: PathBuf,
    /// 索引目录
    pub index_dir: PathBuf,
}

impl Default for AppConfig {
    fn default() -> Self {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("arxiv-manager");

        Self {
            app: AppSettings {
                data_dir: data_dir.clone(),
                language: "zh-CN".to_string(),
                log_level: "info".to_string(),
            },
            download: DownloadSettings {
                download_dir: data_dir.join("papers"),
                max_concurrent_downloads: 4,
                timeout_seconds: 300,
                max_retries: 3,
                speed_limit: 0,
                naming_pattern: "{id}_{title}".to_string(),
            },
            ui: UISettings {
                theme: "gruvbox_dark".to_string(),
                font_size: 14.0,
                window_size: (1200, 800),
                sidebar_width: 300.0,
            },
            database: DatabaseSettings {
                db_path: data_dir.join("arxiv.db"),
                index_dir: data_dir.join("search_index"),
            },
        }
    }
}

impl AppConfig {
    /// 加载配置文件
    pub fn load() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("arxiv-manager");

        let config_path = config_dir.join("config.toml");

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: AppConfig = toml::from_str(&content)?;
            Ok(config)
        } else {
            // 创建默认配置
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    /// 保存配置文件
    pub fn save(&self) -> Result<()> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("arxiv-manager");

        std::fs::create_dir_all(&config_dir)?;

        let config_path = config_dir.join("config.toml");
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;

        Ok(())
    }

    /// 确保必要的目录存在
    pub fn ensure_directories(&self) -> Result<()> {
        std::fs::create_dir_all(&self.app.data_dir)?;
        std::fs::create_dir_all(&self.download.download_dir)?;
        std::fs::create_dir_all(&self.database.index_dir)?;
        Ok(())
    }
}

mod app;
mod config;
mod core;
mod database;
mod downloader;
mod pdf;
mod search;
mod ui;
mod utils;

use anyhow::Result;
use iced::{Application, Settings};
use tracing::{info, Level};
use tracing_subscriber;

use app::ArxivManager;
use config::AppConfig;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志系统
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("启动 arXiv 管理器...");

    // 加载配置
    let config = AppConfig::load().unwrap_or_default();

    // 初始化数据库
    database::init().await?;

    // 启动 GUI 应用
    let settings = Settings {
        window: iced::window::Settings {
            size: (1200, 800),
            min_size: Some((800, 600)),
            max_size: None,
            position: iced::window::Position::default(),
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            level: iced::window::Level::Normal,
            icon: None,
            platform_specific: iced::window::PlatformSpecific::default(),
        },
        flags: config,
        id: None,
        antialiasing: true,
        default_font: iced::Font::default(),
        default_text_size: 14.0,
        exit_on_close_request: true,
    };

    ArxivManager::run(settings)?;

    Ok(())
}

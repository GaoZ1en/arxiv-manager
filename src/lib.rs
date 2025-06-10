// 库文件 - 模块化架构支持

pub mod core;
pub mod database;
pub mod downloader;
pub mod config;
pub mod utils;
pub mod ui;
pub mod app;
// pub mod appearance; // 暂时移除旧的appearance系统

// 搜索模块
pub mod search;

pub use core::*;
pub use utils::*;

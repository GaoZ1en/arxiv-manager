// 库文件 - 模块化架构支持

pub mod core;
pub mod database;
pub mod downloader;
pub mod config;
pub mod utils;
pub mod ui;
pub mod app;
// pub mod appearance; // 暂时移除旧的appearance系统

// 新的模块化组件
pub mod models;
pub mod messages;
pub mod app_state;
pub mod views;
pub mod theme;
pub mod style;
pub mod services;

pub use core::*;
pub use utils::*;

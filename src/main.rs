// arXiv 论文管理器 - 带有 halloy 风格界面
// 模块化架构 - 主入口文件

mod core;
mod ui;
mod search;
mod utils;

use core::ArxivManager;

// 确保所有UI实现被包含
// use ui::*;

fn main() -> iced::Result {
    env_logger::init();
    
    iced::application("arXiv Paper Manager", ArxivManager::update, ArxivManager::view)
        .subscription(ArxivManager::subscription)
        .theme(ArxivManager::theme)
        .window_size((1400.0, 900.0))
        .run_with(|| {
            let (app, task) = ArxivManager::new();
            (app, task)
        })
}

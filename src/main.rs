// arXiv 论文管理器 - 带有 halloy 风格界面
// 模块化架构 - 主入口文件

mod models;
mod messages;
mod app_state;
mod views;
mod theme;
mod style;
mod services;

use app_state::ArxivManager;

fn main() -> iced::Result {
    env_logger::init();
    
    iced::application("arXiv Paper Manager", ArxivManager::update, ArxivManager::view)
        .theme(ArxivManager::theme)
        .window_size((1400.0, 900.0))
        .run_with(|| {
            let (app, task) = ArxivManager::new();
            (app, task)
        })
}

// 消息处理器模块
// 将巨大的update方法拆分为专门的处理器模块

pub mod tab_handler;
pub mod search_handler;
pub mod download_handler;
pub mod settings_handler;
pub mod command_handler;
pub mod paper_handler;
pub mod shortcut_handler;

// 重新导出处理器 trait
pub use tab_handler::TabHandler;
pub use search_handler::SearchHandler;
pub use download_handler::DownloadHandler;
pub use settings_handler::SettingsHandler;
pub use command_handler::CommandHandler;
pub use paper_handler::PaperHandler;
pub use shortcut_handler::ShortcutHandler;

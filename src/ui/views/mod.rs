// 视图模块

pub mod search;
pub mod library;
pub mod downloads;
pub mod settings;
pub mod paper;

// 重新导出主要视图
pub use search::SearchView;
pub use library::LibraryView;
pub use downloads::DownloadsView;
pub use settings::SettingsView;
pub use paper::PaperView;

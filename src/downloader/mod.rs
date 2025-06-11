// 下载器模块 - 提供arXiv论文PDF下载功能

// 子模块
pub mod types;
pub mod manager;
pub mod queue;
pub mod utils;

// 重新导出主要类型和函数
pub use types::{DownloadTask, Priority, DownloadEvent};
pub use manager::DownloadManager;
pub use queue::DownloadQueue;
pub use utils::{generate_file_path, sanitize_filename};

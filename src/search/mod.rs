// 搜索模块 - ArXiv论文搜索功能

pub mod api;
pub mod parsers;
pub mod filters;
pub mod downloader;
pub mod services; // 保留原始services以维持兼容性

// 重新导出主要功能
// pub use api::*;
// pub use parsers::*;
// pub use filters::*;
// pub use downloader::*;
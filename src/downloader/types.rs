// 下载器类型定义模块

use crate::core::ArxivPaper;
use std::path::PathBuf;

/// 下载任务
#[derive(Debug, Clone)]
pub struct DownloadTask {
    pub paper: ArxivPaper,
    pub output_path: PathBuf,
    pub priority: Priority,
}

/// 下载优先级
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
}

/// 下载事件
#[derive(Debug, Clone)]
pub enum DownloadEvent {
    Started { arxiv_id: String },
    Progress { arxiv_id: String, bytes_downloaded: u64, total_bytes: Option<u64> },
    Completed { arxiv_id: String, file_path: PathBuf },
    Failed { arxiv_id: String, error: String },
}

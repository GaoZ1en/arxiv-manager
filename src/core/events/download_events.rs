
use crate::core::ArxivPaper;
// use arxiv_manager::downloader::{DownloadTask, Priority};
use std::path::PathBuf;

// 临时类型定义以避免循环导入
#[derive(Debug, Clone)]
pub struct DownloadTask {
    pub paper: ArxivPaper,
    pub output_path: PathBuf,
    pub priority: Priority,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
}

/// 下载相关事件
#[derive(Debug, Clone)]
pub enum DownloadEvent {
    /// 下载任务添加
    TaskAdded {
        task: DownloadTask,
        queue_position: usize,
    },
    
    /// 下载开始
    Started {
        arxiv_id: String,
        paper: ArxivPaper,
        output_path: PathBuf,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    
    /// 下载进度更新
    Progress {
        arxiv_id: String,
        bytes_downloaded: u64,
        total_bytes: Option<u64>,
        speed_bps: Option<u64>,
        eta_seconds: Option<u64>,
    },
    
    /// 下载暂停
    Paused {
        arxiv_id: String,
        bytes_downloaded: u64,
    },
    
    /// 下载恢复
    Resumed {
        arxiv_id: String,
        bytes_downloaded: u64,
    },
    
    /// 下载完成
    Completed {
        arxiv_id: String,
        file_path: PathBuf,
        file_size: u64,
        download_time_ms: u64,
        average_speed_bps: u64,
    },
    
    /// 下载失败
    Failed {
        arxiv_id: String,
        error: String,
        error_type: DownloadErrorType,
        retry_count: u32,
        can_retry: bool,
    },
    
    /// 下载取消
    Cancelled {
        arxiv_id: String,
        reason: CancelReason,
    },
    
    /// 重试下载
    Retrying {
        arxiv_id: String,
        attempt: u32,
        max_attempts: u32,
    },
    
    /// 队列管理事件
    Queue(QueueEvent),
    
    /// 批量下载事件
    Batch(BatchDownloadEvent),
    
    /// 下载统计事件
    Statistics(DownloadStatisticsEvent),
    
    /// 下载设置事件
    Settings(DownloadSettingsEvent),
}

/// 下载错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum DownloadErrorType {
    /// 网络连接错误
    NetworkError,
    
    /// HTTP错误（包含状态码）
    HttpError(u16),
    
    /// 文件系统错误
    FileSystemError,
    
    /// URL无效
    InvalidUrl,
    
    /// 超时
    Timeout,
    
    /// 权限不足
    PermissionDenied,
    
    /// 磁盘空间不足
    DiskFull,
    
    /// 文件已存在
    FileExists,
    
    /// 服务器错误
    ServerError,
    
    /// 解析错误
    ParseError,
    
    /// 用户取消
    UserCancelled,
    
    /// 配额超出
    QuotaExceeded,
    
    /// 文件损坏
    CorruptedFile,
    
    /// 未知错误
    Unknown,
}

/// 取消原因
#[derive(Debug, Clone, PartialEq)]
pub enum CancelReason {
    /// 用户手动取消
    UserRequested,
    
    /// 系统自动取消
    SystemCancelled,
    
    /// 磁盘空间不足
    InsufficientSpace,
    
    /// 达到错误限制
    ErrorLimitReached,
    
    /// 应用程序关闭
    ApplicationShutdown,
    
    /// 网络不可用
    NetworkUnavailable,
}

/// 队列事件
#[derive(Debug, Clone)]
pub enum QueueEvent {
    /// 任务添加到队列
    TaskQueued {
        arxiv_id: String,
        priority: Priority,
        position: usize,
    },
    
    /// 任务从队列移除
    TaskDequeued {
        arxiv_id: String,
        reason: DequeueReason,
    },
    
    /// 优先级改变
    PriorityChanged {
        arxiv_id: String,
        old_priority: Priority,
        new_priority: Priority,
    },
    
    /// 队列重新排序
    QueueReordered {
        new_order: Vec<String>, // arxiv_ids in new order
    },
    
    /// 队列清空
    QueueCleared {
        removed_count: usize,
    },
    
    /// 队列状态更新
    StatusUpdated {
        queue_size: usize,
        active_downloads: usize,
        completed_downloads: usize,
        failed_downloads: usize,
    },
}

/// 出队原因
#[derive(Debug, Clone, PartialEq)]
pub enum DequeueReason {
    /// 开始下载
    StartedDownload,
    
    /// 用户移除
    UserRemoved,
    
    /// 系统清理
    SystemCleanup,
    
    /// 重复任务
    Duplicate,
    
    /// 错误
    Error,
}

/// 批量下载事件
#[derive(Debug, Clone)]
pub enum BatchDownloadEvent {
    /// 批量下载开始
    Started {
        papers: Vec<ArxivPaper>,
        total_count: usize,
    },
    
    /// 批量下载进度
    Progress {
        completed: usize,
        failed: usize,
        remaining: usize,
        current_paper: Option<String>, // arxiv_id
    },
    
    /// 批量下载完成
    Completed {
        total_count: usize,
        successful_count: usize,
        failed_count: usize,
        duration_ms: u64,
    },
    
    /// 批量下载暂停
    Paused {
        completed: usize,
        remaining: usize,
    },
    
    /// 批量下载恢复
    Resumed {
        remaining: usize,
    },
    
    /// 批量下载取消
    Cancelled {
        completed: usize,
        cancelled: usize,
    },
}

/// 下载统计事件
#[derive(Debug, Clone)]
pub enum DownloadStatisticsEvent {
    /// 统计数据更新
    StatsUpdated {
        total_downloads: u64,
        successful_downloads: u64,
        failed_downloads: u64,
        total_bytes_downloaded: u64,
        average_speed_bps: u64,
        success_rate: f64,
    },
    
    /// 性能指标记录
    PerformanceLogged {
        arxiv_id: String,
        file_size: u64,
        download_time_ms: u64,
        average_speed_bps: u64,
        peak_speed_bps: u64,
    },
    
    /// 每日统计更新
    DailyStatsUpdated {
        date: chrono::NaiveDate,
        downloads: u64,
        bytes_downloaded: u64,
        average_speed: u64,
    },
    
    /// 错误统计更新
    ErrorStatsUpdated {
        error_type: DownloadErrorType,
        count: u64,
        percentage: f64,
    },
    
    /// 热门论文更新
    PopularPapersUpdated {
        papers: Vec<(String, u64)>, // (arxiv_id, download_count)
    },
    
    /// 统计重置
    StatsReset,
    
    /// 统计导出
    StatsExported {
        file_path: PathBuf,
        format: StatisticsFormat,
    },
}

/// 统计格式
#[derive(Debug, Clone, PartialEq)]
pub enum StatisticsFormat {
    Json,
    Csv,
    Html,
    Pdf,
}

/// 下载设置事件
#[derive(Debug, Clone)]
pub enum DownloadSettingsEvent {
    /// 最大并发数改变
    MaxConcurrentChanged {
        old_value: usize,
        new_value: usize,
    },
    
    /// 重试次数改变
    RetryAttemptsChanged {
        old_value: u32,
        new_value: u32,
    },
    
    /// 超时时间改变
    TimeoutChanged {
        old_value: u64,
        new_value: u64,
    },
    
    /// 下载目录改变
    DownloadDirChanged {
        old_path: PathBuf,
        new_path: PathBuf,
    },
    
    /// 文件命名模式改变
    NamingPatternChanged {
        old_pattern: String,
        new_pattern: String,
    },
    
    /// 速度限制改变
    SpeedLimitChanged {
        old_limit: Option<u64>,
        new_limit: Option<u64>,
    },
    
    /// 自动开始下载设置改变
    AutoStartChanged {
        enabled: bool,
    },
    
    /// 覆盖现有文件设置改变
    OverwriteExistingChanged {
        enabled: bool,
    },
    
    /// 设置验证
    SettingsValidated {
        valid: bool,
        errors: Vec<String>,
    },
    
    /// 设置重置
    SettingsReset,
}

/// 下载事件处理器特征
pub trait DownloadEventHandler {
    /// 处理下载事件
    fn handle_download_event(&mut self, event: &DownloadEvent) -> Result<(), DownloadEventError>;
}

/// 下载事件错误
#[derive(Debug, thiserror::Error)]
pub enum DownloadEventError {
    #[error("Download operation failed: {0}")]
    DownloadFailed(String),
    
    #[error("Queue operation failed: {0}")]
    QueueFailed(String),
    
    #[error("Batch operation failed: {0}")]
    BatchFailed(String),
    
    #[error("Statistics operation failed: {0}")]
    StatisticsFailed(String),
    
    #[error("Settings operation failed: {0}")]
    SettingsFailed(String),
    
    #[error("File operation failed: {0}")]
    FileFailed(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Network error: {0}")]
    NetworkError(String),
}

/// 下载事件构建器
pub struct DownloadEventBuilder {
    arxiv_id: Option<String>,
    paper: Option<ArxivPaper>,
    file_path: Option<PathBuf>,
    timestamp: Option<chrono::DateTime<chrono::Utc>>,
    error_type: Option<DownloadErrorType>,
    bytes_downloaded: Option<u64>,
    total_bytes: Option<u64>,
}

impl DownloadEventBuilder {
    /// 创建新的下载事件构建器
    pub fn new() -> Self {
        Self {
            arxiv_id: None,
            paper: None,
            file_path: None,
            timestamp: None,
            error_type: None,
            bytes_downloaded: None,
            total_bytes: None,
        }
    }
    
    /// 设置ArXiv ID
    pub fn with_arxiv_id(mut self, arxiv_id: String) -> Self {
        self.arxiv_id = Some(arxiv_id);
        self
    }
    
    /// 设置论文
    pub fn with_paper(mut self, paper: ArxivPaper) -> Self {
        self.paper = Some(paper);
        self
    }
    
    /// 设置文件路径
    pub fn with_file_path(mut self, file_path: PathBuf) -> Self {
        self.file_path = Some(file_path);
        self
    }
    
    /// 设置时间戳
    pub fn with_timestamp(mut self, timestamp: chrono::DateTime<chrono::Utc>) -> Self {
        self.timestamp = Some(timestamp);
        self
    }
    
    /// 设置错误类型
    pub fn with_error_type(mut self, error_type: DownloadErrorType) -> Self {
        self.error_type = Some(error_type);
        self
    }
    
    /// 设置已下载字节数
    pub fn with_bytes_downloaded(mut self, bytes: u64) -> Self {
        self.bytes_downloaded = Some(bytes);
        self
    }
    
    /// 设置总字节数
    pub fn with_total_bytes(mut self, bytes: u64) -> Self {
        self.total_bytes = Some(bytes);
        self
    }
    
    /// 构建下载开始事件
    pub fn build_started(self) -> Option<DownloadEvent> {
        if let (Some(arxiv_id), Some(paper), Some(output_path), Some(timestamp)) = 
            (self.arxiv_id, self.paper, self.file_path, self.timestamp) {
            Some(DownloadEvent::Started {
                arxiv_id,
                paper,
                output_path,
                timestamp,
            })
        } else {
            None
        }
    }
    
    /// 构建下载进度事件
    pub fn build_progress(self, speed_bps: Option<u64>, eta_seconds: Option<u64>) -> Option<DownloadEvent> {
        if let (Some(arxiv_id), Some(bytes_downloaded)) = (self.arxiv_id, self.bytes_downloaded) {
            Some(DownloadEvent::Progress {
                arxiv_id,
                bytes_downloaded,
                total_bytes: self.total_bytes,
                speed_bps,
                eta_seconds,
            })
        } else {
            None
        }
    }
    
    /// 构建下载失败事件
    pub fn build_failed(self, error: String, retry_count: u32, can_retry: bool) -> Option<DownloadEvent> {
        if let (Some(arxiv_id), Some(error_type)) = (self.arxiv_id, self.error_type) {
            Some(DownloadEvent::Failed {
                arxiv_id,
                error,
                error_type,
                retry_count,
                can_retry,
            })
        } else {
            None
        }
    }
}

impl Default for DownloadEventBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 下载会话跟踪器
pub struct DownloadSessionTracker {
    sessions: std::collections::HashMap<String, DownloadSession>,
}

/// 下载会话信息
#[derive(Debug, Clone)]
pub struct DownloadSession {
    pub arxiv_id: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub paper: ArxivPaper,
    pub events: Vec<DownloadEvent>,
    pub status: DownloadSessionStatus,
    pub final_file_path: Option<PathBuf>,
    pub total_bytes: Option<u64>,
    pub bytes_downloaded: u64,
}

/// 下载会话状态
#[derive(Debug, Clone, PartialEq)]
pub enum DownloadSessionStatus {
    Active,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

impl DownloadSessionTracker {
    /// 创建新的会话跟踪器
    pub fn new() -> Self {
        Self {
            sessions: std::collections::HashMap::new(),
        }
    }
    
    /// 开始新的下载会话
    pub fn start_session(&mut self, arxiv_id: String, paper: ArxivPaper) {
        let session = DownloadSession {
            arxiv_id: arxiv_id.clone(),
            start_time: chrono::Utc::now(),
            paper,
            events: Vec::new(),
            status: DownloadSessionStatus::Active,
            final_file_path: None,
            total_bytes: None,
            bytes_downloaded: 0,
        };
        
        self.sessions.insert(arxiv_id, session);
    }
    
    /// 更新会话事件
    pub fn add_event(&mut self, arxiv_id: &str, event: DownloadEvent) {
        if let Some(session) = self.sessions.get_mut(arxiv_id) {
            // 根据事件类型更新会话状态
            match &event {
                DownloadEvent::Progress { bytes_downloaded, total_bytes, .. } => {
                    session.bytes_downloaded = *bytes_downloaded;
                    if total_bytes.is_some() {
                        session.total_bytes = *total_bytes;
                    }
                }
                DownloadEvent::Completed { file_path, .. } => {
                    session.status = DownloadSessionStatus::Completed;
                    session.final_file_path = Some(file_path.clone());
                }
                DownloadEvent::Failed { .. } => {
                    session.status = DownloadSessionStatus::Failed;
                }
                DownloadEvent::Cancelled { .. } => {
                    session.status = DownloadSessionStatus::Cancelled;
                }
                DownloadEvent::Paused { .. } => {
                    session.status = DownloadSessionStatus::Paused;
                }
                DownloadEvent::Resumed { .. } => {
                    session.status = DownloadSessionStatus::Active;
                }
                _ => {}
            }
            
            session.events.push(event);
        }
    }
    
    /// 获取会话信息
    pub fn get_session(&self, arxiv_id: &str) -> Option<&DownloadSession> {
        self.sessions.get(arxiv_id)
    }
    
    /// 获取活跃会话
    pub fn get_active_sessions(&self) -> Vec<&DownloadSession> {
        self.sessions
            .values()
            .filter(|session| session.status == DownloadSessionStatus::Active)
            .collect()
    }
    
    /// 清理已完成的会话
    pub fn cleanup_completed_sessions(&mut self) -> usize {
        let initial_count = self.sessions.len();
        self.sessions.retain(|_, session| {
            !matches!(session.status, DownloadSessionStatus::Completed | DownloadSessionStatus::Failed | DownloadSessionStatus::Cancelled)
        });
        initial_count - self.sessions.len()
    }
}

impl Default for DownloadSessionTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_download_event_builder() {
        let builder = DownloadEventBuilder::new()
            .with_arxiv_id("test123".to_string())
            .with_bytes_downloaded(1024)
            .with_total_bytes(2048);
        
        let event = builder.build_progress(Some(1000), Some(1));
        assert!(event.is_some());
        
        if let Some(DownloadEvent::Progress { arxiv_id, bytes_downloaded, .. }) = event {
            assert_eq!(arxiv_id, "test123");
            assert_eq!(bytes_downloaded, 1024);
        }
    }
    
    #[test]
    fn test_download_session_tracker() {
        let mut tracker = DownloadSessionTracker::new();
        let paper = ArxivPaper::default(); // Assuming ArxivPaper has Default
        
        tracker.start_session("test123".to_string(), paper);
        
        let event = DownloadEvent::Progress {
            arxiv_id: "test123".to_string(),
            bytes_downloaded: 1024,
            total_bytes: Some(2048),
            speed_bps: Some(1000),
            eta_seconds: Some(1),
        };
        
        tracker.add_event("test123", event);
        
        let session = tracker.get_session("test123");
        assert!(session.is_some());
        
        if let Some(session) = session {
            assert_eq!(session.bytes_downloaded, 1024);
            assert_eq!(session.total_bytes, Some(2048));
            assert_eq!(session.events.len(), 1);
        }
    }
    
    #[test]
    fn test_download_error_types() {
        assert_eq!(DownloadErrorType::NetworkError, DownloadErrorType::NetworkError);
        assert_ne!(DownloadErrorType::NetworkError, DownloadErrorType::Timeout);
        
        let http_error = DownloadErrorType::HttpError(404);
        if let DownloadErrorType::HttpError(code) = http_error {
            assert_eq!(code, 404);
        }
    }
    
    #[test]
    fn test_cancel_reasons() {
        assert_eq!(CancelReason::UserRequested, CancelReason::UserRequested);
        assert_ne!(CancelReason::UserRequested, CancelReason::SystemCancelled);
    }
}

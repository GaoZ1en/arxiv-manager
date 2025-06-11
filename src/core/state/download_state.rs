
use crate::core::ArxivPaper;
// use crate::downloader::{DownloadTask, DownloadQueue};
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

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

#[derive(Debug)]
pub struct DownloadQueue {
    tasks: Vec<DownloadTask>,
}

impl DownloadQueue {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
        }
    }
    
    pub fn add_task(&mut self, task: DownloadTask) {
        self.tasks.push(task);
        self.tasks.sort_by(|a, b| b.priority.cmp(&a.priority));
    }
    
    pub fn next_task(&mut self) -> Option<DownloadTask> {
        self.tasks.pop()
    }
    
    pub fn remove_task(&mut self, arxiv_id: &str) -> Option<DownloadTask> {
        if let Some(index) = self.tasks.iter().position(|t| t.paper.id == arxiv_id) {
            Some(self.tasks.remove(index))
        } else {
            None
        }
    }
    
    pub fn contains_task(&self, arxiv_id: &str) -> bool {
        self.tasks.iter().any(|task| task.paper.id == arxiv_id)
    }
    
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
    
    pub fn len(&self) -> usize {
        self.tasks.len()
    }
}

impl Default for DownloadQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// 下载相关的状态管理
#[derive(Debug)]
pub struct DownloadState {
    /// 下载队列
    pub queue: DownloadQueue,
    
    /// 活跃下载进度 (arxiv_id -> 进度信息)
    pub active_downloads: HashMap<String, DownloadProgress>,
    
    /// 已完成的下载 (arxiv_id -> 下载结果)
    pub completed_downloads: HashMap<String, DownloadResult>,
    
    /// 下载错误 (arxiv_id -> 错误信息)
    pub download_errors: HashMap<String, DownloadError>,
    
    /// 下载统计信息
    pub stats: DownloadStats,
    
    /// 下载设置
    pub settings: DownloadSettings,
    
    /// 暂停的下载任务
    pub paused_downloads: HashMap<String, DownloadTask>,
}

/// 下载进度信息
#[derive(Debug, Clone)]
pub struct DownloadProgress {
    /// ArXiv ID
    pub arxiv_id: String,
    
    /// 已下载字节数
    pub bytes_downloaded: u64,
    
    /// 总字节数（如果已知）
    pub total_bytes: Option<u64>,
    
    /// 下载速度（字节/秒）
    pub speed_bps: Option<u64>,
    
    /// 开始时间
    pub start_time: DateTime<Utc>,
    
    /// 最后更新时间
    pub last_update: DateTime<Utc>,
    
    /// 预计剩余时间（秒）
    pub eta_seconds: Option<u64>,
    
    /// 状态
    pub status: DownloadStatus,
}

/// 下载状态
#[derive(Debug, Clone, PartialEq)]
pub enum DownloadStatus {
    Queued,
    Downloading,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

/// 下载结果
#[derive(Debug, Clone)]
pub struct DownloadResult {
    /// ArXiv ID
    pub arxiv_id: String,
    
    /// 下载的文件路径
    pub file_path: PathBuf,
    
    /// 文件大小
    pub file_size: u64,
    
    /// 完成时间
    pub completed_at: DateTime<Utc>,
    
    /// 下载耗时（毫秒）
    pub download_time_ms: u64,
    
    /// 平均下载速度（字节/秒）
    pub average_speed_bps: u64,
    
    /// 论文信息
    pub paper: ArxivPaper,
}

/// 下载错误
#[derive(Debug, Clone)]
pub struct DownloadError {
    /// ArXiv ID
    pub arxiv_id: String,
    
    /// 错误消息
    pub error_message: String,
    
    /// 错误类型
    pub error_type: DownloadErrorType,
    
    /// 发生时间
    pub occurred_at: DateTime<Utc>,
    
    /// 重试次数
    pub retry_count: u32,
    
    /// 是否可以重试
    pub can_retry: bool,
}

/// 下载错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum DownloadErrorType {
    NetworkError,
    FileSystemError,
    InvalidUrl,
    Timeout,
    PermissionDenied,
    DiskFull,
    Cancelled,
    Unknown,
}

/// 下载统计信息
#[derive(Debug, Clone, Default)]
pub struct DownloadStats {
    /// 总下载数
    pub total_downloads: u64,
    
    /// 成功下载数
    pub successful_downloads: u64,
    
    /// 失败下载数
    pub failed_downloads: u64,
    
    /// 总下载字节数
    pub total_bytes_downloaded: u64,
    
    /// 平均下载速度（字节/秒）
    pub average_speed_bps: u64,
    
    /// 最快下载速度（字节/秒）
    pub max_speed_bps: u64,
    
    /// 总下载时间（毫秒）
    pub total_download_time_ms: u64,
    
    /// 今日下载数
    pub today_downloads: u64,
    
    /// 今日下载字节数
    pub today_bytes_downloaded: u64,
}

/// 下载设置
#[derive(Debug, Clone)]
pub struct DownloadSettings {
    /// 最大并发下载数
    pub max_concurrent_downloads: usize,
    
    /// 自动重试次数
    pub retry_attempts: u32,
    
    /// 超时时间（秒）
    pub timeout_seconds: u64,
    
    /// 下载目录
    pub download_directory: PathBuf,
    
    /// 文件命名模式
    pub naming_pattern: String,
    
    /// 是否自动开始下载
    pub auto_start_downloads: bool,
    
    /// 是否覆盖现有文件
    pub overwrite_existing: bool,
    
    /// 限制下载速度（字节/秒，None为不限制）
    pub speed_limit_bps: Option<u64>,
}

impl Default for DownloadState {
    fn default() -> Self {
        Self {
            queue: DownloadQueue::new(),
            active_downloads: HashMap::new(),
            completed_downloads: HashMap::new(),
            download_errors: HashMap::new(),
            stats: DownloadStats::default(),
            settings: DownloadSettings::default(),
            paused_downloads: HashMap::new(),
        }
    }
}

impl Default for DownloadSettings {
    fn default() -> Self {
        Self {
            max_concurrent_downloads: 3,
            retry_attempts: 3,
            timeout_seconds: 300,
            download_directory: PathBuf::from("downloads"),
            naming_pattern: "{title}_{arxiv_id}.pdf".to_string(),
            auto_start_downloads: true,
            overwrite_existing: false,
            speed_limit_bps: None,
        }
    }
}

impl DownloadState {
    /// 创建新的下载状态
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 添加下载任务
    pub fn add_download(&mut self, task: DownloadTask) {
        let arxiv_id = task.paper.id.clone();
        
        // 检查是否已经在队列中或正在下载
        if self.is_download_active(&arxiv_id) || self.queue.contains_task(&arxiv_id) {
            return;
        }
        
        self.queue.add_task(task);
        self.stats.total_downloads += 1;
    }
    
    /// 开始下载
    pub fn start_download(&mut self, arxiv_id: &str) {
        if let Some(task) = self.queue.next_task() {
            if task.paper.id == arxiv_id {
                let progress = DownloadProgress {
                    arxiv_id: arxiv_id.to_string(),
                    bytes_downloaded: 0,
                    total_bytes: None,
                    speed_bps: None,
                    start_time: Utc::now(),
                    last_update: Utc::now(),
                    eta_seconds: None,
                    status: DownloadStatus::Downloading,
                };
                
                self.active_downloads.insert(arxiv_id.to_string(), progress);
                self.download_errors.remove(arxiv_id);
            }
        }
    }
    
    /// 更新下载进度
    pub fn update_progress(&mut self, arxiv_id: &str, bytes_downloaded: u64, total_bytes: Option<u64>) {
        if let Some(progress) = self.active_downloads.get_mut(arxiv_id) {
            let now = Utc::now();
            let elapsed = (now - progress.last_update).num_milliseconds() as f64 / 1000.0;
            
            if elapsed > 0.0 {
                let bytes_diff = bytes_downloaded.saturating_sub(progress.bytes_downloaded) as f64;
                progress.speed_bps = Some((bytes_diff / elapsed) as u64);
            }
            
            progress.bytes_downloaded = bytes_downloaded;
            progress.total_bytes = total_bytes;
            progress.last_update = now;
            progress.status = DownloadStatus::Downloading;
            
            // 计算预计剩余时间
            if let (Some(total), Some(speed)) = (total_bytes, progress.speed_bps) {
                if speed > 0 {
                    let remaining_bytes = total.saturating_sub(bytes_downloaded);
                    progress.eta_seconds = Some(remaining_bytes / speed);
                }
            }
        }
    }
    
    /// 完成下载
    pub fn complete_download(&mut self, arxiv_id: &str, file_path: PathBuf, file_size: u64, paper: ArxivPaper) {
        if let Some(progress) = self.active_downloads.remove(arxiv_id) {
            let download_time_ms = (Utc::now() - progress.start_time).num_milliseconds() as u64;
            let average_speed_bps = if download_time_ms > 0 {
                (file_size * 1000) / download_time_ms
            } else {
                0
            };
            
            let result = DownloadResult {
                arxiv_id: arxiv_id.to_string(),
                file_path,
                file_size,
                completed_at: Utc::now(),
                download_time_ms,
                average_speed_bps,
                paper,
            };
            
            self.completed_downloads.insert(arxiv_id.to_string(), result);
            self.stats.successful_downloads += 1;
            self.stats.total_bytes_downloaded += file_size;
            self.stats.total_download_time_ms += download_time_ms;
            
            // 更新统计信息
            self.update_stats(average_speed_bps);
            
            // 移除错误记录
            self.download_errors.remove(arxiv_id);
        }
    }
    
    /// 下载失败
    pub fn fail_download(&mut self, arxiv_id: &str, error_message: String, error_type: DownloadErrorType, can_retry: bool) {
        self.active_downloads.remove(arxiv_id);
        
        let error = DownloadError {
            arxiv_id: arxiv_id.to_string(),
            error_message,
            error_type,
            occurred_at: Utc::now(),
            retry_count: self.get_retry_count(arxiv_id),
            can_retry,
        };
        
        self.download_errors.insert(arxiv_id.to_string(), error);
        self.stats.failed_downloads += 1;
    }
    
    /// 暂停下载
    pub fn pause_download(&mut self, arxiv_id: &str) {
        if let Some(mut progress) = self.active_downloads.remove(arxiv_id) {
            progress.status = DownloadStatus::Paused;
            // 这里需要从队列中获取任务并暂存
            // self.paused_downloads.insert(arxiv_id.to_string(), task);
        }
    }
    
    /// 恢复下载
    pub fn resume_download(&mut self, arxiv_id: &str) {
        if let Some(task) = self.paused_downloads.remove(arxiv_id) {
            self.queue.add_task(task);
        }
    }
    
    /// 取消下载
    pub fn cancel_download(&mut self, arxiv_id: &str) {
        self.active_downloads.remove(arxiv_id);
        self.paused_downloads.remove(arxiv_id);
        self.queue.remove_task(arxiv_id);
        
        let error = DownloadError {
            arxiv_id: arxiv_id.to_string(),
            error_message: "Download cancelled by user".to_string(),
            error_type: DownloadErrorType::Cancelled,
            occurred_at: Utc::now(),
            retry_count: 0,
            can_retry: false,
        };
        
        self.download_errors.insert(arxiv_id.to_string(), error);
    }
    
    /// 重试下载
    pub fn retry_download(&mut self, arxiv_id: &str) {
        if let Some(error) = self.download_errors.get_mut(arxiv_id) {
            if error.can_retry && error.retry_count < self.settings.retry_attempts {
                error.retry_count += 1;
                // 需要重新添加到队列
                // self.queue.add_task(task);
            }
        }
    }
    
    /// 清除已完成的下载
    pub fn clear_completed(&mut self) {
        self.completed_downloads.clear();
    }
    
    /// 清除错误记录
    pub fn clear_errors(&mut self) {
        self.download_errors.clear();
    }
    
    /// 检查下载是否活跃
    pub fn is_download_active(&self, arxiv_id: &str) -> bool {
        self.active_downloads.contains_key(arxiv_id)
    }
    
    /// 检查下载是否完成
    pub fn is_download_completed(&self, arxiv_id: &str) -> bool {
        self.completed_downloads.contains_key(arxiv_id)
    }
    
    /// 检查下载是否失败
    pub fn has_download_error(&self, arxiv_id: &str) -> bool {
        self.download_errors.contains_key(arxiv_id)
    }
    
    /// 获取下载进度百分比
    pub fn get_progress_percentage(&self, arxiv_id: &str) -> Option<f64> {
        self.active_downloads.get(arxiv_id).and_then(|progress| {
            progress.total_bytes.map(|total| {
                if total > 0 {
                    (progress.bytes_downloaded as f64 / total as f64) * 100.0
                } else {
                    0.0
                }
            })
        })
    }
    
    /// 获取总体下载进度
    pub fn get_overall_progress(&self) -> (usize, usize, usize) {
        let active = self.active_downloads.len();
        let completed = self.completed_downloads.len();
        let failed = self.download_errors.len();
        (active, completed, failed)
    }
    
    /// 获取下载成功率
    pub fn get_success_rate(&self) -> f64 {
        if self.stats.total_downloads == 0 {
            0.0
        } else {
            self.stats.successful_downloads as f64 / self.stats.total_downloads as f64
        }
    }
    
    /// 获取重试次数
    fn get_retry_count(&self, arxiv_id: &str) -> u32 {
        self.download_errors
            .get(arxiv_id)
            .map(|error| error.retry_count)
            .unwrap_or(0)
    }
    
    /// 更新统计信息
    fn update_stats(&mut self, speed_bps: u64) {
        if speed_bps > self.stats.max_speed_bps {
            self.stats.max_speed_bps = speed_bps;
        }
        
        // 计算平均速度
        if self.stats.successful_downloads > 0 {
            let total_speed = self.stats.average_speed_bps * (self.stats.successful_downloads - 1) + speed_bps;
            self.stats.average_speed_bps = total_speed / self.stats.successful_downloads;
        } else {
            self.stats.average_speed_bps = speed_bps;
        }
    }
    
    /// 获取队列中的任务数量
    pub fn queue_size(&self) -> usize {
        self.queue.len()
    }
    
    /// 获取活跃下载数量
    pub fn active_download_count(&self) -> usize {
        self.active_downloads.len()
    }
    
    /// 检查是否达到最大并发数
    pub fn is_at_max_concurrent(&self) -> bool {
        self.active_downloads.len() >= self.settings.max_concurrent_downloads
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ArxivPaper;
    
    #[test]
    fn test_download_state_creation() {
        let state = DownloadState::new();
        assert_eq!(state.active_downloads.len(), 0);
        assert_eq!(state.completed_downloads.len(), 0);
        assert_eq!(state.download_errors.len(), 0);
    }
    
    #[test]
    fn test_download_progress() {
        let mut state = DownloadState::new();
        let arxiv_id = "test123";
        
        // 模拟开始下载
        state.start_download(arxiv_id);
        
        // 更新进度
        state.update_progress(arxiv_id, 1024, Some(2048));
        
        if let Some(progress) = state.active_downloads.get(arxiv_id) {
            assert_eq!(progress.bytes_downloaded, 1024);
            assert_eq!(progress.total_bytes, Some(2048));
        }
        
        // 检查进度百分比
        let percentage = state.get_progress_percentage(arxiv_id);
        assert_eq!(percentage, Some(50.0));
    }
    
    #[test]
    fn test_download_completion() {
        let mut state = DownloadState::new();
        let arxiv_id = "test123";
        
        // 创建一个测试用的 ArxivPaper
        let paper = ArxivPaper {
            id: arxiv_id.to_string(),
            title: "Test Paper".to_string(),
            authors: vec!["Test Author".to_string()],
            abstract_text: "Test abstract".to_string(),
            categories: vec!["cs.AI".to_string()],
            published: "2023-01-01T00:00:00Z".to_string(),
            updated: "2023-01-01T00:00:00Z".to_string(),
            pdf_url: "https://arxiv.org/pdf/test.pdf".to_string(),
            entry_url: "https://arxiv.org/abs/test".to_string(),
            doi: None,
            journal_ref: None,
            comments: None,
        };
        
        state.start_download(arxiv_id);
        state.complete_download(arxiv_id, PathBuf::from("test.pdf"), 2048, paper);
        
        assert!(!state.is_download_active(arxiv_id));
        assert!(state.is_download_completed(arxiv_id));
        assert_eq!(state.stats.successful_downloads, 1);
        assert_eq!(state.stats.total_bytes_downloaded, 2048);
    }
    
    #[test]
    fn test_download_failure() {
        let mut state = DownloadState::new();
        let arxiv_id = "test123";
        
        state.start_download(arxiv_id);
        state.fail_download(
            arxiv_id, 
            "Network error".to_string(), 
            DownloadErrorType::NetworkError, 
            true
        );
        
        assert!(!state.is_download_active(arxiv_id));
        assert!(state.has_download_error(arxiv_id));
        assert_eq!(state.stats.failed_downloads, 1);
    }
    
    #[test]
    fn test_success_rate() {
        let mut state = DownloadState::new();
        
        // 没有下载时成功率为0
        assert_eq!(state.get_success_rate(), 0.0);
        
        // 一次成功下载
        state.stats.total_downloads = 1;
        state.stats.successful_downloads = 1;
        assert_eq!(state.get_success_rate(), 1.0);
        
        // 一次成功，一次失败
        state.stats.total_downloads = 2;
        state.stats.failed_downloads = 1;
        assert_eq!(state.get_success_rate(), 0.5);
    }
    
    #[test]
    fn test_concurrent_limit() {
        let mut state = DownloadState::new();
        state.settings.max_concurrent_downloads = 2;
        
        assert!(!state.is_at_max_concurrent());
        
        state.start_download("test1");
        state.start_download("test2");
        
        assert!(state.is_at_max_concurrent());
    }
}

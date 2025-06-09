use anyhow::Result;
use reqwest::Client;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio::task::JoinHandle;
use tokio::time::Duration;

use crate::config::AppConfig;
use super::download_task::{DownloadTask, DownloadEvent, DownloadProgress};

/// 下载管理器
pub struct DownloadManager {
    client: Arc<Client>,
    tasks: Arc<RwLock<HashMap<String, Arc<DownloadTask>>>>,
    active_downloads: Arc<RwLock<HashMap<String, JoinHandle<Result<()>>>>>,
    event_sender: mpsc::UnboundedSender<DownloadEvent>,
    event_receiver: Arc<RwLock<Option<mpsc::UnboundedReceiver<DownloadEvent>>>>,
    config: AppConfig,
}

impl DownloadManager {
    /// 创建新的下载管理器
    pub fn new(config: AppConfig) -> Self {
        let client = Arc::new(
            Client::builder()
                .timeout(Duration::from_secs(config.download.timeout_seconds))
                .build()
                .expect("Failed to create HTTP client")
        );

        let (event_sender, event_receiver) = mpsc::unbounded_channel();

        Self {
            client,
            tasks: Arc::new(RwLock::new(HashMap::new())),
            active_downloads: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
            event_receiver: Arc::new(RwLock::new(Some(event_receiver))),
            config,
        }
    }

    /// 添加下载任务
    pub async fn add_download(
        &self,
        paper_id: String,
        paper_title: String,
        url: String,
        file_path: PathBuf,
    ) -> Result<String> {
        let task = Arc::new(DownloadTask::new(paper_id, paper_title, url, file_path));
        let task_id = task.id.clone();

        // 添加到任务列表
        self.tasks.write().await.insert(task_id.clone(), task.clone());

        // 检查并发限制
        let active_count = self.active_downloads.read().await.len();
        if active_count < self.config.download.max_concurrent_downloads {
            self.start_download(task).await?;
        }

        Ok(task_id)
    }

    /// 开始下载任务
    async fn start_download(&self, task: Arc<DownloadTask>) -> Result<()> {
        let task_id = task.id.clone();
        let client = Arc::clone(&self.client);
        let event_sender = self.event_sender.clone();
        let max_retries = self.config.download.max_retries;
        let timeout_duration = Duration::from_secs(self.config.download.timeout_seconds);

        let handle = tokio::spawn(async move {
            task.execute(client, event_sender, max_retries, timeout_duration).await
        });

        self.active_downloads.write().await.insert(task_id, handle);
        Ok(())
    }

    /// 暂停下载
    pub async fn pause_download(&self, task_id: &str) -> Result<()> {
        if let Some(task) = self.tasks.read().await.get(task_id) {
            task.pause().await;
        }
        Ok(())
    }

    /// 恢复下载
    pub async fn resume_download(&self, task_id: &str) -> Result<()> {
        if let Some(task) = self.tasks.read().await.get(task_id) {
            task.resume().await;
        }
        Ok(())
    }

    /// 取消下载
    pub async fn cancel_download(&self, task_id: &str) -> Result<()> {
        if let Some(task) = self.tasks.read().await.get(task_id) {
            task.cancel().await;
        }

        // 从活动下载中移除
        if let Some(handle) = self.active_downloads.write().await.remove(task_id) {
            handle.abort();
        }

        // 启动队列中的下一个任务
        self.start_next_queued_download().await?;

        Ok(())
    }

    /// 移除下载任务
    pub async fn remove_download(&self, task_id: &str) -> Result<()> {
        // 先取消下载
        self.cancel_download(task_id).await?;
        
        // 从任务列表中移除
        self.tasks.write().await.remove(task_id);

        Ok(())
    }

    /// 获取下载任务
    pub async fn get_download(&self, task_id: &str) -> Option<Arc<DownloadTask>> {
        self.tasks.read().await.get(task_id).cloned()
    }

    /// 获取所有下载任务
    pub async fn get_all_downloads(&self) -> Vec<Arc<DownloadTask>> {
        self.tasks.read().await.values().cloned().collect()
    }

    /// 获取活动下载任务
    pub async fn get_active_downloads(&self) -> Vec<Arc<DownloadTask>> {
        let active_ids: Vec<String> = self.active_downloads.read().await.keys().cloned().collect();
        let tasks = self.tasks.read().await;
        
        active_ids
            .into_iter()
            .filter_map(|id| tasks.get(&id).cloned())
            .collect()
    }

    /// 获取队列中的下载任务
    pub async fn get_queued_downloads(&self) -> Vec<Arc<DownloadTask>> {
        let active_ids: std::collections::HashSet<String> = 
            self.active_downloads.read().await.keys().cloned().collect();
        let tasks = self.tasks.read().await;

        let mut queued = Vec::new();
        for (id, task) in tasks.iter() {
            if !active_ids.contains(id) && task.get_status().await == crate::database::models::DownloadStatus::Pending {
                queued.push(task.clone());
            }
        }

        queued
    }

    /// 启动队列中的下一个下载任务
    async fn start_next_queued_download(&self) -> Result<()> {
        let active_count = self.active_downloads.read().await.len();
        if active_count >= self.config.download.max_concurrent_downloads {
            return Ok(());
        }

        let queued_downloads = self.get_queued_downloads().await;
        if let Some(next_task) = queued_downloads.into_iter().next() {
            self.start_download(next_task).await?;
        }

        Ok(())
    }

    /// 获取事件接收器
    pub async fn take_event_receiver(&self) -> Option<mpsc::UnboundedReceiver<DownloadEvent>> {
        self.event_receiver.write().await.take()
    }

    /// 处理下载事件
    pub async fn handle_events(&self) {
        if let Some(mut receiver) = self.take_event_receiver().await {
            tokio::spawn(async move {
                while let Some(event) = receiver.recv().await {
                    match event {
                        DownloadEvent::Completed(task_id) | 
                        DownloadEvent::Failed(task_id, _) | 
                        DownloadEvent::Cancelled(task_id) => {
                            // 任务完成，从活动列表中移除
                            // 注意：这里需要访问 self，但在 spawn 中无法直接访问
                            // 实际实现中应该通过消息传递来处理
                        }
                        _ => {}
                    }
                }
            });
        }
    }

    /// 获取下载统计信息
    pub async fn get_statistics(&self) -> DownloadStatistics {
        let tasks = self.tasks.read().await;
        let mut stats = DownloadStatistics::default();

        for task in tasks.values() {
            stats.total_tasks += 1;
            match task.get_status().await {
                crate::database::models::DownloadStatus::Pending => stats.pending_tasks += 1,
                crate::database::models::DownloadStatus::Downloading => stats.active_tasks += 1,
                crate::database::models::DownloadStatus::Paused => stats.paused_tasks += 1,
                crate::database::models::DownloadStatus::Completed => stats.completed_tasks += 1,
                crate::database::models::DownloadStatus::Failed => stats.failed_tasks += 1,
                crate::database::models::DownloadStatus::Cancelled => stats.cancelled_tasks += 1,
            }

            stats.total_downloaded_bytes += task.get_downloaded_size().await;
            if let Some(total) = task.get_total_size().await {
                stats.total_size_bytes += total;
            }
        }

        // 计算总体下载速度
        let active_tasks = self.get_active_downloads().await;
        for task in active_tasks {
            stats.total_speed += task.get_speed().await;
        }

        stats
    }

    /// 暂停所有下载
    pub async fn pause_all(&self) -> Result<()> {
        let tasks = self.get_active_downloads().await;
        for task in tasks {
            task.pause().await;
        }
        Ok(())
    }

    /// 恢复所有下载
    pub async fn resume_all(&self) -> Result<()> {
        let tasks = self.tasks.read().await;
        for task in tasks.values() {
            if task.get_status().await == crate::database::models::DownloadStatus::Paused {
                task.resume().await;
            }
        }
        Ok(())
    }

    /// 取消所有下载
    pub async fn cancel_all(&self) -> Result<()> {
        let task_ids: Vec<String> = self.tasks.read().await.keys().cloned().collect();
        for task_id in task_ids {
            self.cancel_download(&task_id).await?;
        }
        Ok(())
    }

    /// 清理已完成的下载任务
    pub async fn cleanup_completed(&self) -> Result<()> {
        let mut to_remove = Vec::new();
        let tasks = self.tasks.read().await;
        
        for (id, task) in tasks.iter() {
            let status = task.get_status().await;
            if status == crate::database::models::DownloadStatus::Completed ||
               status == crate::database::models::DownloadStatus::Failed ||
               status == crate::database::models::DownloadStatus::Cancelled {
                to_remove.push(id.clone());
            }
        }
        
        drop(tasks);

        for task_id in to_remove {
            self.remove_download(&task_id).await?;
        }

        Ok(())
    }
}

/// 下载统计信息
#[derive(Debug, Default, Clone)]
pub struct DownloadStatistics {
    pub total_tasks: usize,
    pub pending_tasks: usize,
    pub active_tasks: usize,
    pub paused_tasks: usize,
    pub completed_tasks: usize,
    pub failed_tasks: usize,
    pub cancelled_tasks: usize,
    pub total_downloaded_bytes: u64,
    pub total_size_bytes: u64,
    pub total_speed: u64,
}

impl DownloadStatistics {
    /// 获取完成率
    pub fn completion_rate(&self) -> f32 {
        if self.total_tasks == 0 {
            0.0
        } else {
            self.completed_tasks as f32 / self.total_tasks as f32
        }
    }

    /// 获取下载进度
    pub fn download_progress(&self) -> f32 {
        if self.total_size_bytes == 0 {
            0.0
        } else {
            (self.total_downloaded_bytes as f32 / self.total_size_bytes as f32) * 100.0
        }
    }

    /// 格式化下载速度
    pub fn formatted_speed(&self) -> String {
        format_bytes_per_second(self.total_speed)
    }

    /// 格式化已下载大小
    pub fn formatted_downloaded(&self) -> String {
        format_bytes(self.total_downloaded_bytes)
    }

    /// 格式化总大小
    pub fn formatted_total_size(&self) -> String {
        format_bytes(self.total_size_bytes)
    }
}

/// 格式化字节数
fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// 格式化每秒字节数
fn format_bytes_per_second(bytes_per_sec: u64) -> String {
    format!("{}/s", format_bytes(bytes_per_sec))
}

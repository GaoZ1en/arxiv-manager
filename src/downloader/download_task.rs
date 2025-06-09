use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::time::{sleep, Duration, Instant};
use uuid::Uuid;

use crate::database::models::DownloadStatus;

/// 下载任务
#[derive(Debug, Clone)]
pub struct DownloadTask {
    pub id: String,
    pub paper_id: String,
    pub paper_title: String,
    pub url: String,
    pub file_path: PathBuf,
    pub status: Arc<RwLock<DownloadStatus>>,
    pub progress: Arc<RwLock<f32>>,
    pub speed: Arc<RwLock<u64>>,
    pub total_size: Arc<RwLock<Option<u64>>>,
    pub downloaded_size: Arc<RwLock<u64>>,
    pub error_message: Arc<RwLock<Option<String>>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Arc<RwLock<DateTime<Utc>>>,
    pub cancel_token: Arc<RwLock<bool>>,
    pub pause_token: Arc<RwLock<bool>>,
}

/// 下载进度信息
#[derive(Debug, Clone)]
pub struct DownloadProgress {
    pub task_id: String,
    pub progress: f32,
    pub speed: u64,
    pub downloaded_size: u64,
    pub total_size: Option<u64>,
    pub eta: Option<Duration>,
}

/// 下载事件
#[derive(Debug, Clone)]
pub enum DownloadEvent {
    Started(String),
    Progress(DownloadProgress),
    Completed(String),
    Failed(String, String), // task_id, error
    Cancelled(String),
    Paused(String),
    Resumed(String),
}

impl DownloadTask {
    /// 创建新的下载任务
    pub fn new(paper_id: String, paper_title: String, url: String, file_path: PathBuf) -> Self {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        Self {
            id,
            paper_id,
            paper_title,
            url,
            file_path,
            status: Arc::new(RwLock::new(DownloadStatus::Pending)),
            progress: Arc::new(RwLock::new(0.0)),
            speed: Arc::new(RwLock::new(0)),
            total_size: Arc::new(RwLock::new(None)),
            downloaded_size: Arc::new(RwLock::new(0)),
            error_message: Arc::new(RwLock::new(None)),
            created_at: now,
            updated_at: Arc::new(RwLock::new(now)),
            cancel_token: Arc::new(RwLock::new(false)),
            pause_token: Arc::new(RwLock::new(false)),
        }
    }

    /// 执行下载
    pub async fn execute(
        &self,
        client: Arc<Client>,
        progress_sender: mpsc::UnboundedSender<DownloadEvent>,
        max_retries: usize,
        timeout_duration: Duration,
    ) -> Result<()> {
        let mut retries = 0;

        while retries <= max_retries {
            match self.try_download(&client, &progress_sender, timeout_duration).await {
                Ok(_) => {
                    *self.status.write().await = DownloadStatus::Completed;
                    *self.progress.write().await = 100.0;
                    *self.updated_at.write().await = Utc::now();
                    
                    let _ = progress_sender.send(DownloadEvent::Completed(self.id.clone()));
                    return Ok(());
                }
                Err(e) => {
                    retries += 1;
                    if retries > max_retries {
                        *self.status.write().await = DownloadStatus::Failed;
                        *self.error_message.write().await = Some(e.to_string());
                        *self.updated_at.write().await = Utc::now();
                        
                        let _ = progress_sender.send(DownloadEvent::Failed(
                            self.id.clone(),
                            e.to_string(),
                        ));
                        return Err(e);
                    }
                    
                    // 等待重试间隔
                    sleep(Duration::from_secs(2_u64.pow(retries as u32))).await;
                }
            }
        }

        Ok(())
    }

    /// 尝试下载
    async fn try_download(
        &self,
        client: &Client,
        progress_sender: &mpsc::UnboundedSender<DownloadEvent>,
        timeout_duration: Duration,
    ) -> Result<()> {
        // 检查取消状态
        if *self.cancel_token.read().await {
            *self.status.write().await = DownloadStatus::Cancelled;
            let _ = progress_sender.send(DownloadEvent::Cancelled(self.id.clone()));
            return Err(anyhow!("Download cancelled"));
        }

        *self.status.write().await = DownloadStatus::Downloading;
        let _ = progress_sender.send(DownloadEvent::Started(self.id.clone()));

        // 确保目录存在
        if let Some(parent) = self.file_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // 检查是否支持断点续传
        let mut resume_from = 0u64;
        if self.file_path.exists() {
            let metadata = tokio::fs::metadata(&self.file_path).await?;
            resume_from = metadata.len();
            *self.downloaded_size.write().await = resume_from;
        }

        // 构建请求
        let mut request = client.get(&self.url).timeout(timeout_duration);
        
        if resume_from > 0 {
            request = request.header("Range", format!("bytes={}-", resume_from));
        }

        let response = request.send().await?;

        if !response.status().is_success() && response.status().as_u16() != 206 {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        // 获取文件总大小
        let content_length = response.content_length();
        if let Some(total) = content_length {
            *self.total_size.write().await = Some(total + resume_from);
        }

        // 打开文件进行写入
        let mut file = if resume_from > 0 {
            File::options()
                .create(true)
                .append(true)
                .open(&self.file_path)
                .await?
        } else {
            File::create(&self.file_path).await?
        };

        // 下载数据
        let mut downloaded = resume_from;
        let mut last_update = Instant::now();
        let mut last_downloaded = downloaded;
        
        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            // 检查暂停状态
            while *self.pause_token.read().await {
                *self.status.write().await = DownloadStatus::Paused;
                let _ = progress_sender.send(DownloadEvent::Paused(self.id.clone()));
                
                sleep(Duration::from_millis(100)).await;
                
                if *self.cancel_token.read().await {
                    *self.status.write().await = DownloadStatus::Cancelled;
                    let _ = progress_sender.send(DownloadEvent::Cancelled(self.id.clone()));
                    return Err(anyhow!("Download cancelled"));
                }
            }

            // 检查取消状态
            if *self.cancel_token.read().await {
                *self.status.write().await = DownloadStatus::Cancelled;
                let _ = progress_sender.send(DownloadEvent::Cancelled(self.id.clone()));
                return Err(anyhow!("Download cancelled"));
            }

            let chunk = chunk?;
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;

            // 更新进度
            *self.downloaded_size.write().await = downloaded;
            *self.updated_at.write().await = Utc::now();

            // 计算进度和速度
            let now = Instant::now();
            if now.duration_since(last_update) >= Duration::from_millis(500) {
                let elapsed = now.duration_since(last_update);
                let bytes_diff = downloaded - last_downloaded;
                let speed = if elapsed.as_secs_f64() > 0.0 {
                    (bytes_diff as f64 / elapsed.as_secs_f64()) as u64
                } else {
                    0
                };

                *self.speed.write().await = speed;

                let progress = if let Some(total) = *self.total_size.read().await {
                    if total > 0 {
                        (downloaded as f32 / total as f32) * 100.0
                    } else {
                        0.0
                    }
                } else {
                    0.0
                };

                *self.progress.write().await = progress;

                // 发送进度事件
                let eta = if speed > 0 && progress < 100.0 {
                    if let Some(total) = *self.total_size.read().await {
                        let remaining = total - downloaded;
                        Some(Duration::from_secs(remaining / speed))
                    } else {
                        None
                    }
                } else {
                    None
                };

                let progress_info = DownloadProgress {
                    task_id: self.id.clone(),
                    progress,
                    speed,
                    downloaded_size: downloaded,
                    total_size: *self.total_size.read().await,
                    eta,
                };

                let _ = progress_sender.send(DownloadEvent::Progress(progress_info));

                last_update = now;
                last_downloaded = downloaded;
            }
        }

        file.flush().await?;
        Ok(())
    }

    /// 暂停下载
    pub async fn pause(&self) {
        *self.pause_token.write().await = true;
    }

    /// 恢复下载
    pub async fn resume(&self) {
        *self.pause_token.write().await = false;
        if *self.status.read().await == DownloadStatus::Paused {
            *self.status.write().await = DownloadStatus::Downloading;
        }
    }

    /// 取消下载
    pub async fn cancel(&self) {
        *self.cancel_token.write().await = true;
    }

    /// 获取当前状态
    pub async fn get_status(&self) -> DownloadStatus {
        *self.status.read().await
    }

    /// 获取当前进度
    pub async fn get_progress(&self) -> f32 {
        *self.progress.read().await
    }

    /// 获取下载速度
    pub async fn get_speed(&self) -> u64 {
        *self.speed.read().await
    }

    /// 获取已下载大小
    pub async fn get_downloaded_size(&self) -> u64 {
        *self.downloaded_size.read().await
    }

    /// 获取总大小
    pub async fn get_total_size(&self) -> Option<u64> {
        *self.total_size.read().await
    }

    /// 是否已完成
    pub async fn is_completed(&self) -> bool {
        *self.status.read().await == DownloadStatus::Completed
    }

    /// 是否失败
    pub async fn is_failed(&self) -> bool {
        *self.status.read().await == DownloadStatus::Failed
    }

    /// 是否被取消
    pub async fn is_cancelled(&self) -> bool {
        *self.status.read().await == DownloadStatus::Cancelled
    }

}

// 下载管理器实现模块

use crate::core::models::ArxivPaper;
use crate::database::{Database, DownloadStatus};
use crate::utils::{ArxivError, Result};
use super::types::{DownloadTask, DownloadEvent};
use reqwest::Client;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::{Semaphore, mpsc, Mutex};

/// 下载管理器
#[derive(Debug)]
pub struct DownloadManager {
    client: Client,
    semaphore: Arc<Semaphore>,
    database: Arc<Mutex<Database>>,
    event_tx: mpsc::UnboundedSender<DownloadEvent>,
    max_retries: u32,
    timeout_seconds: u64,
}

impl DownloadManager {
    /// 创建新的下载管理器
    pub fn new(
        max_concurrent: usize,
        database: Arc<Mutex<Database>>,
        event_tx: mpsc::UnboundedSender<DownloadEvent>,
        max_retries: u32,
        timeout_seconds: u64,
    ) -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(timeout_seconds))
                .build()
                .expect("Failed to create HTTP client"),
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            database,
            event_tx,
            max_retries,
            timeout_seconds,
        }
    }
    
    /// 下载论文
    pub async fn download_paper(&self, task: DownloadTask) -> Result<PathBuf> {
        let _permit = self.semaphore.acquire().await
            .map_err(|_| ArxivError::Download("Failed to acquire semaphore".to_string()))?;
        
        let arxiv_id = task.paper.id.clone();
        
        // Notify start
        let _ = self.event_tx.send(DownloadEvent::Started { 
            arxiv_id: arxiv_id.clone() 
        });
        
        // Update database status
        {
            let db = self.database.lock().await;
            db.update_download_status(&arxiv_id, DownloadStatus::Downloading, None)?;
        }
        
        let result = self.download_with_retry(&task).await;
        
        match &result {
            Ok(path) => {
                // Update database on success
                {
                    let db = self.database.lock().await;
                    db.update_download_status(
                        &arxiv_id, 
                        DownloadStatus::Completed, 
                        Some(path.to_string_lossy().as_ref())
                    )?;
                }
                
                let _ = self.event_tx.send(DownloadEvent::Completed {
                    arxiv_id,
                    file_path: path.clone(),
                });
            }
            Err(e) => {
                // Update database on failure
                {
                    let db = self.database.lock().await;
                    db.update_download_status(&arxiv_id, DownloadStatus::Failed(e.to_string()), None)?;
                }
                
                let _ = self.event_tx.send(DownloadEvent::Failed {
                    arxiv_id,
                    error: e.to_string(),
                });
            }
        }
        
        result
    }
    
    /// 带重试的下载
    async fn download_with_retry(&self, task: &DownloadTask) -> Result<PathBuf> {
        let mut last_error = None;
        
        for attempt in 0..=self.max_retries {
            match self.download_single(&task).await {
                Ok(path) => return Ok(path),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.max_retries {
                        // Exponential backoff
                        let delay = std::time::Duration::from_secs(2_u64.pow(attempt));
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }
        
        Err(last_error.unwrap_or_else(|| 
            ArxivError::Download("Unknown download error".to_string())
        ))
    }
    
    /// 单次下载尝试
    async fn download_single(&self, task: &DownloadTask) -> Result<PathBuf> {
        let url = &task.paper.pdf_url;
        let output_path = &task.output_path;
        
        // Ensure directory exists
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        // Start download
        let response = self.client.get(url).send().await?;
        
        if !response.status().is_success() {
            return Err(ArxivError::Download(format!(
                "HTTP error {}: {}", 
                response.status(),
                response.status().canonical_reason().unwrap_or("Unknown")
            )));
        }
        
        let total_bytes = response.content_length();
        let mut stream = response.bytes_stream();
        let mut file = fs::File::create(output_path).await?;
        let mut bytes_downloaded = 0u64;
        
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| ArxivError::Network(e))?;
            file.write_all(&chunk).await?;
            
            bytes_downloaded += chunk.len() as u64;
            
            // Send progress update
            let _ = self.event_tx.send(DownloadEvent::Progress {
                arxiv_id: task.paper.id.clone(),
                bytes_downloaded,
                total_bytes,
            });
        }
        
        file.flush().await?;
        
        Ok(output_path.clone())
    }
    
    /// 生成文件路径
    pub fn generate_file_path(&self, paper: &ArxivPaper, base_dir: &Path, pattern: &str) -> PathBuf {
        super::utils::generate_file_path(paper, base_dir, pattern)
    }
}

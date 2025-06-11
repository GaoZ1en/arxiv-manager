use crate::core::ArxivPaper;
use crate::database::{Database, DownloadStatus};
use crate::utils::{ArxivError, Result};
use reqwest::Client;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::{Semaphore, mpsc, Mutex};

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

#[derive(Debug, Clone)]
pub enum DownloadEvent {
    Started { arxiv_id: String },
    Progress { arxiv_id: String, bytes_downloaded: u64, total_bytes: Option<u64> },
    Completed { arxiv_id: String, file_path: PathBuf },
    Failed { arxiv_id: String, error: String },
}

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
    
    pub fn generate_file_path(&self, paper: &ArxivPaper, base_dir: &Path, pattern: &str) -> PathBuf {
        // Extract year from published date string (assuming format like "2023-01-15T12:00:00Z")
        let year = paper.published.split('-').next().unwrap_or("unknown").to_string();
        let category = paper.categories.first()
            .map(|c| c.replace(".", "_"))
            .unwrap_or_else(|| "unknown".to_string());
        
        // Sanitize title for filename
        let title = self.sanitize_filename(&paper.title);
        let filename = format!("{}.pdf", paper.id);
        
        let path_str = pattern
            .replace("{year}", &year)
            .replace("{category}", &category)
            .replace("{title}", &title)
            .replace("{id}", &paper.id);
        
        let mut path = base_dir.to_path_buf();
        
        // Split path by '/' and build the directory structure
        for component in path_str.split('/') {
            if !component.is_empty() {
                path.push(component);
            }
        }
        
        // Ensure we end with the filename
        if !path.to_string_lossy().ends_with(".pdf") {
            path.push(filename);
        }
        
        path
    }
    
    fn sanitize_filename(&self, filename: &str) -> String {
        filename
            .chars()
            .map(|c| match c {
                '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
                c if c.is_control() => '_',
                c => c,
            })
            .collect::<String>()
            .trim()
            .to_string()
    }
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
        // Sort by priority (higher priority first)
        self.tasks.sort_by(|a, b| b.priority.cmp(&a.priority));
    }
    
    pub fn remove_task(&mut self, arxiv_id: &str) -> Option<DownloadTask> {
        if let Some(index) = self.tasks.iter().position(|t| t.paper.id == arxiv_id) {
            Some(self.tasks.remove(index))
        } else {
            None
        }
    }
    
    pub fn next_task(&mut self) -> Option<DownloadTask> {
        self.tasks.pop()
    }
    
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
    
    pub fn len(&self) -> usize {
        self.tasks.len()
    }
}

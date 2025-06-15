// 下载消息处理器
// 处理所有与下载相关的消息

use iced::Task;
use std::path::PathBuf;

use crate::core::{ArxivManager, ArxivPaper, DownloadItem, DownloadStatus};
use crate::core::messages::Message;

/// 异步下载PDF文件
async fn download_pdf_file(paper_id: String, pdf_url: String) -> Result<(String, PathBuf), (String, String)> {
    use std::fs;
    use std::io::Write;
    
    // 创建下载目录
    let downloads_dir = std::env::current_dir()
        .map_err(|e| (paper_id.clone(), format!("Failed to get current directory: {}", e)))?
        .join("downloads");
    
    if !downloads_dir.exists() {
        fs::create_dir_all(&downloads_dir)
            .map_err(|e| (paper_id.clone(), format!("Failed to create downloads directory: {}", e)))?;
    }
    
    // 构建文件路径
    let file_name = format!("{}.pdf", paper_id);
    let file_path = downloads_dir.join(&file_name);
    
    // 如果文件已存在，直接返回
    if file_path.exists() {
        return Ok((paper_id, file_path));
    }
    
    // 下载文件
    println!("Downloading PDF from: {}", pdf_url);
    
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| (paper_id.clone(), format!("Failed to create HTTP client: {}", e)))?;
    
    let response = client.get(&pdf_url).send().await
        .map_err(|e| (paper_id.clone(), format!("Failed to download PDF: {}", e)))?;
    
    println!("HTTP response status: {}", response.status());
    
    if !response.status().is_success() {
        return Err((paper_id, format!("HTTP error: {} - {}", response.status(), response.status().canonical_reason().unwrap_or("Unknown error"))));
    }
    
    let content_length = response.content_length();
    println!("Content length: {:?}", content_length);
    
    let content = response.bytes().await
        .map_err(|e| (paper_id.clone(), format!("Failed to read response: {}", e)))?;
    
    println!("Downloaded {} bytes", content.len());
    
    // 检查是否为有效的PDF文件
    if content.len() < 4 || !content.starts_with(b"%PDF") {
        return Err((paper_id, format!("Downloaded content is not a valid PDF file (size: {} bytes)", content.len())));
    }
    
    // 保存文件
    let mut file = fs::File::create(&file_path)
        .map_err(|e| (paper_id.clone(), format!("Failed to create file: {}", e)))?;
    
    file.write_all(&content)
        .map_err(|e| (paper_id.clone(), format!("Failed to write file: {}", e)))?;
    
    println!("PDF downloaded successfully: {:?}", file_path);
    Ok((paper_id, file_path))
}

pub trait DownloadHandler {
    fn handle_download_paper(&mut self, paper: ArxivPaper) -> Task<Message>;
    fn handle_download_cancel(&mut self, paper_id: String) -> Task<Message>;
    fn handle_download_retry(&mut self, paper_id: String) -> Task<Message>;
    fn handle_download_progress(&mut self, paper_id: String, progress: f32) -> Task<Message>;
    fn handle_download_completed(&mut self, paper_id: String, file_path: String) -> Task<Message>;
    fn handle_download_failed(&mut self, paper_id: String, error: String) -> Task<Message>;
    fn handle_download_clear_completed(&mut self) -> Task<Message>;
}

impl DownloadHandler for ArxivManager {
    fn handle_download_paper(&mut self, paper: ArxivPaper) -> Task<Message> {
        // 检查是否已经在下载队列中
        if self.downloads.iter().any(|d| d.paper_id == paper.id) {
            return Task::none();
        }
        
        // 添加到下载队列
        let download_item = DownloadItem {
            paper_id: paper.id.clone(),
            title: paper.title.clone(),
            progress: 0.0,
            status: DownloadStatus::Pending,
            file_path: None,
        };
        
        self.downloads.push(download_item);
        
        // 启动实际的下载任务
        let paper_id = paper.id.clone();
        let pdf_url = if paper.pdf_url.starts_with("http") {
            paper.pdf_url.clone()
        } else {
            // 构建正确的arXiv PDF URL
            if paper.id.contains("v") {
                // 如果ID已经包含版本号，直接使用
                format!("https://arxiv.org/pdf/{}.pdf", paper.id)
            } else {
                // 如果没有版本号，添加默认版本
                format!("https://arxiv.org/pdf/{}v1.pdf", paper.id)
            }
        };
        
        println!("Starting download for paper: {} from {}", paper.title, pdf_url);
        
        // 返回异步下载任务
        Task::perform(
            async move {
                download_pdf_file(paper_id, pdf_url).await
            },
            |result| match result {
                Ok((paper_id, file_path)) => Message::DownloadCompleted { paper_id, file_path },
                Err((paper_id, error)) => Message::DownloadFailed { paper_id, error },
            }
        )
    }

    #[allow(dead_code)]
    fn handle_download_cancel(&mut self, paper_id: String) -> Task<Message> {
        if let Some(download) = self.downloads.iter_mut().find(|d| d.paper_id == paper_id) {
            match download.status {
                DownloadStatus::Pending | DownloadStatus::Downloading => {
                    download.status = DownloadStatus::Failed("Cancelled by user".to_string());
                    println!("Download cancelled for paper: {}", download.paper_id);
                }
                _ => {}
            }
        }
        Task::none()
    }

    #[allow(dead_code)]
    fn handle_download_retry(&mut self, paper_id: String) -> Task<Message> {
        if let Some(download) = self.downloads.iter_mut().find(|d| d.paper_id == paper_id.clone()) {
            download.status = DownloadStatus::Pending;
            download.progress = 0.0;
            download.file_path = None;
            
            // 找到对应的论文
            if let Some(paper) = self.saved_papers.iter().find(|p| p.id == paper_id).cloned() {
                println!("Retrying download for paper: {}", paper_id);
                return self.handle_download_paper(paper);
            }
        }
        Task::none()
    }

    fn handle_download_progress(&mut self, paper_id: String, progress: f32) -> Task<Message> {
        if let Some(download) = self.downloads.iter_mut().find(|d| d.paper_id == paper_id) {
            download.progress = progress.clamp(0.0, 1.0);
            download.status = DownloadStatus::Downloading;
        }
        Task::none()
    }

    fn handle_download_completed(&mut self, paper_id: String, file_path: String) -> Task<Message> {
        // 更新下载状态
        if let Some(download) = self.downloads.iter_mut().find(|d| d.paper_id == paper_id) {
            download.progress = 1.0;
            download.status = DownloadStatus::Completed;
            download.file_path = Some(file_path.clone().into());
        }
        
        // 更新论文的本地文件路径
        if let Some(paper) = self.saved_papers.iter_mut().find(|p| p.id == paper_id) {
            paper.local_file_path = Some(file_path.clone());
        }
        
        println!("Download completed: {}", file_path);
        Task::none()
    }

    fn handle_download_failed(&mut self, paper_id: String, error: String) -> Task<Message> {
        if let Some(download) = self.downloads.iter_mut().find(|d| d.paper_id == paper_id) {
            download.status = DownloadStatus::Failed(error);
        }
        Task::none()
    }

    #[allow(dead_code)]
    fn handle_download_clear_completed(&mut self) -> Task<Message> {
        self.downloads.retain(|d| !matches!(d.status, DownloadStatus::Completed));
        Task::none()
    }
}

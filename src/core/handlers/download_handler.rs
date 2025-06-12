// 下载消息处理器
// 处理所有与下载相关的消息

use iced::Task;

use crate::core::{ArxivManager, ArxivPaper, DownloadItem, DownloadStatus};
use crate::core::messages::Message;

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
        
        // TODO: 启动实际的下载任务
        // 这里应该调用下载服务
        println!("Starting download for paper: {}", paper.title);
        
        Task::none()
    }

    #[allow(dead_code)]
    fn handle_download_cancel(&mut self, paper_id: String) -> Task<Message> {
        if let Some(download) = self.downloads.iter_mut().find(|d| d.paper_id == paper_id) {
            match download.status {
                DownloadStatus::Pending | DownloadStatus::Downloading => {
                    download.status = DownloadStatus::Failed("Cancelled by user".to_string());
                    // TODO: 实际取消下载任务
                }
                _ => {}
            }
        }
        Task::none()
    }

    #[allow(dead_code)]
    fn handle_download_retry(&mut self, paper_id: String) -> Task<Message> {
        if let Some(download) = self.downloads.iter_mut().find(|d| d.paper_id == paper_id) {
            download.status = DownloadStatus::Pending;
            download.progress = 0.0;
            download.file_path = None;
            
            // TODO: 重新启动下载任务
            println!("Retrying download for paper: {}", paper_id);
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
        if let Some(download) = self.downloads.iter_mut().find(|d| d.paper_id == paper_id) {
            download.progress = 1.0;
            download.status = DownloadStatus::Completed;
            download.file_path = Some(file_path.into());
        }
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

use iced::widget::{container, text, column, scrollable, progress_bar};
use iced::{Element, Length};
use crate::app::{AppMessage, AppState};

pub fn downloads_view(state: &AppState) -> Element<AppMessage> {
    let title = text("下载管理")
        .size(24);
    
    let queue_info = text(format!("队列中: {} 个任务", state.download_queue.len()))
        .size(16);
    
    let active_downloads = if state.download_progress.is_empty() {
        column![text("暂无下载任务").size(16)]
    } else {
        let mut downloads = column![]
            .spacing(10);
        
        for (arxiv_id, (downloaded, total)) in &state.download_progress {
            downloads = downloads.push(download_item(arxiv_id, *downloaded, *total));
        }
        
        downloads
    };
    
    let failed_downloads = if state.download_errors.is_empty() {
        column![]
    } else {
        let mut errors = column![
            text("下载失败:")
                .size(18)
        ]
        .spacing(10);
        
        for (arxiv_id, error) in &state.download_errors {
            errors = errors.push(
                text(format!("{}: {}", arxiv_id, error))
                    .size(14)
            );
        }
        
        errors
    };
    
    let content = column![
        title,
        queue_info,
        text("正在下载:").size(18),
        active_downloads,
        failed_downloads
    ]
    .spacing(15)
    .padding(20);
    
    container(
        scrollable(content)
            .height(Length::Fill)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn download_item(arxiv_id: &str, downloaded: u64, total: Option<u64>) -> Element<AppMessage> {
    let id_text = text(arxiv_id)
        .size(16);
    
    let progress = if let Some(total) = total {
        let percentage = (downloaded as f32 / total as f32).min(1.0);
        let size_text = text(format!("{:.1} MB / {:.1} MB", 
            downloaded as f32 / 1_000_000.0,
            total as f32 / 1_000_000.0
        ))
        .size(12);
        
        column![
            progress_bar(0.0..=1.0, percentage),
            size_text
        ]
        .spacing(5)
    } else {
        let size_text = text(format!("{:.1} MB", downloaded as f32 / 1_000_000.0))
            .size(12);
        
        column![
            progress_bar(0.0..=1.0, 0.0), // Indeterminate progress
            size_text
        ]
        .spacing(5)
    };
    
    let item_content = column![
        id_text,
        progress
    ]
    .spacing(8)
    .padding(10);
    
    container(item_content)
        .width(Length::Fill)
        .into()
}

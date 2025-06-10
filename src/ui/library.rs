use iced::widget::{container, text, column, row, scrollable, button};
use iced::{Element, Length};
use crate::app::{AppMessage, AppState};
use crate::database::{PaperRecord, DownloadStatus};

pub fn library_view(state: &AppState) -> Element<AppMessage> {
    let title = text("论文库")
        .size(24);
    
    let stats = format!("共 {} 篇论文", state.recent_papers.len());
    let stats_text = text(stats).size(16);
    
    let papers_list = if state.recent_papers.is_empty() {
        column![text("论文库为空，请先搜索并下载论文").size(16)]
    } else {
        let mut papers = column![]
            .spacing(10);
        
        for paper in &state.recent_papers {
            papers = papers.push(library_paper_card(paper));
        }
        
        papers
    };
    
    let content = column![
        title,
        stats_text,
        container(
            scrollable(papers_list)
                .height(Length::Fill)
        )
        .height(Length::Fill)
    ]
    .spacing(15)
    .padding(20);
    
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn library_paper_card(paper: &PaperRecord) -> Element<AppMessage> {
    let title = text(&paper.title)
        .size(18);
    
    let authors: Vec<String> = serde_json::from_str(&paper.authors).unwrap_or_default();
    let authors_text = text(format!("作者: {}", authors.join(", ")))
        .size(14);
    
    let categories: Vec<String> = serde_json::from_str(&paper.categories).unwrap_or_default();
    let categories_text = text(format!("分类: {}", categories.join(", ")))
        .size(12);
    
    let status_text = match paper.download_status {
        DownloadStatus::Downloaded => "已下载",
        DownloadStatus::Downloading => "下载中",
        DownloadStatus::Failed => "下载失败",
        DownloadStatus::NotDownloaded => "未下载",
    };
    
    let status = text(status_text)
        .size(12);
    
    let published = text(format!("发布时间: {}", paper.published.format("%Y-%m-%d")))
        .size(12);
    
    let progress_text = if paper.read_progress > 0.0 {
        text(format!("阅读进度: {:.1}%", paper.read_progress * 100.0))
            .size(12)
    } else {
        text("未开始阅读")
            .size(12)
    };
    
    let actions = if paper.download_status == DownloadStatus::Downloaded {
        row![
            button(text("打开").size(14)),
            button(text("在文件夹中显示").size(14))
        ]
        .spacing(10)
    } else {
        row![]
    };
    
    let card_content = column![
        title,
        authors_text,
        row![categories_text, status, published].spacing(20),
        progress_text,
        actions
    ]
    .spacing(8)
    .padding(15);
    
    container(card_content)
        .width(Length::Fill)
        .into()
}

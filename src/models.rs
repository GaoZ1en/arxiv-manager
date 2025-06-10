// arXiv 管理器数据模型

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ArxivPaper {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub abstract_text: String,
    pub published: String,
    pub updated: String,
    pub categories: Vec<String>,
    pub pdf_url: String,
    pub entry_url: String,
}

#[derive(Debug, Clone)]
pub struct DownloadItem {
    pub paper_id: String,
    pub title: String,
    pub progress: f32,
    pub status: DownloadStatus,
    pub file_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PaneType {
    Search,
    Library,
    Downloads,
    Settings,
    PaperView(usize),
}

#[derive(Clone, Debug)]
pub struct Pane {
    pub pane_type: PaneType,
    pub title: String,
}

// 应用消息定义

use iced::widget::pane_grid;
use std::path::PathBuf;

use crate::models::ArxivPaper;

#[derive(Debug, Clone)]
pub enum Message {
    PaneClicked(pane_grid::Pane),
    PaneResized(pane_grid::ResizeEvent),
    PaneDragged(pane_grid::DragEvent),
    SidebarToggled,
    SearchQueryChanged(String),
    SearchSubmitted,
    SearchCompleted(Result<Vec<ArxivPaper>, String>),
    DownloadPaper(ArxivPaper),
    DownloadProgress { paper_id: String, progress: f32 },
    DownloadCompleted { paper_id: String, file_path: PathBuf },
    DownloadFailed { paper_id: String, error: String },
    SavePaper(ArxivPaper),
    RemovePaper(String),
    OpenPaperPane(ArxivPaper),
    CloseFocusedPane,
    SplitHorizontal,
    SplitVertical,
}

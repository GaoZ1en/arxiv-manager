// 应用状态管理

use std::time::Instant;
use iced::widget::pane_grid;
use iced::Task;

use crate::models::{ArxivPaper, DownloadItem, DownloadStatus, Pane, PaneType};
use crate::messages::Message;
use crate::services::{search_arxiv_papers, download_pdf};

pub struct ArxivManager {
    pub panes: pane_grid::State<Pane>,
    pub focus: Option<pane_grid::Pane>,
    pub sidebar_visible: bool,
    pub search_query: String,
    pub search_results: Vec<ArxivPaper>,
    pub saved_papers: Vec<ArxivPaper>,
    pub downloads: Vec<DownloadItem>,
    pub is_searching: bool,
    pub search_error: Option<String>,
    pub last_interaction: Option<Instant>,
}

impl ArxivManager {
    pub fn new() -> (Self, Task<Message>) {
        let (panes, _first_pane) = pane_grid::State::new(Pane {
            pane_type: PaneType::Search,
            title: "Search".to_string(),
        });

        let manager = Self {
            panes,
            focus: None,
            sidebar_visible: true,
            search_query: String::new(),
            search_results: Vec::new(),
            saved_papers: Vec::new(),
            downloads: Vec::new(),
            is_searching: false,
            search_error: None,
            last_interaction: None,
        };

        (manager, Task::none())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PaneClicked(pane) => {
                self.focus = Some(pane);
                self.last_interaction = Some(Instant::now());
                Task::none()
            }
            Message::PaneResized(resize_event) => {
                self.panes.resize(resize_event.split, resize_event.ratio);
                Task::none()
            }
            Message::PaneDragged(_drag_event) => {
                // In iced 0.13, drag handling is managed automatically by the pane grid
                Task::none()
            }
            Message::SidebarToggled => {
                self.sidebar_visible = !self.sidebar_visible;
                Task::none()
            }
            Message::SearchQueryChanged(query) => {
                self.search_query = query;
                Task::none()
            }
            Message::SearchSubmitted => {
                if !self.search_query.trim().is_empty() {
                    self.is_searching = true;
                    self.search_error = None;
                    let query = self.search_query.clone();
                    
                    Task::perform(
                        search_arxiv_papers(query),
                        Message::SearchCompleted
                    )
                } else {
                    Task::none()
                }
            }
            Message::SearchCompleted(result) => {
                self.is_searching = false;
                match result {
                    Ok(papers) => {
                        self.search_results = papers;
                        self.search_error = None;
                    }
                    Err(error) => {
                        self.search_error = Some(error);
                        self.search_results.clear();
                    }
                }
                Task::none()
            }
            Message::DownloadPaper(paper) => {
                let download_item = DownloadItem {
                    paper_id: paper.id.clone(),
                    title: paper.title.clone(),
                    progress: 0.0,
                    status: DownloadStatus::Pending,
                    file_path: None,
                };
                self.downloads.push(download_item);
                
                Task::perform(
                    download_pdf(paper),
                    |result| match result {
                        Ok((paper_id, file_path)) => Message::DownloadCompleted { paper_id, file_path },
                        Err((paper_id, error)) => Message::DownloadFailed { paper_id, error },
                    }
                )
            }
            Message::DownloadProgress { paper_id, progress } => {
                if let Some(download) = self.downloads.iter_mut().find(|d| d.paper_id == paper_id) {
                    download.progress = progress;
                    download.status = DownloadStatus::Downloading;
                }
                Task::none()
            }
            Message::DownloadCompleted { paper_id, file_path } => {
                if let Some(download) = self.downloads.iter_mut().find(|d| d.paper_id == paper_id) {
                    download.progress = 100.0;
                    download.status = DownloadStatus::Completed;
                    download.file_path = Some(file_path);
                }
                Task::none()
            }
            Message::DownloadFailed { paper_id, error } => {
                if let Some(download) = self.downloads.iter_mut().find(|d| d.paper_id == paper_id) {
                    download.status = DownloadStatus::Failed(error);
                }
                Task::none()
            }
            Message::SavePaper(paper) => {
                if !self.saved_papers.iter().any(|p| p.id == paper.id) {
                    self.saved_papers.push(paper);
                }
                Task::none()
            }
            Message::RemovePaper(paper_id) => {
                self.saved_papers.retain(|p| p.id != paper_id);
                Task::none()
            }
            Message::OpenPaperPane(paper) => {
                let index = self.saved_papers.len();
                self.saved_papers.push(paper.clone());
                
                let pane_type = PaneType::PaperView(index);
                let new_pane = Pane {
                    pane_type,
                    title: paper.title.clone(),
                };
                
                if let Some(focus) = self.focus {
                    let _ = self.panes.split(
                        pane_grid::Axis::Vertical,
                        focus,
                        new_pane,
                    );
                }
                // If no focus, we can't split - just save the paper instead
                Task::none()
            }
            Message::CloseFocusedPane => {
                if let Some(focus) = self.focus {
                    if let Some(_) = self.panes.close(focus) {
                        self.focus = None;
                    }
                }
                Task::none()
            }
            Message::SplitHorizontal => {
                if let Some(focus) = self.focus {
                    let new_pane = Pane {
                        pane_type: PaneType::Search,
                        title: "Search".to_string(),
                    };
                    let _ = self.panes.split(pane_grid::Axis::Horizontal, focus, new_pane);
                }
                Task::none()
            }
            Message::SplitVertical => {
                if let Some(focus) = self.focus {
                    let new_pane = Pane {
                        pane_type: PaneType::Search,
                        title: "Search".to_string(),
                    };
                    let _ = self.panes.split(pane_grid::Axis::Vertical, focus, new_pane);
                }
                Task::none()
            }
        }
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}

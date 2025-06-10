// 应用状态管理

use std::time::Instant;
use iced::widget::pane_grid;
use iced::Task;

use crate::models::{ArxivPaper, DownloadItem, DownloadStatus, Pane, PaneType, SearchConfig, AppSettings};
use crate::messages::Message;
use crate::services::{search_arxiv_papers_advanced, download_pdf};

pub struct ArxivManager {
    pub panes: pane_grid::State<Pane>,
    pub focus: Option<pane_grid::Pane>,
    pub sidebar_visible: bool,
    pub search_query: String,
    pub search_config: SearchConfig,
    pub advanced_search_visible: bool,
    pub search_results: Vec<ArxivPaper>,
    pub saved_papers: Vec<ArxivPaper>,
    pub downloads: Vec<DownloadItem>,
    pub is_searching: bool,
    pub search_error: Option<String>,
    pub last_interaction: Option<Instant>,
    pub settings: AppSettings,
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
            search_config: SearchConfig::default(),
            advanced_search_visible: false,
            search_results: Vec::new(),
            saved_papers: Vec::new(),
            downloads: Vec::new(),
            is_searching: false,
            search_error: None,
            last_interaction: None,
            settings: AppSettings::default(),
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
                self.search_query = query.clone();
                self.search_config.query = query;
                Task::none()
            }
            Message::SearchSubmitted => {
                if !self.search_config.query.trim().is_empty() {
                    self.is_searching = true;
                    self.search_error = None;
                    let config = self.search_config.clone();
                    
                    Task::perform(
                        search_arxiv_papers_advanced(config),
                        Message::SearchCompleted
                    )
                } else {
                    Task::none()
                }
            }
            // 高级搜索消息处理
            Message::AdvancedSearchToggled => {
                self.advanced_search_visible = !self.advanced_search_visible;
                Task::none()
            }
            Message::SearchFieldChanged(field) => {
                self.search_config.search_in = field;
                Task::none()
            }
            Message::CategoryToggled(category) => {
                if let Some(pos) = self.search_config.categories.iter().position(|x| x == &category) {
                    self.search_config.categories.remove(pos);
                } else {
                    self.search_config.categories.push(category);
                }
                Task::none()
            }
            Message::DateRangeChanged(range) => {
                self.search_config.date_range = range;
                Task::none()
            }
            Message::SortByChanged(sort_by) => {
                self.search_config.sort_by = sort_by;
                Task::none()
            }
            Message::SortOrderChanged(order) => {
                self.search_config.sort_order = order;
                Task::none()
            }
            Message::MaxResultsChanged(value) => {
                if let Ok(num) = value.parse::<u32>() {
                    self.search_config.max_results = num.min(100).max(1);
                }
                Task::none()
            }
            Message::AuthorAdded(author) => {
                if !author.trim().is_empty() && !self.search_config.authors.contains(&author) {
                    self.search_config.authors.push(author);
                }
                Task::none()
            }
            Message::AuthorRemoved(index) => {
                if index < self.search_config.authors.len() {
                    self.search_config.authors.remove(index);
                }
                Task::none()
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
                // 检查论文是否已经在saved_papers中
                let index = if let Some(existing_index) = self.saved_papers.iter().position(|p| p.id == paper.id) {
                    existing_index
                } else {
                    // 如果不在，则添加
                    self.saved_papers.push(paper.clone());
                    self.saved_papers.len() - 1
                };
                
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
            // 设置消息处理
            Message::ThemeChanged(theme) => {
                self.settings.theme = theme;
                Task::none()
            }
            Message::DownloadDirectoryChanged(directory) => {
                self.settings.download_directory = directory;
                Task::none()
            }
            Message::AutoDownloadToggled => {
                self.settings.auto_download = !self.settings.auto_download;
                Task::none()
            }
            Message::MaxConcurrentDownloadsChanged(value) => {
                if let Ok(num) = value.parse::<u32>() {
                    self.settings.max_concurrent_downloads = num.min(10).max(1);
                }
                Task::none()
            }
            Message::ShowAbstractsToggled => {
                self.settings.show_abstracts_in_search = !self.settings.show_abstracts_in_search;
                Task::none()
            }
            Message::DefaultSearchFieldChanged(field) => {
                self.settings.default_search_field = field;
                Task::none()
            }
            Message::DefaultSortByChanged(sort_by) => {
                self.settings.default_sort_by = sort_by;
                Task::none()
            }
            Message::DefaultSortOrderChanged(order) => {
                self.settings.default_sort_order = order;
                Task::none()
            }
            Message::DefaultMaxResultsChanged(value) => {
                if let Ok(num) = value.parse::<u32>() {
                    self.settings.default_max_results = num.min(100).max(1);
                }
                Task::none()
            }
            Message::AutoSaveSearchesToggled => {
                self.settings.auto_save_searches = !self.settings.auto_save_searches;
                Task::none()
            }
            Message::NotificationToggled => {
                self.settings.notification_enabled = !self.settings.notification_enabled;
                Task::none()
            }
            Message::CheckUpdatesToggled => {
                self.settings.check_updates = !self.settings.check_updates;
                Task::none()
            }
            Message::LanguageChanged(language) => {
                self.settings.language = language;
                Task::none()
            }
            Message::ResetSettings => {
                self.settings = AppSettings::default();
                Task::none()
            }
            Message::ExportSettings => {
                // TODO: 实现设置导出
                Task::none()
            }
            Message::ImportSettings => {
                // TODO: 实现设置导入
                Task::none()
            }
        }
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}

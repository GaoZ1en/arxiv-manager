// 应用消息定义

use iced::widget::pane_grid;
use std::path::PathBuf;

use crate::core::models::{ArxivPaper, SearchField, DateRange, SortBy, SortOrder};

#[derive(Debug, Clone)]
pub enum Message {
    PaneClicked(pane_grid::Pane),
    PaneResized(pane_grid::ResizeEvent),
    PaneDragged(pane_grid::DragEvent),
    SidebarToggled,
    SearchQueryChanged(String),
    SearchSubmitted,
    SearchCompleted(Result<Vec<ArxivPaper>, String>),
    // 高级搜索消息
    AdvancedSearchToggled,
    SearchFieldChanged(SearchField),
    CategoryToggled(String),
    DateRangeChanged(DateRange),
    SortByChanged(SortBy),
    SortOrderChanged(SortOrder),
    MaxResultsChanged(String),
    AuthorAdded(String),
    AuthorRemoved(usize),
    // 下载和保存操作
    DownloadPaper(ArxivPaper),
    DownloadProgress { paper_id: String, progress: f32 },
    DownloadCompleted { paper_id: String, file_path: PathBuf },
    DownloadFailed { paper_id: String, error: String },
    SavePaper(ArxivPaper),
    RemovePaper(String),
    OpenPaperPane(ArxivPaper),
    // 界面操作
    CloseFocusedPane,
    SplitHorizontal,
    SplitVertical,
    // 面板导航
    OpenSearchPane,
    OpenLibraryPane,
    OpenDownloadsPane,
    OpenSettingsPane,
    // 设置消息
    ThemeChanged(crate::core::models::Theme),
    DownloadDirectoryChanged(String),
    AutoDownloadToggled,
    MaxConcurrentDownloadsChanged(String),
    ShowAbstractsToggled,
    DefaultSearchFieldChanged(SearchField),
    DefaultSortByChanged(SortBy),
    DefaultSortOrderChanged(SortOrder),
    DefaultMaxResultsChanged(String),
    AutoSaveSearchesToggled,
    NotificationToggled,
    CheckUpdatesToggled,
    LanguageChanged(crate::core::models::Language),
    ResetSettings,
    ExportSettings,
    ImportSettings,
}

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
    // 快捷键设置
    ShortcutChanged { action: String, shortcut: String },
    ResetShortcuts,
    // 键盘快捷键和命令栏
    ToggleCommandPalette,
    CommandPaletteInputChanged(String),
    CommandSelected(usize),
    ExecuteCommand(Command),
    ClearCommandPalette,
    // 快捷键操作
    FocusSearchInput,
    QuickSaveCurrentPaper,
    QuickDownloadCurrentPaper,
    NavigateToNextPane,
    NavigateToPreviousPane,
    ToggleFullscreen,
}

#[derive(Debug, Clone)]
pub enum Command {
    // 搜索命令
    NewSearch,
    AdvancedSearch,
    ClearSearch,
    // 导航命令  
    GoToSearch,
    GoToLibrary,
    GoToDownloads,
    GoToSettings,
    // 面板操作
    SplitPaneHorizontal,
    SplitPaneVertical,
    CloseCurrentPane,
    // 文件操作
    OpenPaper(String), // paper title or id
    SaveCurrentPaper,
    DownloadCurrentPaper,
    // 设置操作
    ToggleTheme,
    ToggleSidebar,
    OpenSettings,
    // 应用操作
    ShowHelp,
    ShowAbout,
    Quit,
}

impl Command {
    pub fn display_name(&self) -> &'static str {
        match self {
            Command::NewSearch => "New Search",
            Command::AdvancedSearch => "Advanced Search",
            Command::ClearSearch => "Clear Search",
            Command::GoToSearch => "Go to Search",
            Command::GoToLibrary => "Go to Library",
            Command::GoToDownloads => "Go to Downloads", 
            Command::GoToSettings => "Go to Settings",
            Command::SplitPaneHorizontal => "Split Pane Horizontally",
            Command::SplitPaneVertical => "Split Pane Vertically",
            Command::CloseCurrentPane => "Close Current Pane",
            Command::OpenPaper(_) => "Open Paper",
            Command::SaveCurrentPaper => "Save Current Paper",
            Command::DownloadCurrentPaper => "Download Current Paper",
            Command::ToggleTheme => "Toggle Theme",
            Command::ToggleSidebar => "Toggle Sidebar",
            Command::OpenSettings => "Open Settings",
            Command::ShowHelp => "Show Help",
            Command::ShowAbout => "Show About",
            Command::Quit => "Quit Application",
        }
    }

    pub fn keywords(&self) -> Vec<&'static str> {
        match self {
            Command::NewSearch => vec!["search", "new", "find"],
            Command::AdvancedSearch => vec!["advanced", "search", "filter"],
            Command::ClearSearch => vec!["clear", "reset", "search"],
            Command::GoToSearch => vec!["go", "search", "navigate"],
            Command::GoToLibrary => vec!["go", "library", "saved", "papers"],
            Command::GoToDownloads => vec!["go", "downloads", "files"],
            Command::GoToSettings => vec!["go", "settings", "preferences"],
            Command::SplitPaneHorizontal => vec!["split", "horizontal", "pane"],
            Command::SplitPaneVertical => vec!["split", "vertical", "pane"],
            Command::CloseCurrentPane => vec!["close", "pane", "tab"],
            Command::OpenPaper(_) => vec!["open", "paper", "view"],
            Command::SaveCurrentPaper => vec!["save", "paper", "library"],
            Command::DownloadCurrentPaper => vec!["download", "pdf", "file"],
            Command::ToggleTheme => vec!["theme", "dark", "light", "toggle"],
            Command::ToggleSidebar => vec!["sidebar", "toggle", "hide"],
            Command::OpenSettings => vec!["settings", "preferences", "config"],
            Command::ShowHelp => vec!["help", "documentation"],
            Command::ShowAbout => vec!["about", "version", "info"],
            Command::Quit => vec!["quit", "exit", "close"],
        }
    }
}

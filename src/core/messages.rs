// 应用消息定义

use iced::widget::pane_grid;
use std::path::PathBuf;

use crate::core::models::{ArxivPaper, SearchField, DateRange, SortBy, SortOrder, ArxivCategory, LibrarySortBy, LibraryGroupBy, LibraryViewMode};

#[derive(Debug, Clone)]
pub enum Message {
    PaneClicked(pane_grid::Pane),
    PaneResized(pane_grid::ResizeEvent),
    PaneDragged(pane_grid::DragEvent),
    SidebarToggled,
    SearchQueryChanged(String),
    SearchSubmitted,
    SearchCompleted(Result<Vec<ArxivPaper>, String>),
    // 无限滚动
    LoadMoreResults,
    LoadMoreCompleted(Result<Vec<ArxivPaper>, String>),
    // 滚动事件
    ScrolledToBottom,
    // 搜索建议
    SearchSuggestionSelected(String),
    HideSearchSuggestions,
    // 高级搜索消息
    AdvancedSearchToggled,
    SearchFieldChanged(SearchField),
    CategoryToggled(ArxivCategory),
    DateRangeChanged(DateRange),
    SortByChanged(SortBy),
    SortOrderChanged(SortOrder),
    MaxResultsChanged(String),
    AuthorAdded(String),
    AuthorRemoved(usize),
    // 作者输入
    AuthorInputChanged(String),
    // 快速作者搜索
    SearchByAuthor(String),
    // 下载和保存操作
    DownloadPaper(ArxivPaper),
    DownloadProgress { paper_id: String, progress: f32 },
    DownloadCompleted { paper_id: String, file_path: PathBuf },
    DownloadFailed { paper_id: String, error: String },
    SavePaper(ArxivPaper),
    RemovePaper(String),
    // Collection/Folder 操作
    CreateCollection { name: String, parent_id: Option<i64> },
    RenameCollection { id: i64, new_name: String },
    StartRenameCollection(i64), // 开始重命名集合
    CancelRenameCollection,     // 取消重命名
    CollectionRenameInputChanged(String), // 重命名输入框内容改变
    DeleteCollection(i64),
    MoveCollection { id: i64, new_parent_id: Option<i64> },
    ToggleCollectionExpanded(i64),
    AddPaperToCollection { paper_index: usize, collection_id: i64 },
    RemovePaperFromCollection { paper_index: usize, collection_id: i64 },
    SelectCollection(Option<i64>), // None表示显示所有论文
    CollectionCreated(i64), // 创建成功的回调
    CollectionUpdated(i64), // 更新成功的回调
    CollectionDeleted(i64), // 删除成功的回调
    LoadCollections, // 加载所有集合
    CollectionsLoaded(Vec<crate::core::models::Collection>), // 集合加载完成
    // 论文管理功能
    TogglePaperFavorite(String), // 切换论文收藏状态 (paper_id)
    SetPaperRating { paper_id: String, rating: Option<u8> },
    SetPaperReadStatus { paper_id: String, status: crate::core::models::ReadingStatus },
    AddPaperTag { paper_id: String, tag: String },
    RemovePaperTag { paper_id: String, tag: String },
    SetPaperNotes { paper_id: String, notes: Option<String> },
    // 文章排序和显示
    LibrarySortChanged(LibrarySortBy),
    LibraryGroupChanged(LibraryGroupBy),
    LibraryViewModeChanged(LibraryViewMode),
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
    // 字体和缩放设置
    FontFamilyChanged(String),
    FontSizeChanged(String),
    UIScaleChanged(String),
    // 快捷键设置
    ShortcutChanged { 
        #[allow(dead_code)]
        action: String, 
        #[allow(dead_code)]
        shortcut: String 
    },
    ShortcutEditStarted(String), // 开始编辑某个快捷键
    ShortcutEditCancelled,       // 取消编辑快捷键
    ShortcutInputChanged(String), // 快捷键输入改变
    ResetShortcuts,
    // 键盘快捷键和命令栏
    ToggleCommandPalette,
    CommandPaletteInputChanged(String),
    CommandSelected(usize),
    ExecuteCommand(Command),
    ClearCommandPalette,
    // No operation message (used as placeholder)
    NoOp,
    // 快捷键操作
    FocusSearchInput,
    QuickSaveCurrentPaper,
    QuickDownloadCurrentPaper,
    ToggleFullscreen,
    // 标签页操作
    TabClicked(usize),
    TabClose(usize),
    NewTab(crate::core::models::TabContent),
    NavigateToNextTab,
    NavigateToPreviousTab,
    CloseActiveTab, // 关闭当前活动标签页
    // 右键菜单操作
    TabRightClicked { tab_index: usize, position: iced::Point },
    HideContextMenu,
    TabMoveToGroup(usize, crate::core::models::ui::TabGroup),
    TabDuplicate(usize),
    CloseTabsToRight(usize),
    CloseOtherTabs(usize),
    CloseTabsInGroup(crate::core::models::ui::TabGroup),
    // 会话管理
    SaveSession,
    LoadSession,
    // 窗口事件
    WindowResized { width: f32, height: f32 },
    // 滚动条相关消息
    ScrollbarActivity(String), // 滚动条标识符
    ScrollbarHovered(String, bool), // 滚动条标识符和悬停状态
    ScrollbarDragged(String, bool), // 滚动条标识符和拖拽状态
    ScrollbarTick, // 定时检查淡出
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
    #[allow(dead_code)]
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

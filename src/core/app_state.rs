// 重构后的应用状态管理 - 模块化版本
// 使用处理器模式将巨大的update方法分解为专门的处理器

use std::time::Instant;
use std::collections::HashMap;
use iced::{Task, Subscription};

use crate::core::{ArxivPaper, DownloadItem, DownloadStatus, SearchConfig, AppSettings, Tab, TabContent, SessionManager, LibrarySortBy, LibraryGroupBy, LibraryViewMode};
use crate::core::messages::{Message, Command};
use crate::pdf::{PdfViewer, PdfViewerMessage};

// 导入所有处理器
use crate::core::handlers::{
    TabHandler, SearchHandler, DownloadHandler, SettingsHandler, 
    CommandHandler, PaperHandler, ShortcutHandler, LibraryHandler, CollectionHandler
};

/// 滚动条状态管理
#[derive(Debug, Clone)]
pub struct ScrollbarState {
    pub last_activity: Instant,
    pub is_hovered: bool,
    pub is_dragged: bool,
    pub should_fade: bool,
    pub fade_delay: std::time::Duration,
}

impl ScrollbarState {
    /// 创建新的滚动条状态
    pub fn new() -> Self {
        Self {
            last_activity: Instant::now(),
            is_hovered: false,
            is_dragged: false,
            should_fade: false,
            fade_delay: std::time::Duration::from_secs(1), // 1秒后开始淡出
        }
    }
    
    /// 记录滚动条活动
    pub fn record_activity(&mut self) {
        self.last_activity = Instant::now();
        self.should_fade = false;
    }
    
    /// 设置悬停状态
    pub fn set_hovered(&mut self, hovered: bool) {
        self.is_hovered = hovered;
        if hovered {
            self.record_activity();
        }
    }
    
    /// 设置拖拽状态
    pub fn set_dragged(&mut self, dragged: bool) {
        self.is_dragged = dragged;
        if dragged {
            self.record_activity();
        }
    }
    
    /// 检查是否应该淡出
    pub fn should_auto_fade(&self) -> bool {
        !self.is_hovered && 
        !self.is_dragged && 
        self.last_activity.elapsed() > self.fade_delay
    }
    
    /// 获取透明度值（0.0-1.0）
    pub fn get_alpha(&self) -> f32 {
        if self.is_dragged {
            1.0 // 拖拽时完全不透明
        } else if self.is_hovered {
            0.8 // 悬停时稍微透明
        } else if self.should_auto_fade() {
            0.1 // 淡出时几乎透明
        } else {
            0.4 // 默认半透明
        }
    }
}

impl Default for ScrollbarState {
    fn default() -> Self {
        Self::new()
    }
}

// 搜索缓存项
#[derive(Debug, Clone)]
pub struct SearchCacheItem {
    pub results: Vec<ArxivPaper>,
    pub timestamp: Instant,
    #[allow(dead_code)]
    pub config: SearchConfig,
}

// 搜索性能统计
#[derive(Debug, Clone)]
pub struct SearchPerformanceStats {
    pub total_searches: u32,
    pub cache_hits: u32,
    pub average_response_time: f64,
    pub last_search_duration: Option<std::time::Duration>,
}

pub struct ArxivManager {
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
    pub next_tab_id: usize,
    pub sidebar_visible: bool,
    pub search_query: String,
    pub search_config: SearchConfig,
    pub advanced_search_visible: bool,
    pub search_results: Vec<ArxivPaper>,
    pub saved_papers: Vec<ArxivPaper>,
    pub downloads: Vec<DownloadItem>,
    // Collection/Library 管理
    pub collections: Vec<crate::core::models::Collection>,
    pub selected_collection_id: Option<i64>, // 当前选中的集合ID
    pub collection_tree_expanded: HashMap<i64, bool>, // 集合展开状态
    pub filtered_papers: Vec<ArxivPaper>, // 根据选中集合过滤的论文
    pub is_creating_collection: bool,
    pub collection_name_input: String,
    pub collection_parent_id: Option<i64>, // 创建子集合时的父集合ID
    // 集合重命名状态
    pub editing_collection_id: Option<i64>, // 当前正在编辑的集合ID
    pub collection_rename_input: String,   // 重命名输入框的内容
    // Library视图设置
    pub library_sort_by: LibrarySortBy,     // Library排序方式
    pub library_group_by: LibraryGroupBy,   // Library分组方式
    pub library_view_mode: LibraryViewMode, // Library显示模式
    pub is_searching: bool,
    pub search_error: Option<String>,
    pub last_interaction: Option<Instant>,
    // 无限滚动分页支持
    pub current_page: u32,
    pub total_results_loaded: u32,
    pub has_more_results: bool,
    pub is_loading_more: bool,
    pub settings: AppSettings,
    // 速度优化 - 搜索缓存和防抖动
    pub search_cache: HashMap<String, SearchCacheItem>,
    pub last_search_time: Option<Instant>,
    pub search_debounce_delay: std::time::Duration,
    // 性能优化 - 智能缓存和并发搜索
    pub query_similarity_cache: HashMap<String, Vec<String>>,
    pub query_frequency: HashMap<String, u32>,
    pub preload_queue: Vec<String>,
    pub search_performance_stats: SearchPerformanceStats,
    // 搜索历史和建议
    pub search_history: Vec<String>,
    pub search_suggestions: Vec<String>,
    pub show_search_suggestions: bool,
    // 作者输入状态
    pub author_input: String,
    // 命令栏状态
    pub command_palette_visible: bool,
    pub command_palette_input: String,
    pub command_suggestions: Vec<Command>,
    pub selected_command_index: Option<usize>,
    // 快捷键编辑状态
    pub editing_shortcut: Option<String>, // 正在编辑的快捷键动作
    pub shortcut_input: String,           // 快捷键输入缓存
    // 右键菜单状态
    pub context_menu: crate::ui::components::ContextMenuState,
    // 窗口尺寸跟踪
    pub window_width: f32,
    pub window_height: f32,
    // 滚动条状态管理
    pub scrollbar_states: HashMap<String, ScrollbarState>,
    // PDF查看器管理
    pub pdf_viewers: HashMap<String, PdfViewer>,
}

impl ArxivManager {
    pub fn new() -> (Self, Task<Message>) {
        // 尝试加载保存的会话状态
        let (tabs, active_tab, next_tab_id) = if SessionManager::session_exists() {
            match SessionManager::load_session() {
                Ok(session_data) => {
                    log::info!("Loaded previous session with {} tabs", session_data.tabs.len());
                    let tabs: Vec<Tab> = session_data.tabs.into_iter().map(|tab| tab.into()).collect();
                    let active_tab = session_data.active_tab.min(tabs.len().saturating_sub(1));
                    (tabs, active_tab, session_data.next_tab_id)
                }
                Err(e) => {
                    log::warn!("Failed to load session, using default tabs: {}", e);
                    Self::create_default_tabs()
                }
            }
        } else {
            log::info!("No previous session found, creating default tabs");
            Self::create_default_tabs()
        };

        let mut manager = Self {
            tabs,
            active_tab,
            next_tab_id,
            sidebar_visible: true,
            search_query: String::new(),
            search_config: SearchConfig::default(),
            advanced_search_visible: false,
            search_results: Vec::new(),
            saved_papers: Vec::new(),
            downloads: Vec::new(),
            // Collection/Library 管理初始化
            collections: Vec::new(),
            selected_collection_id: None,
            collection_tree_expanded: HashMap::new(),
            filtered_papers: Vec::new(),
            is_creating_collection: false,
            collection_name_input: String::new(),
            collection_parent_id: None,
            // 集合重命名状态
            editing_collection_id: None,
            collection_rename_input: String::new(),
            // Library视图设置默认值
            library_sort_by: LibrarySortBy::Title,     // 默认按标题排序
            library_group_by: LibraryGroupBy::None,    // 默认不分组
            library_view_mode: LibraryViewMode::default(), // 默认瀑布流视图
            is_searching: false,
            search_error: None,
            last_interaction: None,
            // 无限滚动分页初始化
            current_page: 0,
            total_results_loaded: 0,
            has_more_results: true,
            is_loading_more: false,
            settings: AppSettings::default(),
            // 速度优化 - 搜索缓存和防抖动初始化
            search_cache: HashMap::new(),
            last_search_time: None,
            search_debounce_delay: std::time::Duration::from_millis(300),
            // 性能优化 - 智能缓存和并发搜索初始化
            query_similarity_cache: HashMap::new(),
            query_frequency: HashMap::new(),
            preload_queue: Vec::new(),
            search_performance_stats: SearchPerformanceStats {
                total_searches: 0,
                cache_hits: 0,
                average_response_time: 0.0,
                last_search_duration: None,
            },
            // 搜索历史和建议初始化
            search_history: Vec::new(),
            search_suggestions: Vec::new(),
            show_search_suggestions: false,
            // 作者输入初始化
            author_input: String::new(),
            // 命令栏初始化
            command_palette_visible: false,
            command_palette_input: String::new(),
            command_suggestions: Vec::new(),
            selected_command_index: None,
            // 快捷键编辑状态初始化
            editing_shortcut: None,
            shortcut_input: String::new(),
            // 右键菜单状态初始化
            context_menu: crate::ui::components::ContextMenuState::default(),
            // 窗口尺寸初始化（默认尺寸）
            window_width: 1400.0,
            window_height: 900.0,
            // 滚动条状态管理初始化
            scrollbar_states: HashMap::new(),
            // PDF查看器管理初始化
            pdf_viewers: HashMap::new(),
        };

        // 确保标签页按分组排序
        manager.sort_tabs_by_groups();

        (manager, Task::none())
    }
    
    // 创建默认标签页的辅助方法
    fn create_default_tabs() -> (Vec<Tab>, usize, usize) {
        let mut tabs = Vec::new();
        // 只创建搜索标签页，让用户通过侧边栏或快捷键添加其他标签页
        tabs.push(Tab::new(0, "Search".to_string(), TabContent::Search));
        (tabs, 0, 1)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // PDF查看器相关消息
            Message::PdfViewer(tab_id, pdf_msg) => self.handle_pdf_viewer_message(tab_id, pdf_msg),
            Message::OpenPdfFile(path) => self.handle_open_pdf_file(path),
            Message::OpenOrDownloadPdf(paper) => self.handle_open_or_download_pdf(paper),
            
            // 标签页相关消息 - 委托给 TabHandler
            Message::TabClicked(tab_index) => self.handle_tab_clicked(tab_index),
            Message::TabClose(tab_index) => self.handle_tab_close(tab_index),
            Message::NavigateToNextTab => self.handle_navigate_to_next_tab(),
            Message::NavigateToPreviousTab => self.handle_navigate_to_previous_tab(),
            Message::CloseActiveTab => self.handle_close_active_tab(),
            Message::NewTab(content) => self.handle_new_tab(content),
            // 新的标签页操作
            Message::TabRightClicked { tab_index, position } => self.handle_tab_right_clicked(tab_index, position),
            Message::HideContextMenu => {
                self.context_menu.visible = false;
                Task::none()
            },
            Message::TabMoveToGroup(tab_index, group) => self.handle_tab_move_to_group(tab_index, group),
            Message::TabDuplicate(tab_index) => self.handle_tab_duplicate(tab_index),
            Message::CloseTabsToRight(tab_index) => self.handle_close_tabs_to_right(tab_index),
            Message::CloseOtherTabs(tab_index) => self.handle_close_other_tabs(tab_index),
            Message::CloseTabsInGroup(group) => self.handle_close_tabs_in_group(group),
            // 会话管理
            Message::SaveSession => self.handle_save_session(),
            Message::LoadSession => self.handle_load_session(),

            // 搜索相关消息 - 委托给 SearchHandler
            Message::SearchQueryChanged(query) => self.handle_search_query_changed(query),
            Message::SearchSubmitted => self.handle_search_submitted(),
            Message::SearchCompleted(result) => self.handle_search_completed(result),
            // 搜索建议
            Message::SearchSuggestionSelected(suggestion) => {
                // 更新两个查询字段以保持同步
                self.search_query = suggestion.clone();
                self.search_config.query = suggestion;
                self.show_search_suggestions = false;
                self.handle_search_submitted()
            },
            Message::HideSearchSuggestions => {
                self.show_search_suggestions = false;
                Task::none()
            },
            // 高级搜索消息
            Message::AdvancedSearchToggled => self.handle_advanced_search_toggled(),
            Message::SearchFieldChanged(field) => self.handle_search_field_changed(field),
            Message::CategoryToggled(category) => self.handle_category_toggled(category),
            Message::DateRangeChanged(range) => self.handle_date_range_changed(range),
            Message::SortByChanged(sort_by) => self.handle_sort_by_changed(sort_by),
            Message::SortOrderChanged(order) => self.handle_sort_order_changed(order),
            Message::MaxResultsChanged(value) => self.handle_max_results_changed(value),
            Message::AuthorAdded(author) => self.handle_author_added(author),
            Message::AuthorRemoved(index) => self.handle_author_removed(index),
            Message::AuthorInputChanged(input) => {
                self.author_input = input;
                Task::none()
            },
            Message::SearchByAuthor(author) => self.handle_search_by_author(author),
            
            // 无限滚动相关消息
            Message::LoadMoreResults => self.handle_load_more_results(),
            Message::LoadMoreCompleted(result) => self.handle_load_more_completed(result),
            Message::ScrolledToBottom => {
                // 当滚动到底部时，自动触发加载更多
                if self.has_more_results && !self.is_loading_more && !self.search_results.is_empty() {
                    self.handle_load_more_results()
                } else {
                    Task::none()
                }
            },

            // 论文相关消息 - 委托给 PaperHandler  
            Message::SavePaper(paper) => self.handle_paper_save(paper),
            Message::RemovePaper(paper_id) => {
                // 根据paper_id找到索引并删除
                if let Some(index) = self.saved_papers.iter().position(|p| p.id == paper_id) {
                    self.handle_paper_remove(index)
                } else {
                    Task::none()
                }
            },

            // 下载相关消息 - 委托给 DownloadHandler
            Message::DownloadPaper(paper) => self.handle_download_paper(paper),
            Message::DownloadProgress { paper_id, progress } => self.handle_download_progress(paper_id, progress),
            Message::DownloadCompleted { paper_id, file_path } => self.handle_download_completed(paper_id, file_path.to_string_lossy().to_string()),
            Message::DownloadFailed { paper_id, error } => self.handle_download_failed(paper_id, error),

            // 设置相关消息 - 委托给 SettingsHandler
            Message::ThemeChanged(theme) => self.handle_theme_changed(theme),
            Message::LanguageChanged(language) => self.handle_language_changed(language),
            Message::DownloadDirectoryChanged(path) => self.handle_download_directory_changed(path),
            Message::AutoDownloadToggled => self.handle_auto_download_toggled(),
            Message::MaxConcurrentDownloadsChanged(value) => self.handle_max_concurrent_downloads_changed(value),
            Message::ShowAbstractsToggled => self.handle_show_abstracts_toggled(),
            Message::DefaultSearchFieldChanged(field) => self.handle_default_search_field_changed(field),
            Message::DefaultSortByChanged(sort_by) => self.handle_default_sort_by_changed(sort_by),
            Message::DefaultSortOrderChanged(order) => self.handle_default_sort_order_changed(order),
            Message::DefaultMaxResultsChanged(value) => self.handle_default_max_results_changed(value),
            Message::AutoSaveSearchesToggled => self.handle_auto_save_searches_toggled(),
            Message::NotificationToggled => self.handle_notification_toggled(),
            Message::CheckUpdatesToggled => self.handle_check_updates_toggled(),
            Message::ResetSettings => self.handle_settings_reset(),
            Message::ExportSettings => self.handle_settings_export(),
            Message::ImportSettings => self.handle_settings_import("".to_string()),
            // 字体和缩放设置消息
            Message::FontFamilyChanged(font_family) => self.handle_font_family_changed(font_family),
            Message::FontSizeChanged(font_size) => self.handle_font_size_changed(font_size),
            Message::UIScaleChanged(ui_scale) => self.handle_ui_scale_changed(ui_scale),

            // Library视图相关消息 - 委托给 LibraryHandler
            Message::LibrarySortChanged(sort_by) => self.handle_library_sort_changed(sort_by),
            Message::LibraryGroupChanged(group_by) => self.handle_library_group_changed(group_by),
            Message::LibraryViewModeChanged(view_mode) => self.handle_library_view_mode_changed(view_mode),

            // 论文管理功能消息 - 委托给 LibraryHandler
            Message::TogglePaperFavorite(paper_id) => self.handle_toggle_paper_favorite(&paper_id),
            Message::SetPaperRating { paper_id, rating } => self.handle_set_paper_rating(paper_id, rating),
            Message::SetPaperReadStatus { paper_id, status } => self.handle_set_paper_read_status(paper_id, status),
            Message::AddPaperTag { paper_id, tag } => self.handle_add_paper_tag(paper_id, tag),
            Message::RemovePaperTag { paper_id, tag } => self.handle_remove_paper_tag(paper_id, tag),
            Message::SetPaperNotes { paper_id, notes } => self.handle_set_paper_notes(paper_id, notes),

            // Collection/文件夹相关消息 - 委托给 CollectionHandler
            Message::CreateCollection { name, parent_id } => self.handle_create_collection(name, parent_id),
            Message::RenameCollection { id, new_name } => self.handle_rename_collection(id, new_name),
            Message::StartRenameCollection(id) => {
                // 开始重命名：设置编辑状态，并预填充当前名称
                if let Some(collection) = self.collections.iter().find(|c| c.id == id) {
                    self.editing_collection_id = Some(id);
                    self.collection_rename_input = collection.name.clone();
                }
                Task::none()
            },
            Message::CancelRenameCollection => {
                // 取消重命名：清除编辑状态
                self.editing_collection_id = None;
                self.collection_rename_input.clear();
                Task::none()
            },
            Message::CollectionRenameInputChanged(input) => {
                // 更新重命名输入框内容
                self.collection_rename_input = input;
                Task::none()
            },
            Message::DeleteCollection(id) => self.handle_delete_collection(id),
            Message::MoveCollection { id, new_parent_id } => self.handle_move_collection(id, new_parent_id),
            Message::ToggleCollectionExpanded(id) => self.handle_toggle_collection_expanded(id),
            Message::AddPaperToCollection { paper_index, collection_id } => self.handle_add_paper_to_collection(paper_index, collection_id),
            Message::RemovePaperFromCollection { paper_index, collection_id } => self.handle_remove_paper_from_collection(paper_index, collection_id),
            Message::SelectCollection(collection_id) => self.handle_select_collection(collection_id),
            Message::LoadCollections => self.handle_load_collections(),
            Message::CollectionsLoaded(collections) => self.handle_collections_loaded(collections),
            Message::CollectionCreated(_id) => {
                // 创建成功后可以进行一些额外处理，如刷新UI
                Task::none()
            },
            Message::CollectionUpdated(_id) => {
                // 更新成功后可以进行一些额外处理
                Task::none()
            },
            Message::CollectionDeleted(_id) => {
                // 删除成功后可以进行一些额外处理
                Task::none()
            },

            // 命令面板相关消息 - 委托给 CommandHandler
            Message::ToggleCommandPalette => self.handle_command_palette_toggled(),
            Message::CommandPaletteInputChanged(input) => self.handle_command_input_changed(input),
            Message::CommandSelected(index) => self.handle_command_selected(index),
            Message::ExecuteCommand(command) => self.handle_command_executed(command),
            Message::SidebarToggled => self.handle_sidebar_toggled(),

            // 快捷键相关消息 - 委托给 ShortcutHandler
            Message::ShortcutEditStarted(action) => self.handle_shortcut_edit_started(action),
            Message::ShortcutEditCancelled => self.handle_shortcut_edit_cancelled(),
            Message::ShortcutInputChanged(input) => self.handle_shortcut_input_changed(input),
            Message::ResetShortcuts => self.handle_shortcuts_reset(),

            // 窗口事件处理
            Message::WindowResized { width, height } => {
                self.window_width = width;
                self.window_height = height;
                Task::none()
            },

            // 滚动条相关消息
            Message::ScrollbarActivity(id) => {
                let state = self.scrollbar_states.entry(id).or_default();
                state.record_activity();
                Task::none()
            },
            Message::ScrollbarHovered(id, hovered) => {
                let state = self.scrollbar_states.entry(id).or_default();
                state.set_hovered(hovered);
                Task::none()
            },
            Message::ScrollbarDragged(id, dragged) => {
                let state = self.scrollbar_states.entry(id).or_default();
                state.set_dragged(dragged);
                Task::none()
            },
            Message::ScrollbarTick => {
                // 更新所有滚动条的淡出状态
                for state in self.scrollbar_states.values_mut() {
                    if state.should_auto_fade() && !state.should_fade {
                        state.should_fade = true;
                    }
                }
                Task::none()
            },

            // 其他消息
            _ => Task::none(), // 默认情况下不做任何操作
        }
    }

    // 辅助方法：获取当前选中的论文
    pub fn get_current_paper(&self) -> Option<ArxivPaper> {
        if let Some(tab) = self.tabs.get(self.active_tab) {
            match &tab.content {
                TabContent::Search => {
                    // 在搜索结果中找到选中的论文（这里简化为返回第一个）
                    self.search_results.first().cloned()
                }
                TabContent::Library => {
                    // 在论文库中找到选中的论文（这里简化为返回第一个）
                    self.saved_papers.first().cloned()
                }
                _ => None,
            }
        } else {
            None
        }
    }

    // 订阅和主题方法保持不变
    pub fn subscription(&self) -> Subscription<Message> {
        use iced::time;
        use std::time::Duration;
        
        // 批量订阅：窗口事件 + 滚动条淡出定时器
        Subscription::batch([
            // 订阅窗口事件以获取尺寸变化
            iced::event::listen().map(|event| {
                match event {
                    iced::Event::Window(iced::window::Event::Resized(size)) => {
                        Message::WindowResized { 
                            width: size.width, 
                            height: size.height 
                        }
                    },
                    _ => Message::NoOp,
                }
            }),
            
            // 滚动条自动淡出定时器 - 每250毫秒检查一次
            time::every(Duration::from_millis(250)).map(|_| Message::ScrollbarTick)
        ])
    }

    pub fn theme(&self) -> iced::Theme {
        use crate::core::models::Theme as AppTheme;
        
        match self.settings.theme {
            // 现代主题
            AppTheme::ModernDark => iced::Theme::Dark,
            AppTheme::ModernLight => iced::Theme::Light,
            
            // Gruvbox 主题系列
            AppTheme::GruvboxDark => iced::Theme::GruvboxDark,
            AppTheme::GruvboxLight => iced::Theme::GruvboxLight,
            AppTheme::GruvboxMaterial => iced::Theme::GruvboxDark,
            
            // Catppuccin 主题系列
            AppTheme::CatppuccinMocha => iced::Theme::Dark,
            AppTheme::CatppuccinMacchiato => iced::Theme::Dark,
            AppTheme::CatppuccinFrappe => iced::Theme::Dark,
            AppTheme::CatppuccinLatte => iced::Theme::Light,
            
            // Solarized 主题系列
            AppTheme::SolarizedDark => iced::Theme::Dark,
            AppTheme::SolarizedLight => iced::Theme::Light,
            
            // 其他暗色主题
            AppTheme::Dracula => iced::Theme::Dark,
            AppTheme::Nord => iced::Theme::Dark,
            AppTheme::OneDark => iced::Theme::Dark,
            AppTheme::GitHubDark => iced::Theme::Dark,
            AppTheme::TokyoNight => iced::Theme::Dark,
            AppTheme::AyuDark => iced::Theme::Dark,
            AppTheme::AyuMirage => iced::Theme::Dark,
            
            // 浅色主题
            AppTheme::NordLight => iced::Theme::Light,
            AppTheme::OneLight => iced::Theme::Light,
            AppTheme::GitHubLight => iced::Theme::Light,
            AppTheme::TokyoNightLight => iced::Theme::Light,
            AppTheme::AyuLight => iced::Theme::Light,
            
            // 经典主题
            AppTheme::Dark => iced::Theme::Dark,
            AppTheme::Light => iced::Theme::Light,
        }
    }
    
    /// 获取当前应用主题的颜色
    pub fn theme_colors(&self) -> crate::ui::theme::ThemeColors {
        crate::ui::theme::get_theme_colors(&self.settings.theme)
    }
    
    /// 获取当前字体设置
    pub fn current_font(&self) -> iced::Font {
        let font = match self.settings.font_family.as_str() {
            "Nerd Font" => iced::Font::with_name("JetBrainsMono Nerd Font"),
            "System Default" => iced::Font::default(),
            "Arial" => iced::Font::with_name("Arial"),
            "Times New Roman" => iced::Font::with_name("Times New Roman"),
            "Helvetica" => iced::Font::with_name("Helvetica"),
            "Georgia" => iced::Font::with_name("Georgia"),
            "Verdana" => iced::Font::with_name("Verdana"),
            "Segoe UI" => iced::Font::with_name("Segoe UI"),
            "Calibri" => iced::Font::with_name("Calibri"),
            "Cambria" => iced::Font::with_name("Cambria"),
            "Consolas" => iced::Font::with_name("Consolas"),
            "Courier New" => iced::Font::with_name("Courier New"),
            "Tahoma" => iced::Font::with_name("Tahoma"),
            "Trebuchet MS" => iced::Font::with_name("Trebuchet MS"),
            _ => iced::Font::default(),
        };
        
        font
    }
    
    /// 获取当前字体大小 (应用缩放)
    pub fn current_font_size(&self) -> f32 {
        self.settings.font_size * self.settings.ui_scale
    }
    
    /// 获取当前UI缩放比例
    pub fn current_scale(&self) -> f32 {
        self.settings.ui_scale
    }

    // 搜索历史和建议管理
    pub fn add_to_search_history(&mut self, query: String) {
        if query.trim().is_empty() {
            return;
        }
        
        // 移除重复项
        self.search_history.retain(|q| q != &query);
        
        // 添加到开头
        self.search_history.insert(0, query);
        
        // 限制历史记录数量
        if self.search_history.len() > 20 {
            self.search_history.truncate(20);
        }
    }

    pub fn update_search_suggestions(&mut self, current_query: &str) {
        if current_query.trim().is_empty() {
            self.search_suggestions.clear();
            return;
        }
        
        let query_lower = current_query.to_lowercase();
        let mut suggestions = Vec::new();
        
        // 检查是否是作者搜索模式
        let is_author_search = query_lower.starts_with("author:") || 
                              self.is_likely_author_name(current_query);
        
        if is_author_search {
            // 作者搜索建议
            let author_name = if query_lower.starts_with("author:") {
                current_query[7..].trim()
            } else {
                current_query.trim()
            };
            
            if !author_name.is_empty() {
                suggestions.push(format!("Search by author: {}", author_name));
            }
            
            // 一些知名理论物理学家建议
            let famous_authors = vec![
                "Edward Witten",
                "Juan Maldacena", 
                "Stephen Hawking",
                "Roger Penrose",
                "Nima Arkani-Hamed",
                "Lisa Randall",
                "Brian Greene",
                "Leonard Susskind",
                "Joseph Polchinski",
                "Nathan Seiberg",
                "Cumrun Vafa",
                "Michael Green",
                "Joel Scherk",
                "Gary Horowitz",
                "Andy Strominger",
                "Rafael Sorkin",
                "Abhay Ashtekar",
                "Carlo Rovelli",
                "Lee Smolin",
                "Kirill Krasnov",
            ];
            
            for author in famous_authors {
                if author.to_lowercase().contains(&author_name.to_lowercase()) 
                    && author != author_name {
                    suggestions.push(format!("Search by author: {}", author));
                }
            }
        } else {
            // 从搜索历史中匹配
            for history_item in &self.search_history {
                if history_item.to_lowercase().contains(&query_lower) && history_item != current_query {
                    suggestions.push(history_item.clone());
                }
            }
            
            // 添加一些常见的搜索建议（理论物理学为主）
            let common_suggestions = vec![
                "string theory",
                "quantum field theory",
                "general relativity", 
                "black holes",
                "supersymmetry",
                "conformal field theory",
                "AdS/CFT",
                "quantum gravity",
                "gauge theory",
                "holographic duality",
                "mathematical physics",
                "integrable systems",
                "topological field theory",
                "cosmology",
                "dark matter",
                "quantum mechanics",
                "algebraic geometry",
                "differential geometry",
                "lie algebras",
                "representation theory",
            ];
            
            for suggestion in common_suggestions {
                if suggestion.to_lowercase().contains(&query_lower) 
                    && !suggestions.contains(&suggestion.to_string()) 
                    && suggestion != current_query {
                    suggestions.push(suggestion.to_string());
                }
            }
        }
        
        // 限制建议数量
        suggestions.truncate(5);
        self.search_suggestions = suggestions;
    }

    /// 判断是否可能是作者姓名
    fn is_likely_author_name(&self, query: &str) -> bool {
        let query = query.trim();
        
        // 简单的启发式规则判断是否是作者姓名
        // 1. 包含空格（姓名分开）
        // 2. 首字母大写
        // 3. 不包含常见的技术术语
        if query.contains(' ') {
            let parts: Vec<&str> = query.split_whitespace().collect();
            if parts.len() >= 2 && parts.len() <= 4 {
                // 检查每个部分是否以大写字母开头
                let all_capitalized = parts.iter().all(|part| {
                    part.chars().next().map_or(false, |c| c.is_uppercase())
                });
                
                // 检查是否不包含技术术语
                let tech_terms = vec![
                    "learning", "network", "algorithm", "deep", "neural", 
                    "quantum", "computer", "artificial", "machine", "data"
                ];
                let no_tech_terms = !tech_terms.iter().any(|term| {
                    query.to_lowercase().contains(term)
                });
                
                return all_capitalized && no_tech_terms;
            }
        }
        
        false
    }
    
    // ===================
    // 性能优化相关方法
    // ===================
    
    /// 计算查询相似度（简单的字符串距离算法）
    pub fn calculate_query_similarity(&self, query1: &str, query2: &str) -> f64 {
        let q1 = query1.to_lowercase();
        let q2 = query2.to_lowercase();
        
        if q1 == q2 {
            return 1.0;
        }
        
        // 使用简单的Jaccard相似度
        let words1: std::collections::HashSet<&str> = q1.split_whitespace().collect();
        let words2: std::collections::HashSet<&str> = q2.split_whitespace().collect();
        
        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();
        
        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }
    
    /// 查找相似的已缓存查询
    pub fn find_similar_cached_queries(&self, query: &str, threshold: f64) -> Vec<String> {
        let mut similar_queries = Vec::new();
        
        for cached_query in self.search_cache.keys() {
            let similarity = self.calculate_query_similarity(query, cached_query);
            if similarity >= threshold && similarity < 1.0 { // 不包括完全相同的查询
                similar_queries.push(cached_query.clone());
            }
        }
        
        // 按相似度排序
        similar_queries.sort_by(|a, b| {
            let sim_a = self.calculate_query_similarity(query, a);
            let sim_b = self.calculate_query_similarity(query, b);
            sim_b.partial_cmp(&sim_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        similar_queries
    }
    
    /// 更新查询频率统计
    pub fn update_query_frequency(&mut self, query: &str) {
        let normalized_query = query.trim().to_lowercase();
        *self.query_frequency.entry(normalized_query).or_insert(0) += 1;
    }
    
    /// 获取热门查询
    pub fn get_popular_queries(&self, limit: usize) -> Vec<String> {
        let mut queries: Vec<_> = self.query_frequency.iter().collect();
        queries.sort_by(|a, b| b.1.cmp(a.1));
        queries.into_iter()
            .take(limit)
            .map(|(query, _)| query.clone())
            .collect()
    }
    
    /// 预测下一个可能的查询
    pub fn predict_next_queries(&self, current_query: &str) -> Vec<String> {
        let mut predictions = Vec::new();
        
        // 基于历史查询模式预测
        for history_query in &self.search_history {
            if history_query.starts_with(current_query) && history_query != current_query {
                predictions.push(history_query.clone());
            }
        }
        
        // 基于相似查询预测
        let similar_queries = self.find_similar_cached_queries(current_query, 0.6);
        predictions.extend(similar_queries);
        
        // 去重并限制数量
        predictions.sort();
        predictions.dedup();
        predictions.truncate(5);
        
        predictions
    }
    
    /// 更新搜索性能统计
    pub fn update_search_performance(&mut self, duration: std::time::Duration, was_cache_hit: bool) {
        self.search_performance_stats.total_searches += 1;
        if was_cache_hit {
            self.search_performance_stats.cache_hits += 1;
        }
        
        self.search_performance_stats.last_search_duration = Some(duration);
        
        // 更新平均响应时间（移动平均）
        let new_time = duration.as_millis() as f64;
        let current_avg = self.search_performance_stats.average_response_time;
        let total = self.search_performance_stats.total_searches as f64;
        
        self.search_performance_stats.average_response_time = 
            (current_avg * (total - 1.0) + new_time) / total;
    }
    
    /// 获取缓存命中率
    pub fn get_cache_hit_rate(&self) -> f64 {
        if self.search_performance_stats.total_searches == 0 {
            0.0
        } else {
            self.search_performance_stats.cache_hits as f64 / 
            self.search_performance_stats.total_searches as f64
        }
    }
    
    /// 智能缓存清理（保留热门和最近的查询）
    pub fn smart_cache_cleanup(&mut self) {
        const MAX_CACHE_SIZE: usize = 100;
        const _MIN_FREQUENCY_THRESHOLD: u32 = 2;
        
        if self.search_cache.len() <= MAX_CACHE_SIZE {
            return;
        }
        
        let now = Instant::now();
        let mut cache_scores: Vec<(String, f64)> = Vec::new();
        
        for (query, cache_item) in &self.search_cache {
            let age_seconds = now.duration_since(cache_item.timestamp).as_secs() as f64;
            let frequency = self.query_frequency.get(query).unwrap_or(&1);
            
            // 计算缓存项的重要性分数（频率高、时间新的分数高）
            let score = (*frequency as f64) / (1.0 + age_seconds / 3600.0); // 以小时为单位的衰减
            cache_scores.push((query.clone(), score));
        }
        
        // 按分数排序，保留分数高的
        cache_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        let keep_queries: std::collections::HashSet<String> = cache_scores
            .into_iter()
            .take(MAX_CACHE_SIZE * 3 / 4) // 保留75%的空间
            .map(|(query, _)| query)
            .collect();
        
        self.search_cache.retain(|query, _| keep_queries.contains(query));
    }
    
    /// 获取滚动条状态
    pub fn get_scrollbar_state(&self, id: &str) -> Option<&ScrollbarState> {
        self.scrollbar_states.get(id)
    }
    
    /// 获取滚动条透明度
    pub fn get_scrollbar_alpha(&self, id: &str) -> f32 {
        self.scrollbar_states
            .get(id)
            .map(|state| state.get_alpha())
            .unwrap_or(0.4) // 默认透明度
    }
    
    // PDF查看器相关方法
    
    /// 处理PDF查看器消息
    fn handle_pdf_viewer_message(&mut self, tab_id: String, pdf_msg: PdfViewerMessage) -> Task<Message> {
        println!("Handling PDF viewer message for tab_id: {}, message: {:?}", tab_id, pdf_msg);
        // 查找对应的PDF查看器并处理消息
        if let Some(pdf_viewer) = self.pdf_viewers.get_mut(&tab_id) {
            println!("Found PDF viewer for tab_id: {}", tab_id);
            // 调用PDF查看器的update方法处理消息
            if let Some(message) = pdf_viewer.update(pdf_msg) {
                return Task::done(message);
            }
        } else {
            println!("No PDF viewer found for tab_id: {}", tab_id);
        }
        
        Task::none()
    }
    
    /// 打开PDF文件
    fn handle_open_pdf_file(&mut self, path: std::path::PathBuf) -> Task<Message> {
        println!("Opening PDF file: {:?}", path);
        
        // 检查是否已经有这个PDF的标签页
        if let Some(tab_index) = self.tabs.iter().position(|tab| {
            matches!(&tab.content, TabContent::PdfViewer(p) if p == &path)
        }) {
            // 如果已存在，直接切换到该标签页
            println!("PDF viewer already exists, switching to tab {}", tab_index);
            self.active_tab = tab_index;
            return Task::none();
        }
        
        // 创建PDF查看器ID (使用当前的next_tab_id)
        let pdf_id = format!("pdf_{}", self.next_tab_id);
        println!("Creating PDF viewer with ID: {}", pdf_id);
        
        // 创建新的PDF查看器实例
        let pdf_viewer = PdfViewer::new(pdf_id.clone(), path.to_string_lossy().to_string());
        self.pdf_viewers.insert(pdf_id.clone(), pdf_viewer);
        
        // 创建新标签页 (这会递增 next_tab_id)
        let content = TabContent::PdfViewer(path.clone());
        self.handle_new_tab(content);
        
        // 返回打开PDF文件的消息
        let message = Message::PdfViewer(pdf_id.clone(), PdfViewerMessage::OpenFile(path.clone()));
        println!("Sending PDF viewer message: {:?}", message);
        
        Task::done(message)
    }
    
    /// 处理打开或下载PDF的操作
    fn handle_open_or_download_pdf(&mut self, paper: ArxivPaper) -> Task<Message> {
        // 检查是否已有本地文件
        if let Some(ref local_path) = paper.local_file_path {
            // 如果有本地文件，直接打开
            let pdf_path = std::path::PathBuf::from(local_path);
            return self.handle_open_pdf_file(pdf_path);
        } else {
            // 如果没有本地文件，先下载再打开
            // 检查是否已经在下载队列中
            if self.downloads.iter().any(|d| d.paper_id == paper.id) {
                return Task::none();
            }
            
            // 添加到下载队列并设置自动打开标志
            let download_item = DownloadItem {
                paper_id: paper.id.clone(),
                title: paper.title.clone(),
                progress: 0.0,
                status: DownloadStatus::Pending,
                file_path: None,
                auto_open_after_download: true, // 设置自动打开标志
            };
            
            self.downloads.push(download_item);
            
            // 启动实际的下载任务
            let paper_id = paper.id.clone();
            let pdf_url = if paper.pdf_url.starts_with("http") {
                paper.pdf_url.clone()
            } else {
                // 构建正确的arXiv PDF URL
                if paper.id.contains("v") {
                    // 如果ID已经包含版本号，直接使用
                    format!("https://arxiv.org/pdf/{}.pdf", paper.id)
                } else {
                    // 如果没有版本号，添加默认版本
                    format!("https://arxiv.org/pdf/{}v1.pdf", paper.id)
                }
            };
            
            println!("Starting download for paper: {} from {}", paper.title, pdf_url);
            
            // 返回异步下载任务
            return Task::perform(
                async move {
                    // 这里调用异步下载函数
                    use std::fs;
                    use std::io::Write;
                    
                    // 创建下载目录
                    let downloads_dir = std::env::current_dir()
                        .map_err(|e| (paper_id.clone(), format!("Failed to get current directory: {}", e)))?
                        .join("downloads");
                    
                    if !downloads_dir.exists() {
                        fs::create_dir_all(&downloads_dir)
                            .map_err(|e| (paper_id.clone(), format!("Failed to create downloads directory: {}", e)))?;
                    }
                    
                    // 构建文件路径
                    let file_name = format!("{}.pdf", paper_id);
                    let file_path = downloads_dir.join(&file_name);
                    
                    // 如果文件已存在，直接返回
                    if file_path.exists() {
                        return Ok((paper_id, file_path));
                    }
                    
                    // 下载文件
                    println!("Downloading PDF from: {}", pdf_url);
                    
                    let client = reqwest::Client::builder()
                        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
                        .timeout(std::time::Duration::from_secs(60))
                        .build()
                        .map_err(|e| (paper_id.clone(), format!("Failed to create HTTP client: {}", e)))?;
                    
                    let response = client.get(&pdf_url).send().await
                        .map_err(|e| (paper_id.clone(), format!("Failed to download PDF: {}", e)))?;
                    
                    println!("HTTP response status: {}", response.status());
                    
                    if !response.status().is_success() {
                        return Err((paper_id, format!("HTTP error: {} - {}", response.status(), response.status().canonical_reason().unwrap_or("Unknown error"))));
                    }
                    
                    let content_length = response.content_length();
                    println!("Content length: {:?}", content_length);
                    
                    let content = response.bytes().await
                        .map_err(|e| (paper_id.clone(), format!("Failed to read response: {}", e)))?;
                    
                    println!("Downloaded {} bytes", content.len());
                    
                    // 检查是否为有效的PDF文件
                    if content.len() < 4 || !content.starts_with(b"%PDF") {
                        return Err((paper_id, format!("Downloaded content is not a valid PDF file (size: {} bytes)", content.len())));
                    }
                    
                    // 保存文件
                    let mut file = fs::File::create(&file_path)
                        .map_err(|e| (paper_id.clone(), format!("Failed to create file: {}", e)))?;
                    
                    file.write_all(&content)
                        .map_err(|e| (paper_id.clone(), format!("Failed to write file: {}", e)))?;
                    
                    println!("PDF downloaded successfully: {:?}", file_path);
                    Ok((paper_id, file_path))
                },
                |result| match result {
                    Ok((paper_id, file_path)) => Message::DownloadCompleted { paper_id, file_path },
                    Err((paper_id, error)) => Message::DownloadFailed { paper_id, error },
                }
            );
        }
    }
}

// 为ArxivManager实现Drop trait，在应用程序退出时自动保存会话
impl Drop for ArxivManager {
    fn drop(&mut self) {
        // 自动保存会话状态
        if let Err(e) = self.handle_save_session_internal() {
            eprintln!("Failed to save session on exit: {}", e);
        } else {
            println!("Session saved successfully on exit");
        }
    }
}

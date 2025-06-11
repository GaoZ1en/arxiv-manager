// 重构后的应用状态管理 - 模块化版本
// 使用处理器模式将巨大的update方法分解为专门的处理器

use std::time::Instant;
use iced::{Task, Subscription};

use crate::core::{ArxivPaper, DownloadItem, SearchConfig, AppSettings, Tab, TabContent};
use crate::core::messages::{Message, Command};

// 导入所有处理器
use crate::core::handlers::{
    TabHandler, SearchHandler, DownloadHandler, SettingsHandler, 
    CommandHandler, PaperHandler, ShortcutHandler
};

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
    pub is_searching: bool,
    pub search_error: Option<String>,
    pub last_interaction: Option<Instant>,
    pub settings: AppSettings,
    // 命令栏状态
    pub command_palette_visible: bool,
    pub command_palette_input: String,
    pub command_suggestions: Vec<Command>,
    pub selected_command_index: Option<usize>,
    // 快捷键编辑状态
    pub editing_shortcut: Option<String>, // 正在编辑的快捷键动作
    pub shortcut_input: String,           // 快捷键输入缓存
}

impl ArxivManager {
    pub fn new() -> (Self, Task<Message>) {
        // 创建默认标签页
        let mut tabs = Vec::new();
        tabs.push(Tab::new(0, "搜索".to_string(), TabContent::Search));
        tabs.push(Tab::new(1, "论文库".to_string(), TabContent::Library));
        tabs.push(Tab::new(2, "下载".to_string(), TabContent::Downloads));
        tabs.push(Tab::new(3, "设置".to_string(), TabContent::Settings));

        let manager = Self {
            tabs,
            active_tab: 0,
            next_tab_id: 4,
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
            // 命令栏初始化
            command_palette_visible: false,
            command_palette_input: String::new(),
            command_suggestions: Vec::new(),
            selected_command_index: None,
            // 快捷键编辑状态初始化
            editing_shortcut: None,
            shortcut_input: String::new(),
        };

        (manager, Task::none())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // 标签页相关消息 - 委托给 TabHandler
            Message::TabClicked(tab_index) => self.handle_tab_clicked(tab_index),
            Message::TabClose(tab_index) => self.handle_tab_close(tab_index),
            Message::NavigateToNextTab => self.handle_navigate_to_next_tab(),
            Message::NavigateToPreviousTab => self.handle_navigate_to_previous_tab(),
            Message::CloseActiveTab => self.handle_close_active_tab(),
            Message::NewTab(content) => self.handle_new_tab(content),

            // 搜索相关消息 - 委托给 SearchHandler
            Message::SearchQueryChanged(query) => self.handle_search_query_changed(query),
            Message::SearchSubmitted => self.handle_search_submitted(),
            Message::SearchCompleted(result) => self.handle_search_completed(result),
            Message::AdvancedSearchToggled => self.handle_advanced_search_toggled(),
            Message::SearchFieldChanged(field) => self.handle_search_field_changed(field),
            Message::CategoryToggled(category) => self.handle_category_toggled(category),
            Message::DateRangeChanged(range) => self.handle_date_range_changed(range),
            Message::SortByChanged(sort_by) => self.handle_sort_by_changed(sort_by),
            Message::SortOrderChanged(order) => self.handle_sort_order_changed(order),
            Message::MaxResultsChanged(value) => self.handle_max_results_changed(value),
            Message::AuthorAdded(author) => self.handle_author_added(author),
            Message::AuthorRemoved(index) => self.handle_author_removed(index),

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
        // 暂时返回空订阅，因为KeyPressed消息在当前枚举中不存在
        Subscription::none()
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
}

// 重构后的应用状态管理 - 模块化版本
// 使用处理器模式将巨大的update方法分解为专门的处理器

use std::time::Instant;
use std::collections::HashMap;
use iced::{Task, Subscription};

use crate::core::{ArxivPaper, DownloadItem, SearchConfig, AppSettings, Tab, TabContent};
use crate::core::messages::{Message, Command};

// 导入所有处理器
use crate::core::handlers::{
    TabHandler, SearchHandler, DownloadHandler, SettingsHandler, 
    CommandHandler, PaperHandler, ShortcutHandler
};

// 搜索缓存项
#[derive(Debug, Clone)]
pub struct SearchCacheItem {
    pub results: Vec<ArxivPaper>,
    pub timestamp: Instant,
    pub config: SearchConfig,
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
    
    /// 获取当前字体设置 (支持emoji)
    pub fn current_font(&self) -> iced::Font {
        match self.settings.font_family.as_str() {
            "系统默认" => iced::Font::default(),
            "Arial" => iced::Font::with_name("Arial"),
            "Times New Roman" => iced::Font::with_name("Times New Roman"),
            "Helvetica" => iced::Font::with_name("Helvetica"),
            "Georgia" => iced::Font::with_name("Georgia"),
            "Verdana" => iced::Font::with_name("Verdana"),
            "Segoe UI" => iced::Font::with_name("Segoe UI"),
            "Calibri" => iced::Font::with_name("Calibri"),
            "Cambria" => iced::Font::with_name("Cambria"),
            "Consolas" => iced::Font::with_name("Consolas"),
            "微软雅黑" => iced::Font::with_name("Microsoft YaHei"),
            "宋体" => iced::Font::with_name("SimSun"),
            "黑体" => iced::Font::with_name("SimHei"),
            "楷体" => iced::Font::with_name("KaiTi"),
            "仿宋" => iced::Font::with_name("FangSong"),
            // 支持emoji的字体
            "Noto Color Emoji" => iced::Font::with_name("Noto Color Emoji"),
            "Apple Color Emoji" => iced::Font::with_name("Apple Color Emoji"),
            "Segoe UI Emoji" => iced::Font::with_name("Segoe UI Emoji"),
            "EmojiOne Color" => iced::Font::with_name("EmojiOne Color"),
            _ => iced::Font::default(),
        }
    }
    
    /// 获取emoji字体 (用于fallback)
    pub fn emoji_font(&self) -> iced::Font {
        // 根据操作系统选择合适的emoji字体
        #[cfg(target_os = "windows")]
        return iced::Font::with_name("Segoe UI Emoji");
        
        #[cfg(target_os = "macos")]
        return iced::Font::with_name("Apple Color Emoji");
        
        #[cfg(target_os = "linux")]
        return iced::Font::with_name("Noto Color Emoji");
        
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        return iced::Font::default();
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
                              query_lower.starts_with("作者:") ||
                              self.is_likely_author_name(current_query);
        
        if is_author_search {
            // 作者搜索建议
            let author_name = if query_lower.starts_with("author:") {
                current_query[7..].trim()
            } else if query_lower.starts_with("作者:") {
                current_query[6..].trim()
            } else {
                current_query.trim()
            };
            
            if !author_name.is_empty() {
                suggestions.push(format!("按作者搜索: {}", author_name));
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
                    suggestions.push(format!("按作者搜索: {}", author));
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
}

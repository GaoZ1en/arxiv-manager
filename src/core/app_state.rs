// 重构后的应用状态管理 - 模块化版本
// 使用处理器模式将巨大的update方法分解为专门的处理器

use std::time::Instant;
use std::collections::HashMap;
use iced::{Task, Subscription};

use crate::core::{ArxivPaper, DownloadItem, SearchConfig, AppSettings, Tab, TabContent, SessionManager};
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
            Message::TabPin(tab_index) => self.handle_tab_pin(tab_index),
            Message::TabUnpin(tab_index) => self.handle_tab_unpin(tab_index),
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

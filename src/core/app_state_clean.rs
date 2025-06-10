// 应用状态管理 - 重构版本
// 将巨大的update方法分解为专门的处理器模块

use std::time::Instant;
use iced::{Task, Subscription, Event};
use iced::keyboard::{Key, Modifiers};

use crate::core::{ArxivPaper, DownloadItem, DownloadStatus, SearchConfig, AppSettings, Tab, TabContent};
use crate::core::messages::{Message, Command};
use crate::search::services::{search_arxiv_papers_advanced, download_pdf};

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
            // 标签页相关消息
            Message::TabClicked(tab_index) => self.handle_tab_clicked(tab_index),
            Message::TabClose(tab_index) => self.handle_tab_close(tab_index),
            Message::NavigateToNextTab => self.handle_navigate_to_next_tab(),
            Message::NavigateToPreviousTab => self.handle_navigate_to_previous_tab(),
            Message::CloseActiveTab => self.handle_close_active_tab(),
            Message::NewTab(content) => self.handle_new_tab(content),

            // 搜索相关消息
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

            // 论文相关消息
            Message::SavePaper(paper) => self.handle_paper_save(paper),
            Message::RemovePaper(paper_id) => self.handle_paper_remove_by_id(paper_id),

            // 下载相关消息
            Message::DownloadPaper(paper) => self.handle_download_paper(paper),
            Message::DownloadProgress { paper_id, progress } => self.handle_download_progress(paper_id, progress),
            Message::DownloadCompleted { paper_id, file_path } => self.handle_download_completed(paper_id, file_path.to_string_lossy().to_string()),
            Message::DownloadFailed { paper_id, error } => self.handle_download_failed(paper_id, error),

            // 设置相关消息
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
            Message::ImportSettings => self.handle_settings_import(),

            // 命令面板相关消息
            Message::ToggleCommandPalette => self.handle_command_palette_toggled(),
            Message::CommandPaletteInputChanged(input) => self.handle_command_input_changed(input),
            Message::CommandSelected(index) => self.handle_command_selected(index),
            Message::ExecuteCommand(command) => self.handle_command_executed(command),
            Message::SidebarToggled => self.handle_sidebar_toggled(),

            // 快捷键相关消息
            Message::ShortcutEditStarted(action) => self.handle_shortcut_edit_started(action),
            Message::ShortcutEditCancelled => self.handle_shortcut_edit_cancelled(),
            Message::ShortcutInputChanged(input) => self.handle_shortcut_input_changed(input),
            Message::ResetShortcuts => self.handle_shortcuts_reset(),

            // 其他消息
            _ => Task::none(), // 暂时忽略其他消息
        }
    }

    // ======== 标签页处理方法 ========
    fn handle_tab_clicked(&mut self, tab_index: usize) -> Task<Message> {
        if tab_index < self.tabs.len() {
            self.active_tab = tab_index;
            self.last_interaction = Some(Instant::now());
        }
        Task::none()
    }

    fn handle_tab_close(&mut self, tab_index: usize) -> Task<Message> {
        if tab_index < self.tabs.len() && self.tabs[tab_index].closable {
            self.tabs.remove(tab_index);
            // 调整活动标签页索引
            if self.active_tab >= self.tabs.len() && !self.tabs.is_empty() {
                self.active_tab = self.tabs.len() - 1;
            }
        }
        Task::none()
    }

    fn handle_navigate_to_next_tab(&mut self) -> Task<Message> {
        self.navigate_to_next_tab();
        Task::none()
    }

    fn handle_navigate_to_previous_tab(&mut self) -> Task<Message> {
        self.navigate_to_previous_tab();
        Task::none()
    }

    fn handle_close_active_tab(&mut self) -> Task<Message> {
        if self.active_tab < self.tabs.len() && self.tabs[self.active_tab].closable {
            self.update(Message::TabClose(self.active_tab))
        } else {
            Task::none()
        }
    }

    fn handle_new_tab(&mut self, content: TabContent) -> Task<Message> {
        let title = match &content {
            TabContent::Search => "搜索".to_string(),
            TabContent::Library => "论文库".to_string(),
            TabContent::Downloads => "下载".to_string(),
            TabContent::Settings => "设置".to_string(),
            TabContent::PaperView(index) => {
                if let Some(paper) = self.saved_papers.get(*index) {
                    format!("论文: {}", paper.title)
                } else {
                    "论文详情".to_string()
                }
            }
        };
        
        let new_tab = Tab::new(self.next_tab_id, title, content);
        self.tabs.push(new_tab);
        self.active_tab = self.tabs.len() - 1;
        self.next_tab_id += 1;
        
        Task::none()
    }

    fn navigate_to_next_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active_tab = (self.active_tab + 1) % self.tabs.len();
        }
    }

    fn navigate_to_previous_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active_tab = if self.active_tab == 0 {
                self.tabs.len() - 1
            } else {
                self.active_tab - 1
            };
        }
    }

    // ======== 搜索处理方法 ========
    fn handle_search_query_changed(&mut self, query: String) -> Task<Message> {
        self.search_query = query.clone();
        self.search_config.query = query;
        Task::none()
    }

    fn handle_search_submitted(&mut self) -> Task<Message> {
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

    fn handle_search_completed(&mut self, result: Result<Vec<ArxivPaper>, String>) -> Task<Message> {
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

    fn handle_advanced_search_toggled(&mut self) -> Task<Message> {
        self.advanced_search_visible = !self.advanced_search_visible;
        Task::none()
    }

    fn handle_search_field_changed(&mut self, field: crate::core::models::SearchField) -> Task<Message> {
        self.search_config.search_in = field;
        Task::none()
    }

    fn handle_category_toggled(&mut self, category: String) -> Task<Message> {
        if let Some(pos) = self.search_config.categories.iter().position(|x| x == &category) {
            self.search_config.categories.remove(pos);
        } else {
            self.search_config.categories.push(category);
        }
        Task::none()
    }

    fn handle_date_range_changed(&mut self, range: crate::core::models::DateRange) -> Task<Message> {
        self.search_config.date_range = range;
        Task::none()
    }

    fn handle_sort_by_changed(&mut self, sort_by: crate::core::models::SortBy) -> Task<Message> {
        self.search_config.sort_by = sort_by;
        Task::none()
    }

    fn handle_sort_order_changed(&mut self, order: crate::core::models::SortOrder) -> Task<Message> {
        self.search_config.sort_order = order;
        Task::none()
    }

    fn handle_max_results_changed(&mut self, value: String) -> Task<Message> {
        if let Ok(num) = value.parse::<u32>() {
            self.search_config.max_results = num.min(100).max(1);
        }
        Task::none()
    }

    fn handle_author_added(&mut self, author: String) -> Task<Message> {
        if !author.trim().is_empty() && !self.search_config.authors.contains(&author) {
            self.search_config.authors.push(author);
        }
        Task::none()
    }

    fn handle_author_removed(&mut self, index: usize) -> Task<Message> {
        if index < self.search_config.authors.len() {
            self.search_config.authors.remove(index);
        }
        Task::none()
    }

    // ======== 论文处理方法 ========
    fn handle_paper_save(&mut self, paper: ArxivPaper) -> Task<Message> {
        // 检查是否已经保存
        if !self.saved_papers.iter().any(|p| p.id == paper.id) {
            self.saved_papers.push(paper);
        }
        Task::none()
    }

    fn handle_paper_remove_by_id(&mut self, paper_id: String) -> Task<Message> {
        if let Some(index) = self.saved_papers.iter().position(|p| p.id == paper_id) {
            self.saved_papers.remove(index);
            
            // 关闭所有引用该论文的标签页
            let mut tabs_to_close = Vec::new();
            for (i, tab) in self.tabs.iter().enumerate() {
                if let TabContent::PaperView(paper_index) = &tab.content {
                    if *paper_index == index {
                        tabs_to_close.push(i);
                    }
                }
            }
            
            // 从后往前关闭标签页，避免索引问题
            for &tab_index in tabs_to_close.iter().rev() {
                if tab_index < self.tabs.len() && self.tabs[tab_index].closable {
                    self.tabs.remove(tab_index);
                }
            }
            
            // 调整活动标签页索引
            if self.active_tab >= self.tabs.len() && !self.tabs.is_empty() {
                self.active_tab = self.tabs.len() - 1;
            }
        }
        Task::none()
    }

    // ======== 下载处理方法 ========
    fn handle_download_paper(&mut self, paper: ArxivPaper) -> Task<Message> {
        // 检查是否已经在下载队列中
        if self.downloads.iter().any(|d| d.paper_id == paper.id) {
            return Task::none();
        }
        
        // 添加到下载队列
        let download_item = DownloadItem {
            paper_id: paper.id.clone(),
            title: paper.title.clone(),
            progress: 0.0,
            status: DownloadStatus::Pending,
            file_path: None,
        };
        
        self.downloads.push(download_item);
        
        // TODO: 启动实际的下载任务
        println!("Starting download for paper: {}", paper.title);
        
        Task::none()
    }

    fn handle_download_progress(&mut self, paper_id: String, progress: f32) -> Task<Message> {
        if let Some(download) = self.downloads.iter_mut().find(|d| d.paper_id == paper_id) {
            download.progress = progress.clamp(0.0, 1.0);
            download.status = DownloadStatus::Downloading;
        }
        Task::none()
    }

    fn handle_download_completed(&mut self, paper_id: String, file_path: String) -> Task<Message> {
        if let Some(download) = self.downloads.iter_mut().find(|d| d.paper_id == paper_id) {
            download.progress = 1.0;
            download.status = DownloadStatus::Completed;
            download.file_path = Some(file_path.into());
        }
        Task::none()
    }

    fn handle_download_failed(&mut self, paper_id: String, error: String) -> Task<Message> {
        if let Some(download) = self.downloads.iter_mut().find(|d| d.paper_id == paper_id) {
            download.status = DownloadStatus::Failed(error);
        }
        Task::none()
    }

    // ======== 设置处理方法 ========
    fn handle_theme_changed(&mut self, theme: crate::core::models::Theme) -> Task<Message> {
        self.settings.theme = theme;
        Task::none()
    }

    fn handle_language_changed(&mut self, language: crate::core::models::Language) -> Task<Message> {
        self.settings.language = language;
        Task::none()
    }

    fn handle_download_directory_changed(&mut self, path: String) -> Task<Message> {
        self.settings.download_directory = path;
        Task::none()
    }

    fn handle_auto_download_toggled(&mut self) -> Task<Message> {
        self.settings.auto_download = !self.settings.auto_download;
        Task::none()
    }

    fn handle_max_concurrent_downloads_changed(&mut self, value: String) -> Task<Message> {
        if let Ok(num) = value.parse::<u32>() {
            self.settings.max_concurrent_downloads = num.clamp(1, 10);
        }
        Task::none()
    }

    fn handle_show_abstracts_toggled(&mut self) -> Task<Message> {
        self.settings.show_abstracts_in_search = !self.settings.show_abstracts_in_search;
        Task::none()
    }

    fn handle_default_search_field_changed(&mut self, field: crate::core::models::SearchField) -> Task<Message> {
        self.settings.default_search_field = field;
        Task::none()
    }

    fn handle_default_sort_by_changed(&mut self, sort_by: crate::core::models::SortBy) -> Task<Message> {
        self.settings.default_sort_by = sort_by;
        Task::none()
    }

    fn handle_default_sort_order_changed(&mut self, order: crate::core::models::SortOrder) -> Task<Message> {
        self.settings.default_sort_order = order;
        Task::none()
    }

    fn handle_default_max_results_changed(&mut self, value: String) -> Task<Message> {
        if let Ok(num) = value.parse::<u32>() {
            self.settings.default_max_results = num.clamp(1, 100);
        }
        Task::none()
    }

    fn handle_auto_save_searches_toggled(&mut self) -> Task<Message> {
        self.settings.auto_save_searches = !self.settings.auto_save_searches;
        Task::none()
    }

    fn handle_notification_toggled(&mut self) -> Task<Message> {
        self.settings.notification_enabled = !self.settings.notification_enabled;
        Task::none()
    }

    fn handle_check_updates_toggled(&mut self) -> Task<Message> {
        self.settings.check_updates = !self.settings.check_updates;
        Task::none()
    }

    fn handle_settings_reset(&mut self) -> Task<Message> {
        use crate::core::models::AppSettings;
        self.settings = AppSettings::default();
        Task::none()
    }

    fn handle_settings_export(&mut self) -> Task<Message> {
        // TODO: 实现设置导出功能
        println!("Exporting settings...");
        Task::none()
    }

    fn handle_settings_import(&mut self) -> Task<Message> {
        // TODO: 实现设置导入功能
        println!("Importing settings...");
        Task::none()
    }

    // ======== 命令面板处理方法 ========
    fn handle_command_palette_toggled(&mut self) -> Task<Message> {
        self.command_palette_visible = !self.command_palette_visible;
        
        if self.command_palette_visible {
            self.command_palette_input.clear();
            self.update_command_suggestions();
            self.selected_command_index = if self.command_suggestions.is_empty() { 
                None 
            } else { 
                Some(0) 
            };
        } else {
            self.command_suggestions.clear();
            self.selected_command_index = None;
        }
        
        Task::none()
    }

    fn handle_command_input_changed(&mut self, input: String) -> Task<Message> {
        self.command_palette_input = input;
        self.update_command_suggestions();
        self.selected_command_index = if self.command_suggestions.is_empty() { 
            None 
        } else { 
            Some(0) 
        };
        Task::none()
    }

    fn handle_command_selected(&mut self, index: usize) -> Task<Message> {
        if index < self.command_suggestions.len() {
            self.selected_command_index = Some(index);
        }
        Task::none()
    }

    fn handle_command_executed(&mut self, command: Command) -> Task<Message> {
        self.command_palette_visible = false;
        self.command_palette_input.clear();
        self.command_suggestions.clear();
        self.selected_command_index = None;
        
        self.execute_command(&command)
    }

    fn handle_sidebar_toggled(&mut self) -> Task<Message> {
        self.sidebar_visible = !self.sidebar_visible;
        Task::none()
    }

    fn update_command_suggestions(&mut self) {
        let all_commands = vec![
            Command::ToggleSidebar,
            Command::NewSearch,
            Command::GoToLibrary,
            Command::GoToDownloads,
            Command::GoToSettings,
            Command::ToggleTheme,
            // Command::ExportPapers, // 暂时注释掉不存在的命令
            // Command::ClearSearchHistory,
            Command::ShowHelp,
        ];

        if self.command_palette_input.is_empty() {
            self.command_suggestions = all_commands;
        } else {
            let query = self.command_palette_input.to_lowercase();
            self.command_suggestions = all_commands
                .into_iter()
                .filter(|cmd| {
                    let cmd_text = format!("{:?}", cmd).to_lowercase();
                    cmd_text.contains(&query)
                })
                .collect();
        }
    }

    fn execute_command(&mut self, command: &Command) -> Task<Message> {
        match command {
            Command::ToggleSidebar => {
                self.sidebar_visible = !self.sidebar_visible;
                Task::none()
            }
            Command::NewSearch => {
                self.update(Message::NewTab(TabContent::Search))
            }
            Command::GoToLibrary => {
                // 查找或创建Library标签页
                if let Some(index) = self.tabs.iter().position(|tab| {
                    matches!(tab.content, TabContent::Library)
                }) {
                    self.active_tab = index;
                } else {
                    return self.update(Message::NewTab(TabContent::Library));
                }
                Task::none()
            }
            Command::GoToDownloads => {
                // 查找或创建Downloads标签页
                if let Some(index) = self.tabs.iter().position(|tab| {
                    matches!(tab.content, TabContent::Downloads)
                }) {
                    self.active_tab = index;
                } else {
                    return self.update(Message::NewTab(TabContent::Downloads));
                }
                Task::none()
            }
            Command::GoToSettings => {
                // 查找或创建Settings标签页
                if let Some(index) = self.tabs.iter().position(|tab| {
                    matches!(tab.content, TabContent::Settings)
                }) {
                    self.active_tab = index;
                } else {
                    return self.update(Message::NewTab(TabContent::Settings));
                }
                Task::none()
            }
            Command::ToggleTheme => {
                use crate::core::models::Theme;
                self.settings.theme = match self.settings.theme {
                    Theme::GruvboxDark => Theme::GruvboxLight,
                    Theme::GruvboxLight => Theme::Dark,
                    Theme::Dark => Theme::Light,
                    Theme::Light => Theme::GruvboxDark,
                };
                Task::none()
            }
            Command::ShowHelp => {
                // TODO: 打开帮助文档
                println!("Opening help...");
                Task::none()
            }
            _ => Task::none(),
        }
    }

    // ======== 快捷键处理方法 ========
    fn handle_shortcut_edit_started(&mut self, action: String) -> Task<Message> {
        self.editing_shortcut = Some(action.clone());
        
        // 预填充当前快捷键
        let current_shortcut = match action.as_str() {
            "toggle_command_palette" => &self.settings.shortcuts.toggle_command_palette.display,
            "focus_search" => &self.settings.shortcuts.focus_search.display,
            "quick_save_paper" => &self.settings.shortcuts.quick_save_paper.display,
            "quick_download_paper" => &self.settings.shortcuts.quick_download_paper.display,
            "toggle_sidebar" => &self.settings.shortcuts.toggle_sidebar.display,
            "next_tab" => &self.settings.shortcuts.next_tab.display,
            "previous_tab" => &self.settings.shortcuts.previous_tab.display,
            "close_tab" => &self.settings.shortcuts.close_tab.display,
            "new_tab" => &self.settings.shortcuts.new_tab.display,
            "go_to_search" => &self.settings.shortcuts.go_to_search.display,
            "go_to_library" => &self.settings.shortcuts.go_to_library.display,
            "go_to_downloads" => &self.settings.shortcuts.go_to_downloads.display,
            "go_to_settings" => &self.settings.shortcuts.go_to_settings.display,
            _ => "",
        };
        
        self.shortcut_input = current_shortcut.to_string();
        Task::none()
    }

    fn handle_shortcut_edit_cancelled(&mut self) -> Task<Message> {
        self.editing_shortcut = None;
        self.shortcut_input.clear();
        Task::none()
    }

    fn handle_shortcut_input_changed(&mut self, input: String) -> Task<Message> {
        self.shortcut_input = input;
        Task::none()
    }

    fn handle_shortcuts_reset(&mut self) -> Task<Message> {
        use crate::core::models::KeyboardShortcuts;
        self.settings.shortcuts = KeyboardShortcuts::default();
        Task::none()
    }

    fn update_shortcut(&mut self, action: &str, shortcut: &str) {
        use crate::core::models::ShortcutKey;
        
        let new_shortcut = ShortcutKey::new(shortcut);
        
        match action {
            "toggle_command_palette" => self.settings.shortcuts.toggle_command_palette = new_shortcut,
            "focus_search" => self.settings.shortcuts.focus_search = new_shortcut,
            "quick_save_paper" => self.settings.shortcuts.quick_save_paper = new_shortcut,
            "quick_download_paper" => self.settings.shortcuts.quick_download_paper = new_shortcut,
            "toggle_sidebar" => self.settings.shortcuts.toggle_sidebar = new_shortcut,
            "next_tab" => self.settings.shortcuts.next_tab = new_shortcut,
            "previous_tab" => self.settings.shortcuts.previous_tab = new_shortcut,
            "close_tab" => self.settings.shortcuts.close_tab = new_shortcut,
            "new_tab" => self.settings.shortcuts.new_tab = new_shortcut,
            "go_to_search" => self.settings.shortcuts.go_to_search = new_shortcut,
            "go_to_library" => self.settings.shortcuts.go_to_library = new_shortcut,
            "go_to_downloads" => self.settings.shortcuts.go_to_downloads = new_shortcut,
            "go_to_settings" => self.settings.shortcuts.go_to_settings = new_shortcut,
            _ => {}
        }
    }

    // ======== 辅助方法 ========
    fn get_current_paper(&self) -> Option<ArxivPaper> {
        // 从当前活动标签页获取论文
        if let Some(current_tab) = self.tabs.get(self.active_tab) {
            match &current_tab.content {
                TabContent::PaperView(index) => {
                    self.saved_papers.get(*index).cloned()
                }
                TabContent::Search => {
                    // 如果在搜索标签页，获取第一个搜索结果
                    self.search_results.first().cloned()
                }
                _ => None,
            }
        } else {
            None
        }
    }

    // 订阅和主题方法保持不变
    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none() // 先简化订阅
    }

    pub fn theme(&self) -> iced::Theme {
        match self.settings.theme {
            crate::core::models::Theme::GruvboxDark => iced::Theme::GruvboxDark,
            crate::core::models::Theme::GruvboxLight => iced::Theme::GruvboxLight,
            crate::core::models::Theme::Dark => iced::Theme::Dark,
            crate::core::models::Theme::Light => iced::Theme::Light,
        }
    }
}

// 重构后的应用状态管理 - 模块化版本
// 使用处理器模式将巨大的update方法分解为专门的处理器

use std::time::Instant;
use iced::{Task, Subscription, Event};
use iced::keyboard::{Key, Modifiers};

use crate::core::{ArxivPaper, DownloadItem, DownloadStatus, SearchConfig, AppSettings, Tab, TabContent};
use crate::core::messages::{Message, Command};
use crate::search::services::{search_arxiv_papers_advanced, download_pdf};

// 导入所有处理器
mod handlers;
use handlers::{
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
            Message::PaperSave(paper) => self.handle_paper_save(paper),
            Message::PaperRemove(index) => self.handle_paper_remove(index),
            Message::PaperView(paper) => self.handle_paper_view(paper),
            Message::PaperExport(format) => self.handle_paper_export(format),

            // 下载相关消息 - 委托给 DownloadHandler
            Message::DownloadPaper(paper) => self.handle_download_paper(paper),
            Message::DownloadCancel(paper_id) => self.handle_download_cancel(paper_id),
            Message::DownloadRetry(paper_id) => self.handle_download_retry(paper_id),
            Message::DownloadProgress(paper_id, progress) => self.handle_download_progress(paper_id, progress),
            Message::DownloadCompleted(paper_id, file_path) => self.handle_download_completed(paper_id, file_path),
            Message::DownloadFailed(paper_id, error) => self.handle_download_failed(paper_id, error),
            Message::DownloadClearCompleted => self.handle_download_clear_completed(),

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
            Message::SettingsReset => self.handle_settings_reset(),
            Message::SettingsExport => self.handle_settings_export(),
            Message::SettingsImport(path) => self.handle_settings_import(path),

            // 命令面板相关消息 - 委托给 CommandHandler
            Message::CommandPaletteToggled => self.handle_command_palette_toggled(),
            Message::CommandInputChanged(input) => self.handle_command_input_changed(input),
            Message::CommandSelected(index) => self.handle_command_selected(index),
            Message::CommandExecuted(command) => self.handle_command_executed(command),
            Message::SidebarToggled => self.handle_sidebar_toggled(),

            // 快捷键相关消息 - 委托给 ShortcutHandler
            Message::ShortcutEditStarted(action) => self.handle_shortcut_edit_started(action),
            Message::ShortcutEditCancelled => self.handle_shortcut_edit_cancelled(),
            Message::ShortcutInputChanged(input) => self.handle_shortcut_input_changed(input),
            Message::ShortcutConfirmed => self.handle_shortcut_confirmed(),
            Message::ShortcutsReset => self.handle_shortcuts_reset(),

            // 键盘输入处理
            Message::KeyPressed(key, modifiers) => self.handle_key_pressed(key, modifiers),
        }
    }

    // 键盘输入处理（保留在主文件中，因为它需要协调多个处理器）
    fn handle_key_pressed(&mut self, key: Key, modifiers: Modifiers) -> Task<Message> {
        // 构建当前按键组合的字符串表示
        let mut key_combo = Vec::new();
        
        if modifiers.control() {
            key_combo.push("Ctrl");
        }
        if modifiers.shift() {
            key_combo.push("Shift");
        }
        if modifiers.alt() {
            key_combo.push("Alt");
        }
        if modifiers.logo() {
            key_combo.push("Super");
        }
        
        let key_str = match key {
            Key::Character(c) => c.to_uppercase(),
            Key::Named(named_key) => format!("{:?}", named_key),
            _ => return Task::none(),
        };
        
        key_combo.push(&key_str);
        let current_shortcut = key_combo.join("+");
        
        // 检查是否匹配任何已配置的快捷键
        let shortcuts = &self.settings.shortcuts;
        
        if current_shortcut == shortcuts.toggle_command_palette.display {
            return self.update(Message::CommandPaletteToggled);
        }
        if current_shortcut == shortcuts.focus_search.display {
            // TODO: 聚焦搜索框
        }
        if current_shortcut == shortcuts.quick_save_paper.display {
            if let Some(paper) = self.get_current_paper() {
                return self.update(Message::PaperSave(paper));
            }
        }
        if current_shortcut == shortcuts.quick_download_paper.display {
            if let Some(paper) = self.get_current_paper() {
                return self.update(Message::DownloadPaper(paper));
            }
        }
        if current_shortcut == shortcuts.toggle_sidebar.display {
            return self.update(Message::SidebarToggled);
        }
        if current_shortcut == shortcuts.next_tab.display {
            return self.update(Message::NavigateToNextTab);
        }
        if current_shortcut == shortcuts.previous_tab.display {
            return self.update(Message::NavigateToPreviousTab);
        }
        if current_shortcut == shortcuts.close_tab.display {
            return self.update(Message::CloseActiveTab);
        }
        if current_shortcut == shortcuts.new_tab.display {
            return self.update(Message::NewTab(TabContent::Search));
        }
        if current_shortcut == shortcuts.go_to_search.display {
            if let Some(index) = self.tabs.iter().position(|tab| matches!(tab.content, TabContent::Search)) {
                return self.update(Message::TabClicked(index));
            } else {
                return self.update(Message::NewTab(TabContent::Search));
            }
        }
        if current_shortcut == shortcuts.go_to_library.display {
            if let Some(index) = self.tabs.iter().position(|tab| matches!(tab.content, TabContent::Library)) {
                return self.update(Message::TabClicked(index));
            } else {
                return self.update(Message::NewTab(TabContent::Library));
            }
        }
        if current_shortcut == shortcuts.go_to_downloads.display {
            if let Some(index) = self.tabs.iter().position(|tab| matches!(tab.content, TabContent::Downloads)) {
                return self.update(Message::TabClicked(index));
            } else {
                return self.update(Message::NewTab(TabContent::Downloads));
            }
        }
        if current_shortcut == shortcuts.go_to_settings.display {
            if let Some(index) = self.tabs.iter().position(|tab| matches!(tab.content, TabContent::Settings)) {
                return self.update(Message::TabClicked(index));
            } else {
                return self.update(Message::NewTab(TabContent::Settings));
            }
        }
        
        Task::none()
    }

    // 订阅和主题方法保持不变
    pub fn subscription(&self) -> Subscription<Message> {
        iced::event::listen().map(|event| {
            if let Event::Keyboard(iced::keyboard::Event::KeyPressed { key, modifiers, .. }) = event {
                Message::KeyPressed(key, modifiers)
            } else {
                // 对于其他事件，我们需要一个默认消息
                Message::SearchQueryChanged(String::new()) // 临时的默认消息
            }
        })
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

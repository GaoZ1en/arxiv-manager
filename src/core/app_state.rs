// 应用状态管理

use std::time::Instant;
use iced::widget::pane_grid;
use iced::{Task, Subscription, Event};
use iced::keyboard::{Key, Modifiers};

use crate::core::{ArxivPaper, DownloadItem, DownloadStatus, Pane, PaneType, SearchConfig, AppSettings};
use crate::core::messages::{Message, Command};
use crate::search::services::{search_arxiv_papers_advanced, download_pdf};

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
    // 命令栏状态
    pub command_palette_visible: bool,
    pub command_palette_input: String,
    pub command_suggestions: Vec<Command>,
    pub selected_command_index: Option<usize>,
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
            // 命令栏初始化
            command_palette_visible: false,
            command_palette_input: String::new(),
            command_suggestions: Vec::new(),
            selected_command_index: None,
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
            // 面板导航消息处理
            Message::OpenSearchPane => {
                self.open_pane(PaneType::Search, "Search".to_string())
            }
            Message::OpenLibraryPane => {
                self.open_pane(PaneType::Library, "Library".to_string())
            }
            Message::OpenDownloadsPane => {
                self.open_pane(PaneType::Downloads, "Downloads".to_string())
            }
            Message::OpenSettingsPane => {
                self.open_pane(PaneType::Settings, "Settings".to_string())
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
            // 快捷键设置消息处理
            Message::ShortcutChanged { action, shortcut } => {
                self.update_shortcut(&action, &shortcut);
                Task::none()
            }
            Message::ResetShortcuts => {
                self.settings.shortcuts = crate::core::models::KeyboardShortcuts::default();
                Task::none()
            }
            // 命令栏消息处理
            Message::ToggleCommandPalette => {
                self.command_palette_visible = !self.command_palette_visible;
                if self.command_palette_visible {
                    self.command_palette_input.clear();
                    self.command_suggestions = self.get_all_commands();
                    self.selected_command_index = None;
                } else {
                    self.command_suggestions.clear();
                }
                Task::none()
            }
            Message::CommandPaletteInputChanged(input) => {
                self.command_palette_input = input.clone();
                self.command_suggestions = self.filter_commands(&input);
                self.selected_command_index = if self.command_suggestions.is_empty() {
                    None
                } else {
                    Some(0)
                };
                Task::none()
            }
            Message::CommandSelected(index) => {
                self.selected_command_index = Some(index);
                Task::none()
            }
            Message::ExecuteCommand(command) => {
                self.command_palette_visible = false;
                self.command_palette_input.clear();
                self.command_suggestions.clear();
                self.execute_command(command)
            }
            Message::ClearCommandPalette => {
                self.command_palette_visible = false;
                self.command_palette_input.clear();
                self.command_suggestions.clear();
                self.selected_command_index = None;
                Task::none()
            }
            // 快捷键操作处理
            Message::FocusSearchInput => {
                // 如果不在搜索面板，先打开搜索面板
                if !self.is_search_pane_active() {
                    self.open_pane(PaneType::Search, "Search".to_string())
                } else {
                    Task::none()
                }
            }
            Message::QuickSaveCurrentPaper => {
                if let Some(paper) = self.get_current_paper() {
                    self.update(Message::SavePaper(paper))
                } else {
                    Task::none()
                }
            }
            Message::QuickDownloadCurrentPaper => {
                if let Some(paper) = self.get_current_paper() {
                    self.update(Message::DownloadPaper(paper))
                } else {
                    Task::none()
                }
            }
            Message::NavigateToNextPane => {
                self.navigate_to_next_pane();
                Task::none()
            }
            Message::NavigateToPreviousPane => {
                self.navigate_to_previous_pane();
                Task::none()
            }
            Message::ToggleFullscreen => {
                // TODO: 实现全屏切换
                Task::none()
            }
            Message::NoOp => {
                // 占位符消息，不执行任何操作
                Task::none()
            }
        }
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    // 键盘事件订阅
    pub fn subscription(&self) -> Subscription<Message> {
        iced::event::listen().map(|event| {
            match event {
                Event::Keyboard(iced::keyboard::Event::KeyPressed {
                    key,
                    modifiers,
                    ..
                }) => {
                    match (key.as_ref(), modifiers) {
                        // Ctrl+Shift+P: 打开命令栏
                        (Key::Character("P"), Modifiers::CTRL | Modifiers::SHIFT) |
                        (Key::Character("p"), Modifiers::CTRL | Modifiers::SHIFT) => {
                            Message::ToggleCommandPalette
                        }
                        // Ctrl+K: 打开命令栏
                        (Key::Character("K"), Modifiers::CTRL) |
                        (Key::Character("k"), Modifiers::CTRL) => {
                            Message::ToggleCommandPalette
                        }
                        // Escape: 关闭命令栏
                        (Key::Named(iced::keyboard::key::Named::Escape), _) => {
                            Message::ClearCommandPalette
                        }
                        // Ctrl+F: 聚焦搜索
                        (Key::Character("F"), Modifiers::CTRL) |
                        (Key::Character("f"), Modifiers::CTRL) => {
                            Message::FocusSearchInput
                        }
                        // Ctrl+S: 快速保存当前论文
                        (Key::Character("S"), Modifiers::CTRL) |
                        (Key::Character("s"), Modifiers::CTRL) => {
                            Message::QuickSaveCurrentPaper
                        }
                        // Ctrl+D: 快速下载当前论文
                        (Key::Character("D"), Modifiers::CTRL) |
                        (Key::Character("d"), Modifiers::CTRL) => {
                            Message::QuickDownloadCurrentPaper
                        }
                        // Ctrl+`: 切换侧边栏
                        (Key::Character("`"), Modifiers::CTRL) => {
                            Message::SidebarToggled
                        }
                        // Ctrl+Tab: 下一个面板
                        (Key::Named(iced::keyboard::key::Named::Tab), modifiers) if modifiers.contains(Modifiers::CTRL) && !modifiers.contains(Modifiers::SHIFT) => {
                            Message::NavigateToNextPane
                        }
                        // Ctrl+Shift+Tab: 上一个面板
                        (Key::Named(iced::keyboard::key::Named::Tab), modifiers) if modifiers.contains(Modifiers::CTRL) && modifiers.contains(Modifiers::SHIFT) => {
                            Message::NavigateToPreviousPane
                        }
                        // Ctrl+W: 关闭当前面板
                        (Key::Character("W"), Modifiers::CTRL) |
                        (Key::Character("w"), Modifiers::CTRL) => {
                            Message::CloseFocusedPane
                        }
                        // Ctrl+Shift+\: 垂直分割
                        (Key::Character("\\"), Modifiers::CTRL | Modifiers::SHIFT) => {
                            Message::SplitVertical
                        }
                        // Ctrl+Shift+-: 水平分割
                        (Key::Character("-"), Modifiers::CTRL | Modifiers::SHIFT) => {
                            Message::SplitHorizontal
                        }
                        // F11: 全屏切换
                        (Key::Named(iced::keyboard::key::Named::F11), _) => {
                            Message::ToggleFullscreen
                        }
                        // 数字键快速导航
                        (Key::Character("1"), Modifiers::CTRL) => {
                            Message::OpenSearchPane
                        }
                        (Key::Character("2"), Modifiers::CTRL) => {
                            Message::OpenLibraryPane
                        }
                        (Key::Character("3"), Modifiers::CTRL) => {
                            Message::OpenDownloadsPane
                        }
                        (Key::Character("4"), Modifiers::CTRL) => {
                            Message::OpenSettingsPane
                        }
                        _ => Message::NoOp, // 不匹配的按键使用NoOp
                    }
                }
                _ => Message::NoOp, // 非键盘事件使用NoOp
            }
        })
    }

    // 辅助方法：打开新面板
    fn open_pane(&mut self, pane_type: PaneType, title: String) -> Task<Message> {
        let new_pane = Pane { pane_type, title };
        
        let target_pane = if let Some(focus) = self.focus {
            Some(focus)
        } else {
            // 获取第一个面板的ID
            self.panes.iter().next().map(|(id, _)| *id)
        };
        
        if let Some(pane_id) = target_pane {
            let _ = self.panes.split(pane_grid::Axis::Vertical, pane_id, new_pane);
        }
        
        Task::none()
    }

    // 命令栏相关辅助方法
    fn get_all_commands(&self) -> Vec<Command> {
        let mut commands = vec![
            Command::NewSearch,
            Command::AdvancedSearch,
            Command::ClearSearch,
            Command::GoToSearch,
            Command::GoToLibrary,
            Command::GoToDownloads,
            Command::GoToSettings,
            Command::SplitPaneHorizontal,
            Command::SplitPaneVertical,
            Command::CloseCurrentPane,
            Command::SaveCurrentPaper,
            Command::DownloadCurrentPaper,
            Command::ToggleTheme,
            Command::ToggleSidebar,
            Command::OpenSettings,
            Command::ShowHelp,
            Command::ShowAbout,
            Command::Quit,
        ];

        // 添加保存的论文作为打开命令
        for paper in &self.saved_papers {
            commands.push(Command::OpenPaper(paper.title.clone()));
        }

        commands
    }

    fn filter_commands(&self, query: &str) -> Vec<Command> {
        if query.trim().is_empty() {
            return self.get_all_commands();
        }

        let query_lower = query.to_lowercase();
        let mut commands = Vec::new();

        for command in self.get_all_commands() {
            let mut matches = false;
            
            // 检查显示名称
            if command.display_name().to_lowercase().contains(&query_lower) {
                matches = true;
            }
            
            // 检查关键词
            if !matches {
                for keyword in command.keywords() {
                    if keyword.to_lowercase().contains(&query_lower) {
                        matches = true;
                        break;
                    }
                }
            }

            if matches {
                commands.push(command);
            }
        }

        commands
    }

    fn execute_command(&mut self, command: Command) -> Task<Message> {
        match command {
            Command::NewSearch => {
                self.search_query.clear();
                self.search_config.query.clear();
                self.search_results.clear();
                self.search_error = None;
                self.open_pane(PaneType::Search, "Search".to_string())
            }
            Command::AdvancedSearch => {
                self.advanced_search_visible = true;
                self.open_pane(PaneType::Search, "Search".to_string())
            }
            Command::ClearSearch => {
                self.search_query.clear();
                self.search_config.query.clear();
                self.search_results.clear();
                self.search_error = None;
                Task::none()
            }
            Command::GoToSearch => self.open_pane(PaneType::Search, "Search".to_string()),
            Command::GoToLibrary => self.open_pane(PaneType::Library, "Library".to_string()),
            Command::GoToDownloads => self.open_pane(PaneType::Downloads, "Downloads".to_string()),
            Command::GoToSettings => self.open_pane(PaneType::Settings, "Settings".to_string()),
            Command::SplitPaneHorizontal => {
                if let Some(focus) = self.focus {
                    let new_pane = Pane {
                        pane_type: PaneType::Search,
                        title: "Search".to_string(),
                    };
                    let _ = self.panes.split(pane_grid::Axis::Horizontal, focus, new_pane);
                }
                Task::none()
            }
            Command::SplitPaneVertical => {
                if let Some(focus) = self.focus {
                    let new_pane = Pane {
                        pane_type: PaneType::Search,
                        title: "Search".to_string(),
                    };
                    let _ = self.panes.split(pane_grid::Axis::Vertical, focus, new_pane);
                }
                Task::none()
            }
            Command::CloseCurrentPane => {
                if let Some(focus) = self.focus {
                    if let Some(_) = self.panes.close(focus) {
                        self.focus = None;
                    }
                }
                Task::none()
            }
            Command::OpenPaper(title) => {
                if let Some(paper) = self.saved_papers.iter().find(|p| p.title == title).cloned() {
                    self.update(Message::OpenPaperPane(paper))
                } else {
                    Task::none()
                }
            }
            Command::SaveCurrentPaper => {
                if let Some(paper) = self.get_current_paper() {
                    self.update(Message::SavePaper(paper))
                } else {
                    Task::none()
                }
            }
            Command::DownloadCurrentPaper => {
                if let Some(paper) = self.get_current_paper() {
                    self.update(Message::DownloadPaper(paper))
                } else {
                    Task::none()
                }
            }
            Command::ToggleTheme => {
                // TODO: 实现主题切换
                Task::none()
            }
            Command::ToggleSidebar => {
                self.sidebar_visible = !self.sidebar_visible;
                Task::none()
            }
            Command::OpenSettings => self.open_pane(PaneType::Settings, "Settings".to_string()),
            Command::ShowHelp => {
                // TODO: 实现帮助对话框
                Task::none()
            }
            Command::ShowAbout => {
                // TODO: 实现关于对话框
                Task::none()
            }
            Command::Quit => {
                // TODO: 实现应用退出
                Task::none()
            }
        }
    }

    // 辅助方法
    fn is_search_pane_active(&self) -> bool {
        if let Some(focus) = self.focus {
            if let Some(pane) = self.panes.get(focus) {
                matches!(pane.pane_type, PaneType::Search)
            } else {
                false
            }
        } else {
            false
        }
    }

    fn get_current_paper(&self) -> Option<ArxivPaper> {
        // 尝试从当前焦点面板获取论文
        if let Some(focus) = self.focus {
            if let Some(pane) = self.panes.get(focus) {
                match &pane.pane_type {
                    PaneType::PaperView(index) => {
                        self.saved_papers.get(*index).cloned()
                    }
                    PaneType::Search => {
                        // 如果在搜索面板，获取第一个搜索结果
                        self.search_results.first().cloned()
                    }
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn navigate_to_next_pane(&mut self) {
        let panes: Vec<_> = self.panes.iter().map(|(id, _)| *id).collect();
        if panes.is_empty() {
            return;
        }

        if let Some(current_focus) = self.focus {
            if let Some(current_index) = panes.iter().position(|&id| id == current_focus) {
                let next_index = (current_index + 1) % panes.len();
                self.focus = Some(panes[next_index]);
            }
        } else {
            self.focus = Some(panes[0]);
        }
    }

    fn navigate_to_previous_pane(&mut self) {
        let panes: Vec<_> = self.panes.iter().map(|(id, _)| *id).collect();
        if panes.is_empty() {
            return;
        }

        if let Some(current_focus) = self.focus {
            if let Some(current_index) = panes.iter().position(|&id| id == current_focus) {
                let prev_index = if current_index == 0 {
                    panes.len() - 1
                } else {
                    current_index - 1
                };
                self.focus = Some(panes[prev_index]);
            }
        } else {
            self.focus = Some(panes[0]);
        }
    }

    // 快捷键更新方法
    fn update_shortcut(&mut self, action: &str, shortcut: &str) {
        use crate::core::models::ShortcutKey;
        
        let new_shortcut = ShortcutKey::new(shortcut);
        
        match action {
            "toggle_command_palette" => self.settings.shortcuts.toggle_command_palette = new_shortcut,
            "focus_search" => self.settings.shortcuts.focus_search = new_shortcut,
            "quick_save_paper" => self.settings.shortcuts.quick_save_paper = new_shortcut,
            "quick_download_paper" => self.settings.shortcuts.quick_download_paper = new_shortcut,
            "toggle_sidebar" => self.settings.shortcuts.toggle_sidebar = new_shortcut,
            "next_pane" => self.settings.shortcuts.next_pane = new_shortcut,
            "previous_pane" => self.settings.shortcuts.previous_pane = new_shortcut,
            "close_pane" => self.settings.shortcuts.close_pane = new_shortcut,
            "split_horizontal" => self.settings.shortcuts.split_horizontal = new_shortcut,
            "split_vertical" => self.settings.shortcuts.split_vertical = new_shortcut,
            "go_to_search" => self.settings.shortcuts.go_to_search = new_shortcut,
            "go_to_library" => self.settings.shortcuts.go_to_library = new_shortcut,
            "go_to_downloads" => self.settings.shortcuts.go_to_downloads = new_shortcut,
            "go_to_settings" => self.settings.shortcuts.go_to_settings = new_shortcut,
            _ => {}
        }
    }


}

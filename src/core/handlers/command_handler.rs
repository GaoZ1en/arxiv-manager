// 命令面板消息处理器
// 处理所有与命令面板相关的消息

use iced::Task;

use crate::core::{ArxivManager, Message, Command, TabContent};

pub trait CommandHandler {
    fn handle_command_palette_toggled(&mut self) -> Task<Message>;
    fn handle_command_input_changed(&mut self, input: String) -> Task<Message>;
    fn handle_command_selected(&mut self, index: usize) -> Task<Message>;
    fn handle_command_executed(&mut self, command: Command) -> Task<Message>;
    fn handle_sidebar_toggled(&mut self) -> Task<Message>;
    
    // 辅助方法
    fn update_command_suggestions(&mut self);
    fn execute_command(&mut self, command: &Command) -> Task<Message>;
}

impl CommandHandler for ArxivManager {
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

    // 辅助方法实现
    fn update_command_suggestions(&mut self) {
        use crate::core::messages::Command;
        
        let all_commands = vec![
            Command::ToggleSidebar,
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
            Command::OpenSettings,
            Command::ShowHelp,
            Command::ShowAbout,
            Command::Quit,
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
            Command::AdvancedSearch => {
                // 切换高级搜索
                self.update(Message::AdvancedSearchToggled)
            }
            Command::ClearSearch => {
                // 清空搜索
                self.search_query.clear();
                self.search_results.clear();
                Task::none()
            }
            Command::GoToSearch => {
                // 查找或创建Search标签页
                if let Some(index) = self.tabs.iter().position(|tab| {
                    matches!(tab.content, TabContent::Search)
                }) {
                    self.active_tab = index;
                } else {
                    return self.update(Message::NewTab(TabContent::Search));
                }
                Task::none()
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
            Command::SplitPaneHorizontal => {
                // TODO: 实现水平分割面板
                println!("Split pane horizontally");
                Task::none()
            }
            Command::SplitPaneVertical => {
                // TODO: 实现垂直分割面板
                println!("Split pane vertically");
                Task::none()
            }
            Command::CloseCurrentPane => {
                // 关闭当前标签页
                self.update(Message::CloseActiveTab)
            }
            Command::OpenPaper(paper_id) => {
                // TODO: 打开指定论文
                println!("Opening paper: {}", paper_id);
                Task::none()
            }
            Command::SaveCurrentPaper => {
                // 保存当前论文
                if let Some(paper) = self.get_current_paper() {
                    self.update(Message::SavePaper(paper))
                } else {
                    Task::none()
                }
            }
            Command::DownloadCurrentPaper => {
                // 下载当前论文
                if let Some(paper) = self.get_current_paper() {
                    self.update(Message::DownloadPaper(paper))
                } else {
                    Task::none()
                }
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
            Command::OpenSettings => {
                // 打开设置页面
                if let Some(index) = self.tabs.iter().position(|tab| {
                    matches!(tab.content, TabContent::Settings)
                }) {
                    self.active_tab = index;
                } else {
                    return self.update(Message::NewTab(TabContent::Settings));
                }
                Task::none()
            }
            Command::ShowHelp => {
                // TODO: 打开帮助文档
                println!("Opening help...");
                Task::none()
            }
            Command::ShowAbout => {
                // TODO: 显示关于信息
                println!("Showing about...");
                Task::none()
            }
            Command::Quit => {
                // TODO: 退出应用
                std::process::exit(0);
            }
        }
    }
}

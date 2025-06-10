// 标签页消息处理器
// 处理所有与标签页相关的消息

use iced::Task;
use std::time::Instant;

use crate::core::{ArxivManager, Message, Tab, TabContent};

pub trait TabHandler {
    fn handle_tab_clicked(&mut self, tab_index: usize) -> Task<Message>;
    fn handle_tab_close(&mut self, tab_index: usize) -> Task<Message>;
    fn handle_navigate_to_next_tab(&mut self) -> Task<Message>;
    fn handle_navigate_to_previous_tab(&mut self) -> Task<Message>;
    fn handle_close_active_tab(&mut self) -> Task<Message>;
    fn handle_new_tab(&mut self, content: TabContent) -> Task<Message>;
    
    // 辅助方法
    fn navigate_to_next_tab(&mut self);
    fn navigate_to_previous_tab(&mut self);
}

impl TabHandler for ArxivManager {
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

    // 辅助方法实现
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
}

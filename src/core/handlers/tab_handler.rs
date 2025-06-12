// 标签页消息处理器
// 处理所有与标签页相关的消息

use iced::Task;
use std::time::Instant;

use crate::core::{ArxivManager, Tab, TabContent, SessionManager};
use crate::core::messages::Message;
use crate::core::models::ui::TabGroup;

pub trait TabHandler {
    fn handle_tab_clicked(&mut self, tab_index: usize) -> Task<Message>;
    fn handle_tab_close(&mut self, tab_index: usize) -> Task<Message>;
    fn handle_navigate_to_next_tab(&mut self) -> Task<Message>;
    fn handle_navigate_to_previous_tab(&mut self) -> Task<Message>;
    fn handle_close_active_tab(&mut self) -> Task<Message>;
    fn handle_new_tab(&mut self, content: TabContent) -> Task<Message>;
    
    // 新增的标签页操作
    fn handle_tab_right_clicked(&mut self, tab_index: usize, position: iced::Point) -> Task<Message>;
    fn handle_tab_pin(&mut self, tab_index: usize) -> Task<Message>;
    fn handle_tab_unpin(&mut self, tab_index: usize) -> Task<Message>;
    fn handle_tab_move_to_group(&mut self, tab_index: usize, group: TabGroup) -> Task<Message>;
    fn handle_tab_duplicate(&mut self, tab_index: usize) -> Task<Message>;
    fn handle_close_tabs_to_right(&mut self, tab_index: usize) -> Task<Message>;
    fn handle_close_other_tabs(&mut self, tab_index: usize) -> Task<Message>;
    fn handle_close_tabs_in_group(&mut self, group: TabGroup) -> Task<Message>;
    
    // 会话管理
    fn handle_save_session(&mut self) -> Task<Message>;
    fn handle_load_session(&mut self) -> Task<Message>;
    
    // 辅助方法
    fn navigate_to_next_tab(&mut self);
    fn navigate_to_previous_tab(&mut self);
    fn sort_tabs_by_groups(&mut self);
    fn handle_save_session_internal(&self) -> Result<(), Box<dyn std::error::Error>>;
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
                    Self::generate_paper_tab_title(paper)
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
    
    fn handle_tab_right_clicked(&mut self, tab_index: usize, position: iced::Point) -> Task<Message> {
        // 显示/隐藏右键菜单
        if tab_index < self.tabs.len() {
            self.context_menu.visible = !self.context_menu.visible;
            self.context_menu.tab_index = tab_index;
            // 使用传入的位置显示菜单
            self.context_menu.x = position.x;
            self.context_menu.y = position.y;
        }
        Task::none()
    }
    
    fn handle_tab_pin(&mut self, tab_index: usize) -> Task<Message> {
        if tab_index < self.tabs.len() {
            self.tabs[tab_index].pin();
            self.sort_tabs_by_groups();
            if let Err(e) = self.handle_save_session_internal() {
                log::warn!("Failed to save session after pinning tab: {}", e);
            }
        }
        Task::none()
    }
    
    fn handle_tab_unpin(&mut self, tab_index: usize) -> Task<Message> {
        if tab_index < self.tabs.len() {
            self.tabs[tab_index].unpin();
            self.sort_tabs_by_groups();
            if let Err(e) = self.handle_save_session_internal() {
                log::warn!("Failed to save session after unpinning tab: {}", e);
            }
        }
        Task::none()
    }
    
    fn handle_tab_move_to_group(&mut self, tab_index: usize, group: TabGroup) -> Task<Message> {
        if tab_index < self.tabs.len() {
            self.tabs[tab_index].group = group;
            self.sort_tabs_by_groups();
            if let Err(e) = self.handle_save_session_internal() {
                log::warn!("Failed to save session after moving tab to group: {}", e);
            }
        }
        Task::none()
    }
    
    fn handle_tab_duplicate(&mut self, tab_index: usize) -> Task<Message> {
        if tab_index < self.tabs.len() {
            let original_tab = &self.tabs[tab_index];
            let new_tab = Tab::new_with_group(
                self.next_tab_id,
                format!("{} (副本)", original_tab.title),
                original_tab.content.clone(),
                original_tab.group.clone()
            );
            
            self.tabs.insert(tab_index + 1, new_tab);
            self.next_tab_id += 1;
            
            // 调整活动标签页索引
            if self.active_tab > tab_index {
                self.active_tab += 1;
            }
            
            if let Err(e) = self.handle_save_session_internal() {
                log::warn!("Failed to save session after duplicating tab: {}", e);
            }
        }
        Task::none()
    }
    
    fn handle_close_tabs_to_right(&mut self, tab_index: usize) -> Task<Message> {
        if tab_index < self.tabs.len() {
            // 收集所有可关闭的标签页（在指定索引右侧）
            let mut indices_to_remove = Vec::new();
            for i in (tab_index + 1)..self.tabs.len() {
                if self.tabs[i].closable {
                    indices_to_remove.push(i);
                }
            }
            
            // 从后往前删除，避免索引变化
            for &i in indices_to_remove.iter().rev() {
                self.tabs.remove(i);
            }
            
            // 调整活动标签页索引
            if self.active_tab >= self.tabs.len() && !self.tabs.is_empty() {
                self.active_tab = self.tabs.len() - 1;
            }
            
            if let Err(e) = self.handle_save_session_internal() {
                log::warn!("Failed to save session after closing tabs to right: {}", e);
            }
        }
        Task::none()
    }
    
    fn handle_close_other_tabs(&mut self, tab_index: usize) -> Task<Message> {
        if tab_index < self.tabs.len() {
            let keep_tab = self.tabs[tab_index].clone();
            
            // 保留不可关闭的标签页和指定的标签页
            self.tabs.retain(|tab| !tab.closable || tab.id == keep_tab.id);
            
            // 找到保留标签页的新索引
            if let Some(new_index) = self.tabs.iter().position(|tab| tab.id == keep_tab.id) {
                self.active_tab = new_index;
            } else {
                self.active_tab = 0;
            }
            
            if let Err(e) = self.handle_save_session_internal() {
                log::warn!("Failed to save session after closing other tabs: {}", e);
            }
        }
        Task::none()
    }
    
    fn handle_close_tabs_in_group(&mut self, group: TabGroup) -> Task<Message> {
        // 收集要删除的标签页索引
        let mut indices_to_remove = Vec::new();
        for (i, tab) in self.tabs.iter().enumerate() {
            if tab.group == group && tab.closable {
                indices_to_remove.push(i);
            }
        }
        
        // 从后往前删除
        for &i in indices_to_remove.iter().rev() {
            self.tabs.remove(i);
        }
        
        // 调整活动标签页索引
        if self.active_tab >= self.tabs.len() && !self.tabs.is_empty() {
            self.active_tab = self.tabs.len() - 1;
        }
        
        if let Err(e) = self.handle_save_session_internal() {
            log::warn!("Failed to save session after closing tabs in group: {}", e);
        }
        Task::none()
    }
    
    fn handle_save_session(&mut self) -> Task<Message> {
        if let Err(e) = self.handle_save_session_internal() {
            log::error!("Failed to save session: {}", e);
        }
        Task::none()
    }
    
    fn handle_load_session(&mut self) -> Task<Message> {
        match SessionManager::load_session() {
            Ok(session_data) => {
                self.tabs = session_data.tabs.into_iter().map(|tab| tab.into()).collect();
                self.active_tab = session_data.active_tab.min(self.tabs.len().saturating_sub(1));
                self.next_tab_id = session_data.next_tab_id;
                self.sort_tabs_by_groups();
            }
            Err(e) => {
                log::warn!("Failed to load session: {}", e);
            }
        }
        Task::none()
    }
    
    fn sort_tabs_by_groups(&mut self) {
        // 按固定状态和分组排序标签页
        // 固定的标签页在前，然后按分组排序
        self.tabs.sort_by(|a, b| {
            use std::cmp::Ordering;
            
            // 首先按固定状态排序
            match (a.pinned, b.pinned) {
                (true, false) => Ordering::Less,
                (false, true) => Ordering::Greater,
                _ => {
                    // 然后按分组排序
                    let a_group_order = match &a.group {
                        TabGroup::Default => 0,
                        TabGroup::Research => 1,
                        TabGroup::Library => 2,
                        TabGroup::Downloads => 3,
                        TabGroup::Custom(_) => 4,
                    };
                    let b_group_order = match &b.group {
                        TabGroup::Default => 0,
                        TabGroup::Research => 1,
                        TabGroup::Library => 2,
                        TabGroup::Downloads => 3,
                        TabGroup::Custom(_) => 4,
                    };
                    a_group_order.cmp(&b_group_order)
                }
            }
        });
        
        // 重新计算活动标签页索引
        if let Some(active_tab) = self.tabs.get(self.active_tab) {
            let active_id = active_tab.id;
            if let Some(new_index) = self.tabs.iter().position(|tab| tab.id == active_id) {
                self.active_tab = new_index;
            }
        }
    }
    
    // 内部会话保存方法
    fn handle_save_session_internal(&self) -> Result<(), Box<dyn std::error::Error>> {
        SessionManager::save_session(&self.tabs, self.active_tab, self.next_tab_id)
    }
}

impl ArxivManager {
    /// 生成论文标签页标题的辅助函数
    /// 格式：arXiv:ID | 标题前几个单词...
    pub fn generate_paper_tab_title(paper: &crate::core::models::ArxivPaper) -> String {
        let mut title_words: Vec<&str> = paper.title.split_whitespace().collect();
        
        // 限制标题单词数量，避免标签页过长
        let max_words = 4;
        if title_words.len() > max_words {
            title_words.truncate(max_words);
            format!("arXiv:{} | {}...", paper.id, title_words.join(" "))
        } else {
            format!("arXiv:{} | {}", paper.id, title_words.join(" "))
        }
    }
}

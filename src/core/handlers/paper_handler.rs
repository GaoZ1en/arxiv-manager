// 论文操作消息处理器
// 处理所有与论文相关的消息

use iced::Task;

use crate::core::{ArxivManager, Message, ArxivPaper, TabContent, Tab};

pub trait PaperHandler {
    fn handle_paper_save(&mut self, paper: ArxivPaper) -> Task<Message>;
    fn handle_paper_remove(&mut self, index: usize) -> Task<Message>;
    fn handle_paper_view(&mut self, paper: ArxivPaper) -> Task<Message>;
    fn handle_paper_export(&mut self, format: String) -> Task<Message>;
    
    // 辅助方法
    fn get_current_paper(&self) -> Option<ArxivPaper>;
}

impl PaperHandler for ArxivManager {
    fn handle_paper_save(&mut self, paper: ArxivPaper) -> Task<Message> {
        // 检查是否已经保存
        if !self.saved_papers.iter().any(|p| p.id == paper.id) {
            self.saved_papers.push(paper);
        }
        Task::none()
    }

    fn handle_paper_remove(&mut self, index: usize) -> Task<Message> {
        if index < self.saved_papers.len() {
            self.saved_papers.remove(index);
            
            // 关闭所有引用该论文的标签页
            let mut tabs_to_close = Vec::new();
            for (i, tab) in self.tabs.iter().enumerate() {
                if let TabContent::PaperView(paper_index) = &tab.content {
                    if *paper_index == index {
                        tabs_to_close.push(i);
                    } else if *paper_index > index {
                        // 更新索引，因为移除了一个论文
                        // 这里需要更新标签页的内容
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
            
            // 更新其他标签页的论文索引
            for tab in &mut self.tabs {
                if let TabContent::PaperView(paper_index) = &mut tab.content {
                    if *paper_index > index {
                        *paper_index -= 1;
                    }
                }
            }
        }
        Task::none()
    }

    fn handle_paper_view(&mut self, paper: ArxivPaper) -> Task<Message> {
        // 查找或创建论文在saved_papers中的索引
        let paper_index = if let Some(index) = self.saved_papers.iter().position(|p| p.id == paper.id) {
            index
        } else {
            // 如果论文不在saved_papers中，先添加它
            self.saved_papers.push(paper.clone());
            self.saved_papers.len() - 1
        };
        
        // 检查是否已经有该论文的标签页
        if let Some(existing_tab_index) = self.tabs.iter().position(|tab| {
            matches!(&tab.content, TabContent::PaperView(index) if *index == paper_index)
        }) {
            // 如果已经有该标签页，直接切换到它
            self.active_tab = existing_tab_index;
        } else {
            // 创建新的论文详情标签页
            let title = format!("论文: {}", paper.title);
            let new_tab = Tab::new(self.next_tab_id, title, TabContent::PaperView(paper_index));
            self.tabs.push(new_tab);
            self.active_tab = self.tabs.len() - 1;
            self.next_tab_id += 1;
        }
        
        Task::none()
    }

    fn handle_paper_export(&mut self, format: String) -> Task<Message> {
        // TODO: 实现论文导出功能
        println!("Exporting papers to format: {}", format);
        Task::none()
    }

    // 辅助方法实现
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
}

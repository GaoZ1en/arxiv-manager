// 下载队列实现模块

use super::types::DownloadTask;

/// 下载队列
#[derive(Debug)]
pub struct DownloadQueue {
    tasks: Vec<DownloadTask>,
}

impl DownloadQueue {
    /// 创建新的下载队列
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
        }
    }
    
    /// 添加下载任务
    pub fn add_task(&mut self, task: DownloadTask) {
        self.tasks.push(task);
        // Sort by priority (higher priority first)
        self.tasks.sort_by(|a, b| b.priority.cmp(&a.priority));
    }
    
    /// 移除下载任务
    pub fn remove_task(&mut self, arxiv_id: &str) -> Option<DownloadTask> {
        if let Some(index) = self.tasks.iter().position(|t| t.paper.id == arxiv_id) {
            Some(self.tasks.remove(index))
        } else {
            None
        }
    }
    
    /// 获取下一个任务
    pub fn next_task(&mut self) -> Option<DownloadTask> {
        self.tasks.pop()
    }
    
    /// 检查队列是否为空
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
    
    /// 获取队列长度
    pub fn len(&self) -> usize {
        self.tasks.len()
    }
}

impl Default for DownloadQueue {
    fn default() -> Self {
        Self::new()
    }
}

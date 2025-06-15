// 集合操作处理器
// 处理所有与集合相关的消息

use iced::Task;
use crate::core::{ArxivManager, messages::Message};
use crate::core::models::{Collection, SystemCollections};

pub trait CollectionHandler {
    fn handle_create_collection(&mut self, name: String, parent_id: Option<i64>) -> Task<Message>;
    fn handle_rename_collection(&mut self, id: i64, new_name: String) -> Task<Message>;
    fn handle_delete_collection(&mut self, id: i64) -> Task<Message>;
    fn handle_move_collection(&mut self, id: i64, new_parent_id: Option<i64>) -> Task<Message>;
    fn handle_toggle_collection_expanded(&mut self, id: i64) -> Task<Message>;
    fn handle_add_paper_to_collection(&mut self, paper_index: usize, collection_id: i64) -> Task<Message>;
    fn handle_remove_paper_from_collection(&mut self, paper_index: usize, collection_id: i64) -> Task<Message>;
    fn handle_select_collection(&mut self, collection_id: Option<i64>) -> Task<Message>;
    fn handle_load_collections(&mut self) -> Task<Message>;
    fn handle_collections_loaded(&mut self, collections: Vec<Collection>) -> Task<Message>;
    
    // 辅助方法
    fn filter_papers_by_collection(&mut self, collection_id: Option<i64>);
    fn build_collection_tree(&self) -> Vec<crate::core::models::CollectionTreeNode>;
    fn get_system_collections(&self) -> Vec<Collection>;
}

impl CollectionHandler for ArxivManager {
    fn handle_create_collection(&mut self, name: String, parent_id: Option<i64>) -> Task<Message> {
        if name.trim().is_empty() {
            return Task::none();
        }
        
        let mut new_collection = Collection::new(name.trim().to_string());
        new_collection.parent_id = parent_id;
        
        // TODO: 这里应该调用数据库服务创建集合
        // 现在先简单地添加到内存中
        new_collection.id = self.collections.len() as i64 + 1; // 临时ID分配
        self.collections.push(new_collection.clone());
        
        // 清空输入
        self.collection_name_input.clear();
        self.is_creating_collection = false;
        self.collection_parent_id = None;
        
        Task::none()
    }

    fn handle_rename_collection(&mut self, id: i64, new_name: String) -> Task<Message> {
        if let Some(collection) = self.collections.iter_mut().find(|c| c.id == id) {
            collection.name = new_name;
            // TODO: 更新数据库
        }
        // 清除编辑状态
        self.editing_collection_id = None;
        self.collection_rename_input.clear();
        Task::none()
    }

    fn handle_delete_collection(&mut self, id: i64) -> Task<Message> {
        // 删除集合及其子集合
        self.delete_collection_recursive(id);
        
        // 如果删除的是当前选中的集合，重置选择
        if self.selected_collection_id == Some(id) {
            self.selected_collection_id = None;
            self.filter_papers_by_collection(None);
        }
        
        Task::none()
    }

    fn handle_move_collection(&mut self, id: i64, new_parent_id: Option<i64>) -> Task<Message> {
        if let Some(collection) = self.collections.iter_mut().find(|c| c.id == id) {
            collection.parent_id = new_parent_id;
            // TODO: 更新数据库
        }
        Task::none()
    }

    fn handle_toggle_collection_expanded(&mut self, id: i64) -> Task<Message> {
        if let Some(collection) = self.collections.iter_mut().find(|c| c.id == id) {
            collection.is_expanded = !collection.is_expanded;
        }
        // 同时更新展开状态映射
        let current_state = self.collection_tree_expanded.get(&id).unwrap_or(&true);
        self.collection_tree_expanded.insert(id, !current_state);
        Task::none()
    }

    fn handle_add_paper_to_collection(&mut self, paper_index: usize, collection_id: i64) -> Task<Message> {
        if paper_index < self.saved_papers.len() {
            // TODO: 在数据库中创建关联关系
            // 这里可能需要先将论文保存到数据库，然后创建关联
            log::info!("Adding paper {} to collection {}", paper_index, collection_id);
        }
        Task::none()
    }

    fn handle_remove_paper_from_collection(&mut self, paper_index: usize, collection_id: i64) -> Task<Message> {
        if paper_index < self.saved_papers.len() {
            // TODO: 在数据库中删除关联关系
            log::info!("Removing paper {} from collection {}", paper_index, collection_id);
        }
        Task::none()
    }

    fn handle_select_collection(&mut self, collection_id: Option<i64>) -> Task<Message> {
        self.selected_collection_id = collection_id;
        self.filter_papers_by_collection(collection_id);
        Task::none()
    }

    fn handle_load_collections(&mut self) -> Task<Message> {
        // TODO: 从数据库加载集合
        // 现在先返回一些示例集合
        let mut collections = self.get_system_collections();
        
        // 添加一些示例用户集合
        collections.push(Collection {
            id: 1,
            name: "Machine Learning".to_string(),
            description: Some("Papers about machine learning".to_string()),
            parent_id: None,
            created_at: chrono::Utc::now(),
            is_expanded: true,
            color: Some("#3b82f6".to_string()), // blue
            icon: None,
        });
        
        collections.push(Collection {
            id: 2,
            name: "Computer Vision".to_string(),
            description: Some("Computer vision papers".to_string()),
            parent_id: Some(1), // 子集合
            created_at: chrono::Utc::now(),
            is_expanded: true,
            color: Some("#8b5cf6".to_string()), // violet
            icon: None,
        });
        
        Task::done(Message::CollectionsLoaded(collections))
    }

    fn handle_collections_loaded(&mut self, collections: Vec<Collection>) -> Task<Message> {
        self.collections = collections;
        // 如果没有选中的集合，默认选择"All Papers"
        if self.selected_collection_id.is_none() {
            self.selected_collection_id = Some(-1); // All Papers的ID
        }
        self.filter_papers_by_collection(self.selected_collection_id);
        Task::none()
    }

    fn filter_papers_by_collection(&mut self, collection_id: Option<i64>) {
        match collection_id {
            None | Some(-1) => {
                // 显示所有论文
                self.filtered_papers = self.saved_papers.clone();
            }
            Some(-2) => {
                // 最近添加的论文（按添加时间排序，取最新的10篇）
                let mut recent_papers = self.saved_papers.clone();
                recent_papers.sort_by(|a, b| {
                    let a_time = a.added_at.unwrap_or_else(|| chrono::DateTime::parse_from_rfc3339(&a.published).unwrap_or_default().with_timezone(&chrono::Utc));
                    let b_time = b.added_at.unwrap_or_else(|| chrono::DateTime::parse_from_rfc3339(&b.published).unwrap_or_default().with_timezone(&chrono::Utc));
                    b_time.cmp(&a_time) // 最新的在前
                });
                recent_papers.truncate(10);
                self.filtered_papers = recent_papers;
            }
            Some(-3) => {
                // 收藏夹 - 显示所有收藏的论文
                self.filtered_papers = self.saved_papers.iter()
                    .filter(|paper| paper.is_favorite)
                    .cloned()
                    .collect();
            }
            Some(-4) => {
                // 未分类 - 显示不属于任何用户集合的论文
                self.filtered_papers = self.saved_papers.iter()
                    .filter(|paper| paper.is_uncategorized())
                    .cloned()
                    .collect();
            }
            Some(collection_id) => {
                // 特定集合的论文
                self.filtered_papers = self.saved_papers.iter()
                    .filter(|paper| paper.belongs_to_collection(collection_id))
                    .cloned()
                    .collect();
            }
        }
    }

    fn build_collection_tree(&self) -> Vec<crate::core::models::CollectionTreeNode> {
        // 构建树状结构
        let mut nodes = Vec::new();
        let root_collections: Vec<&Collection> = self.collections.iter()
            .filter(|c| c.parent_id.is_none())
            .collect();
        
        for collection in root_collections {
            let node = self.build_tree_node(collection);
            nodes.push(node);
        }
        
        nodes
    }

    fn get_system_collections(&self) -> Vec<Collection> {
        SystemCollections::get_all_system_collections()
    }
}

impl ArxivManager {
    fn delete_collection_recursive(&mut self, id: i64) {
        // 找到所有子集合并递归删除
        let child_ids: Vec<i64> = self.collections.iter()
            .filter(|c| c.parent_id == Some(id))
            .map(|c| c.id)
            .collect();
        
        for child_id in child_ids {
            self.delete_collection_recursive(child_id);
        }
        
        // 删除当前集合
        self.collections.retain(|c| c.id != id);
        self.collection_tree_expanded.remove(&id);
    }
    
    fn build_tree_node(&self, collection: &Collection) -> crate::core::models::CollectionTreeNode {
        let children: Vec<crate::core::models::CollectionTreeNode> = self.collections.iter()
            .filter(|c| c.parent_id == Some(collection.id))
            .map(|c| self.build_tree_node(c))
            .collect();
        
        let paper_count = self.get_collection_paper_count(collection.id);
        
        crate::core::models::CollectionTreeNode {
            collection: collection.clone(),
            children,
            paper_count,
        }
    }
    
    fn get_collection_paper_count(&self, collection_id: i64) -> usize {
        match collection_id {
            -1 => self.saved_papers.len(), // All Papers
            -2 => self.saved_papers.len().min(10), // Recently Added (最多显示10个)
            -3 => self.saved_papers.iter().filter(|p| p.is_favorite).count(), // Favorites
            -4 => self.saved_papers.iter().filter(|p| p.is_uncategorized()).count(), // Uncategorized
            _ => self.saved_papers.iter().filter(|p| p.belongs_to_collection(collection_id)).count(), // 其他集合
        }
    }
}

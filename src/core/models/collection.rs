// Collection模型 - 文件夹/集合管理
// 类似Zotero的文件夹结构

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 论文集合/文件夹
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Collection {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<i64>, // 支持嵌套文件夹
    pub created_at: DateTime<Utc>,
    pub is_expanded: bool, // UI状态：是否展开
    pub color: Option<String>, // 可选的颜色标识
    pub icon: Option<String>, // 可选的图标
}

/// 论文与集合的关联
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperCollection {
    pub paper_id: i64,
    pub collection_id: i64,
    pub added_at: DateTime<Utc>,
}

/// 集合树节点 - 用于UI显示
#[derive(Debug, Clone)]
pub struct CollectionTreeNode {
    pub collection: Collection,
    pub children: Vec<CollectionTreeNode>,
    pub paper_count: usize,
}

/// 集合类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CollectionType {
    /// 用户创建的普通文件夹
    Folder,
    /// 智能集合（基于规则自动收集）
    SmartCollection,
    /// 标签集合
    TagCollection,
    /// 特殊系统集合
    System,
}

/// 扩展的集合信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionInfo {
    pub collection: Collection,
    pub collection_type: CollectionType,
    pub paper_count: usize,
    pub child_count: usize,
}

impl Collection {
    pub fn new(name: String) -> Self {
        Self {
            id: 0, // 将由数据库分配
            name,
            description: None,
            parent_id: None,
            created_at: Utc::now(),
            is_expanded: true, // 默认展开
            color: None,
            icon: None,
        }
    }

    pub fn with_parent(name: String, parent_id: i64) -> Self {
        Self {
            id: 0,
            name,
            description: None,
            parent_id: Some(parent_id),
            created_at: Utc::now(),
            is_expanded: true,
            color: None,
            icon: None,
        }
    }

    /// 检查是否为根级集合
    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }

    /// 设置展开状态
    pub fn set_expanded(&mut self, expanded: bool) {
        self.is_expanded = expanded;
    }
}

impl Default for Collection {
    fn default() -> Self {
        Self::new("New Collection".to_string())
    }
}

/// 预定义的系统集合
pub struct SystemCollections;

impl SystemCollections {
    /// 所有论文集合
    pub fn all_papers() -> Collection {
        Collection {
            id: -1,
            name: "All Papers".to_string(),
            description: Some("All saved papers".to_string()),
            parent_id: None,
            created_at: Utc::now(),
            is_expanded: true,
            color: Some("#6366f1".to_string()), // indigo
            icon: None,
        }
    }

    /// 最近添加集合
    pub fn recent() -> Collection {
        Collection {
            id: -2,
            name: "Recently Added".to_string(),
            description: Some("Recently saved papers".to_string()),
            parent_id: None,
            created_at: Utc::now(),
            is_expanded: true,
            color: Some("#10b981".to_string()), // emerald
            icon: None,
        }
    }

    /// 收藏夹
    pub fn favorites() -> Collection {
        Collection {
            id: -3,
            name: "Favorites".to_string(),
            description: Some("Favorite papers".to_string()),
            parent_id: None,
            created_at: Utc::now(),
            is_expanded: true,
            color: Some("#f59e0b".to_string()), // amber
            icon: None,
        }
    }

    /// 未分类
    pub fn uncategorized() -> Collection {
        Collection {
            id: -4,
            name: "Uncategorized".to_string(),
            description: Some("Papers not in any collection".to_string()),
            parent_id: None,
            created_at: Utc::now(),
            is_expanded: true,
            color: Some("#6b7280".to_string()), // gray
            icon: None,
        }
    }

    pub fn get_all_system_collections() -> Vec<Collection> {
        vec![
            Self::all_papers(),
            Self::recent(),
            Self::favorites(),
            Self::uncategorized(),
        ]
    }
}

// 数据库模块 - 提供arXiv论文的本地存储功能

use std::path::Path;
use crate::utils::Result;

// 导入子模块
pub mod models;
pub mod operations;
pub mod connection;
pub mod migrations;

// 重新导出常用类型和函数
pub use models::*;
pub use operations::*;
pub use connection::{DatabaseConfig, ConnectionManager};

/// 数据库服务层 - 提供高级数据库操作接口
pub struct DatabaseService {
    connection_manager: connection::ConnectionManager,
}

impl std::fmt::Debug for DatabaseService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DatabaseService")
            .field("connection_manager", &"<ConnectionManager>")
            .finish()
    }
}

impl DatabaseService {
    /// 创建新的数据库服务
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let config = connection::DatabaseConfig {
            db_path: db_path.as_ref().to_string_lossy().to_string(),
            ..Default::default()
        };
        let connection_manager = connection::ConnectionManager::new(config);
        
        // 运行数据库迁移
        let conn = connection_manager.create_connection()?;
        let migration_manager = migrations::MigrationManager::new();
        migration_manager.migrate(&conn)?;
        
        Ok(DatabaseService {
            connection_manager,
        })
    }
    
    /// 创建带自定义配置的数据库服务
    pub fn with_config(config: connection::DatabaseConfig) -> Result<Self> {
        let connection_manager = connection::ConnectionManager::new(config);
        
        // 运行数据库迁移
        let conn = connection_manager.create_connection()?;
        let migration_manager = migrations::MigrationManager::new();
        migration_manager.migrate(&conn)?;
        
        Ok(DatabaseService {
            connection_manager,
        })
    }
    
    /// 插入或更新论文记录
    pub fn insert_paper(&self, paper: &crate::core::models::ArxivPaper) -> Result<i64> {
        let conn = self.connection_manager.create_connection()?;
        operations::create::insert_paper(&conn, paper)
    }
    
    /// 根据arXiv ID获取论文记录
    pub fn get_paper_by_arxiv_id(&self, arxiv_id: &str) -> Result<Option<PaperRecord>> {
        let conn = self.connection_manager.create_connection()?;
        operations::read::get_paper_by_arxiv_id(&conn, arxiv_id)
    }
    
    /// 更新论文的下载状态
    pub fn update_download_status(
        &self, 
        arxiv_id: &str, 
        status: DownloadStatus,
        local_path: Option<&str>
    ) -> Result<()> {
        let conn = self.connection_manager.create_connection()?;
        operations::update::update_download_status(&conn, arxiv_id, status, local_path)
    }
    
    /// 搜索论文
    pub fn search_papers(&self, query: &str, limit: usize) -> Result<Vec<PaperRecord>> {
        let conn = self.connection_manager.create_connection()?;
        operations::search_ops::search_papers(&conn, query, limit)
    }
    
    /// 获取最近的论文
    pub fn get_recent_papers(&self, limit: usize) -> Result<Vec<PaperRecord>> {
        let conn = self.connection_manager.create_connection()?;
        operations::read::get_recent_papers(&conn, limit)
    }
    
    /// 根据下载状态获取论文列表
    pub fn get_papers_by_status(&self, status: DownloadStatus) -> Result<Vec<PaperRecord>> {
        let conn = self.connection_manager.create_connection()?;
        operations::read::get_papers_by_status(&conn, status)
    }
    
    /// 按分类搜索论文
    pub fn search_papers_by_category(&self, category: &str, limit: usize) -> Result<Vec<PaperRecord>> {
        let conn = self.connection_manager.create_connection()?;
        operations::search_ops::search_papers_by_category(&conn, category, limit)
    }
    
    /// 高级搜索
    pub fn advanced_search(
        &self,
        title: Option<&str>,
        authors: Option<&str>,
        abstract_text: Option<&str>,
        categories: Option<&str>,
        limit: usize
    ) -> Result<Vec<PaperRecord>> {
        let conn = self.connection_manager.create_connection()?;
        operations::search_ops::advanced_search(
            &conn, title, authors, abstract_text, categories, limit
        )
    }
    
    /// 批量插入论文
    pub fn batch_insert_papers(&self, papers: &[crate::core::models::ArxivPaper]) -> Result<Vec<i64>> {
        let conn = self.connection_manager.create_connection()?;
        operations::create::batch_insert_papers(&conn, papers)
    }
    
    /// 删除论文记录
    pub fn delete_paper(&self, arxiv_id: &str) -> Result<bool> {
        let conn = self.connection_manager.create_connection()?;
        operations::delete::delete_paper_by_arxiv_id(&conn, arxiv_id)
    }
    
    /// 清理旧记录
    pub fn cleanup_old_records(&self, days_old: u32) -> Result<usize> {
        let conn = self.connection_manager.create_connection()?;
        operations::delete::delete_old_papers(&conn, days_old)
    }
}

// 为了向后兼容，保留Database别名
pub type Database = DatabaseService;

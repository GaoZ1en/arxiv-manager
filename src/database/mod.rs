// 数据库模块 - 提供arXiv论文的本地存储功能

use rusqlite::Connection;
use std::path::Path;
use crate::utils::Result;

// 导入子模块
pub mod models;
pub mod operations;

// 重新导出常用类型和函数
pub use models::*;
pub use operations::*;

/// 数据库连接包装器
pub struct Database {
    conn: Connection,
}

impl std::fmt::Debug for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Database")
            .field("conn", &"<rusqlite::Connection>")
            .finish()
    }
}

impl Database {
    /// 创建新的数据库连接
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let db = Database { conn };
        operations::schema::initialize_schema(&db.conn)?;
        Ok(db)
    }
    
    /// 插入或更新论文记录
    pub fn insert_paper(&self, paper: &crate::core::ArxivPaper) -> Result<i64> {
        operations::paper_ops::insert_paper(&self.conn, paper)
    }
    
    /// 根据arXiv ID获取论文记录
    pub fn get_paper_by_arxiv_id(&self, arxiv_id: &str) -> Result<Option<PaperRecord>> {
        operations::paper_ops::get_paper_by_arxiv_id(&self.conn, arxiv_id)
    }
    
    /// 更新论文的下载状态
    pub fn update_download_status(
        &self, 
        arxiv_id: &str, 
        status: DownloadStatus, 
        local_path: Option<&str>
    ) -> Result<()> {
        operations::paper_ops::update_download_status(&self.conn, arxiv_id, status, local_path)
    }
    
    /// 搜索论文
    pub fn search_papers(&self, query: &str, limit: usize) -> Result<Vec<PaperRecord>> {
        operations::search_ops::search_papers(&self.conn, query, limit)
    }
    
    /// 获取最近的论文
    pub fn get_recent_papers(&self, limit: usize) -> Result<Vec<PaperRecord>> {
        operations::paper_ops::get_recent_papers(&self.conn, limit)
    }
    
    /// 根据下载状态获取论文列表
    pub fn get_papers_by_status(&self, status: DownloadStatus) -> Result<Vec<PaperRecord>> {
        operations::paper_ops::get_papers_by_status(&self.conn, status)
    }
    
    /// 按分类搜索论文
    pub fn search_papers_by_category(&self, category: &str, limit: usize) -> Result<Vec<PaperRecord>> {
        operations::search_ops::search_papers_by_category(&self.conn, category, limit)
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
        operations::search_ops::advanced_search(
            &self.conn, title, authors, abstract_text, categories, limit
        )
    }
}

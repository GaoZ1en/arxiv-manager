use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, Row};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub mod migrations;
pub mod models;

pub use models::*;

/// 初始化数据库
pub async fn init() -> Result<()> {
    let config = crate::config::AppConfig::load().unwrap_or_default();
    config.ensure_directories()?;

    let conn = Connection::open(&config.database.db_path)?;
    migrations::run_migrations(&conn)?;

    Ok(())
}

/// 数据库连接管理器
pub struct Database {
    conn: Connection,
}

impl Database {
    /// 创建新的数据库连接
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }

    /// 插入新论文
    pub fn insert_paper(&self, paper: &Paper) -> Result<i64> {
        let now = Utc::now();
        let id = self.conn.execute(
            "INSERT INTO papers (
                arxiv_id, title, authors, abstract, categories, 
                primary_category, published, updated, doi, journal_ref,
                pdf_url, abs_url, downloaded, download_path, tags, notes,
                read_status, reading_progress, favorite, created_at, updated_at
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21
            )",
            rusqlite::params![
                &paper.arxiv_id,
                &paper.title,
                &paper.authors.join("; "),
                &paper.abstract_text,
                &paper.categories.join("; "),
                &paper.primary_category,
                &paper.published.to_rfc3339(),
                paper.updated.as_ref().map(|d| d.to_rfc3339()),
                &paper.doi,
                &paper.journal_ref,
                &paper.pdf_url,
                &paper.abs_url,
                paper.downloaded,
                &paper.download_path,
                &paper.tags.join("; "),
                &paper.notes,
                paper.read_status as i32,
                paper.reading_progress,
                paper.favorite,
                &now.to_rfc3339(),
                &now.to_rfc3339(),
            ]
        )?;
        
        let id = self.conn.last_insert_rowid();

        Ok(id)
    }

    /// 根据 arXiv ID 查找论文
    pub fn find_paper_by_arxiv_id(&self, arxiv_id: &str) -> Result<Option<Paper>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM papers WHERE arxiv_id = ?1"
        )?;

        let mut rows = stmt.query_map([arxiv_id], |row| {
            Ok(Paper::from_row(row))
        })?;

        match rows.next() {
            Some(paper) => Ok(Some(paper??)),
            None => Ok(None),
        }
    }

    /// 获取所有论文
    pub fn get_all_papers(&self) -> Result<Vec<Paper>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM papers ORDER BY updated_at DESC"
        )?;

        let paper_iter = stmt.query_map([], |row| {
            Ok(Paper::from_row(row))
        })?;

        let mut papers = Vec::new();
        for paper in paper_iter {
            papers.push(paper??);
        }

        Ok(papers)
    }

    /// 搜索论文
    pub fn search_papers(&self, query: &str) -> Result<Vec<Paper>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM papers 
             WHERE title LIKE ?1 
                OR abstract LIKE ?1 
                OR authors LIKE ?1 
                OR categories LIKE ?1
             ORDER BY updated_at DESC"
        )?;

        let search_term = format!("%{}%", query);
        let paper_iter = stmt.query_map([&search_term], |row| {
            Ok(Paper::from_row(row))
        })?;

        let mut papers = Vec::new();
        for paper in paper_iter {
            papers.push(paper??);
        }

        Ok(papers)
    }

    /// 更新论文
    pub fn update_paper(&self, paper: &Paper) -> Result<()> {
        let now = Utc::now();
        self.conn.execute(
            "UPDATE papers SET 
                title = ?2, authors = ?3, abstract = ?4, categories = ?5,
                primary_category = ?6, published = ?7, updated = ?8, doi = ?9,
                journal_ref = ?10, pdf_url = ?11, abs_url = ?12, downloaded = ?13,
                download_path = ?14, tags = ?15, notes = ?16, read_status = ?17,
                reading_progress = ?18, favorite = ?19, updated_at = ?20
             WHERE id = ?1",
            rusqlite::params![
                paper.id,
                &paper.title,
                &paper.authors.join("; "),
                &paper.abstract_text,
                &paper.categories.join("; "),
                &paper.primary_category,
                &paper.published.to_rfc3339(),
                paper.updated.as_ref().map(|d| d.to_rfc3339()),
                &paper.doi,
                &paper.journal_ref,
                &paper.pdf_url,
                &paper.abs_url,
                paper.downloaded,
                &paper.download_path,
                &paper.tags.join("; "),
                &paper.notes,
                paper.read_status as i32,
                paper.reading_progress,
                paper.favorite,
                &now.to_rfc3339(),
            ]
        )?;

        Ok(())
    }

    /// 删除论文
    pub fn delete_paper(&self, id: i64) -> Result<()> {
        let mut stmt = self.conn.prepare("DELETE FROM papers WHERE id = ?1")?;
        stmt.execute([id])?;
        Ok(())
    }

    /// 获取统计信息
    pub fn get_statistics(&self) -> Result<Statistics> {
        let mut stmt = self.conn.prepare(
            "SELECT 
                COUNT(*) as total_papers,
                COUNT(CASE WHEN downloaded = 1 THEN 1 END) as downloaded_papers,
                COUNT(CASE WHEN favorite = 1 THEN 1 END) as favorite_papers,
                COUNT(CASE WHEN read_status = 2 THEN 1 END) as read_papers
             FROM papers"
        )?;

        let stats = stmt.query_row([], |row| {
            Ok(Statistics {
                total_papers: row.get(0)?,
                downloaded_papers: row.get(1)?,
                favorite_papers: row.get(2)?,
                read_papers: row.get(3)?,
            })
        })?;

        Ok(stats)
    }
}

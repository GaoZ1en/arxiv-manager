// 论文相关的数据库操作

use rusqlite::{Connection, params};
use chrono::Utc;
use crate::core::ArxivPaper;
use crate::utils::Result;
use super::super::models::{PaperRecord, DownloadStatus};

/// 插入或更新论文记录
pub fn insert_paper(conn: &Connection, paper: &ArxivPaper) -> Result<i64> {
    let now = Utc::now().to_rfc3339();
    let authors_json = serde_json::to_string(&paper.authors)?;
    let categories_json = serde_json::to_string(&paper.categories)?;
    
    let _id = conn.execute(
        r#"
        INSERT OR REPLACE INTO papers (
            arxiv_id, title, authors, abstract, categories,
            published, updated, pdf_url, abstract_url,
            doi, journal_ref, comments, download_status,
            tags, read_progress, created_at, updated_at
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17
        )
        "#,
        params![
            paper.id,
            paper.title,
            authors_json,
            paper.abstract_text,
            categories_json,
            paper.published,
            paper.updated,
            paper.pdf_url,
            paper.entry_url,
            paper.doi,
            paper.journal_ref,
            paper.comments,
            DownloadStatus::Pending.to_db_value(),
            "[]",
            0.0,
            now,
            now
        ],
    )?;
    
    Ok(conn.last_insert_rowid())
}

/// 根据arXiv ID获取论文记录
pub fn get_paper_by_arxiv_id(conn: &Connection, arxiv_id: &str) -> Result<Option<PaperRecord>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM papers WHERE arxiv_id = ?1"
    )?;
    
    let mut rows = stmt.query_map([arxiv_id], |row| {
        PaperRecord::from_row(row)
    })?;
    
    match rows.next() {
        Some(Ok(paper)) => Ok(Some(paper)),
        Some(Err(e)) => Err(e.into()),
        None => Ok(None),
    }
}

/// 更新论文的下载状态
pub fn update_download_status(
    conn: &Connection, 
    arxiv_id: &str, 
    status: DownloadStatus, 
    local_path: Option<&str>
) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    
    conn.execute(
        "UPDATE papers SET download_status = ?1, local_path = ?2, updated_at = ?3 WHERE arxiv_id = ?4",
        params![status.to_db_value(), local_path, now, arxiv_id],
    )?;
    
    Ok(())
}

/// 根据下载状态获取论文列表
pub fn get_papers_by_status(conn: &Connection, status: DownloadStatus) -> Result<Vec<PaperRecord>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM papers WHERE download_status = ?1"
    )?;
    
    let rows = stmt.query_map([status.to_db_value()], |row| {
        PaperRecord::from_row(row)
    })?;
    
    let mut papers = Vec::new();
    for row in rows {
        papers.push(row?);
    }
    
    Ok(papers)
}

/// 获取最近的论文
pub fn get_recent_papers(conn: &Connection, limit: usize) -> Result<Vec<PaperRecord>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM papers ORDER BY created_at DESC LIMIT ?1"
    )?;
    
    let rows = stmt.query_map([limit], |row| {
        PaperRecord::from_row(row)
    })?;
    
    let mut papers = Vec::new();
    for row in rows {
        papers.push(row?);
    }
    
    Ok(papers)
}

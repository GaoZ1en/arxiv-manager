// 创建操作模块 - 处理数据插入操作

use rusqlite::{Connection, params};
use chrono::Utc;
use crate::core::models::ArxivPaper;
use crate::utils::Result;
use super::super::models::DownloadStatus;

/// 插入新论文记录
pub fn insert_paper(conn: &Connection, paper: &ArxivPaper) -> Result<i64> {
    let now = Utc::now().to_rfc3339();
    let authors_json = serde_json::to_string(&paper.authors)?;
    let categories_json = serde_json::to_string(&paper.categories)?;
    
    let _id = conn.execute(
        r#"
        INSERT INTO papers (
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

/// 插入或更新论文记录（如果已存在则更新）
pub fn upsert_paper(conn: &Connection, paper: &ArxivPaper) -> Result<i64> {
    let now = Utc::now().to_rfc3339();
    let authors_json = serde_json::to_string(&paper.authors)?;
    let categories_json = serde_json::to_string(&paper.categories)?;
    
    let _rows_affected = conn.execute(
        r#"
        INSERT OR REPLACE INTO papers (
            arxiv_id, title, authors, abstract, categories,
            published, updated, pdf_url, abstract_url,
            doi, journal_ref, comments, download_status,
            tags, read_progress, created_at, updated_at
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, 
            COALESCE((SELECT download_status FROM papers WHERE arxiv_id = ?1), ?13),
            COALESCE((SELECT tags FROM papers WHERE arxiv_id = ?1), ?14),
            COALESCE((SELECT read_progress FROM papers WHERE arxiv_id = ?1), ?15),
            COALESCE((SELECT created_at FROM papers WHERE arxiv_id = ?1), ?16),
            ?17
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

/// 批量插入论文
pub fn batch_insert_papers(conn: &Connection, papers: &[ArxivPaper]) -> Result<Vec<i64>> {
    let tx = conn.unchecked_transaction()?;
    let mut ids = Vec::new();
    
    for paper in papers {
        let id = insert_paper(&tx, paper)?;
        ids.push(id);
    }
    
    tx.commit()?;
    Ok(ids)
}

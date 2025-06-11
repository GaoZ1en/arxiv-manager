// 查询操作模块 - 处理数据读取操作

use rusqlite::{Connection, params};
use crate::utils::Result;
use super::super::models::{PaperRecord, DownloadStatus};

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

/// 根据ID获取论文记录
pub fn get_paper_by_id(conn: &Connection, id: i64) -> Result<Option<PaperRecord>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM papers WHERE id = ?1"
    )?;
    
    let mut rows = stmt.query_map([id], |row| {
        PaperRecord::from_row(row)
    })?;
    
    match rows.next() {
        Some(Ok(paper)) => Ok(Some(paper)),
        Some(Err(e)) => Err(e.into()),
        None => Ok(None),
    }
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

/// 根据下载状态获取论文列表
pub fn get_papers_by_status(conn: &Connection, status: DownloadStatus) -> Result<Vec<PaperRecord>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM papers WHERE download_status = ?1 ORDER BY created_at DESC"
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

/// 根据分类获取论文
pub fn get_papers_by_category(conn: &Connection, category: &str, limit: usize) -> Result<Vec<PaperRecord>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM papers WHERE categories LIKE ?1 ORDER BY published DESC LIMIT ?2"
    )?;
    
    let pattern = format!("%{}%", category);
    let rows = stmt.query_map(params![pattern, limit], |row| {
        PaperRecord::from_row(row)
    })?;
    
    let mut papers = Vec::new();
    for row in rows {
        papers.push(row?);
    }
    
    Ok(papers)
}

/// 获取论文总数
pub fn get_paper_count(conn: &Connection) -> Result<i64> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM papers")?;
    let count: i64 = stmt.query_row([], |row| row.get(0))?;
    Ok(count)
}

/// 根据状态获取论文数量
pub fn get_paper_count_by_status(conn: &Connection, status: DownloadStatus) -> Result<i64> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM papers WHERE download_status = ?1")?;
    let count: i64 = stmt.query_row([status.to_db_value()], |row| row.get(0))?;
    Ok(count)
}

/// 分页获取论文
pub fn get_papers_paginated(
    conn: &Connection, 
    offset: usize, 
    limit: usize
) -> Result<Vec<PaperRecord>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM papers ORDER BY created_at DESC LIMIT ?1 OFFSET ?2"
    )?;
    
    let rows = stmt.query_map(params![limit, offset], |row| {
        PaperRecord::from_row(row)
    })?;
    
    let mut papers = Vec::new();
    for row in rows {
        papers.push(row?);
    }
    
    Ok(papers)
}

/// 检查论文是否存在
pub fn paper_exists(conn: &Connection, arxiv_id: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT 1 FROM papers WHERE arxiv_id = ?1")?;
    match stmt.query_row([arxiv_id], |_| Ok(())) {
        Ok(_) => Ok(true),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(false),
        Err(e) => Err(e.into()),
    }
}

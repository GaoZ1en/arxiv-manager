// 更新操作模块 - 处理数据更新操作

use rusqlite::{Connection, params};
use chrono::Utc;
use crate::utils::Result;
use super::super::models::DownloadStatus;

/// 更新论文的下载状态
pub fn update_download_status(
    conn: &Connection, 
    arxiv_id: &str, 
    status: DownloadStatus, 
    local_path: Option<&str>
) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    
    let rows_affected = conn.execute(
        "UPDATE papers SET download_status = ?1, local_path = ?2, updated_at = ?3 WHERE arxiv_id = ?4",
        params![status.to_db_value(), local_path, now, arxiv_id],
    )?;
    
    if rows_affected == 0 {
        return Err(crate::utils::ArxivError::Config(
            format!("未找到arXiv ID为 {} 的论文", arxiv_id)
        ));
    }
    
    Ok(())
}

/// 更新论文的阅读进度
pub fn update_read_progress(
    conn: &Connection, 
    arxiv_id: &str, 
    progress: f64
) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    let progress = progress.clamp(0.0, 1.0);
    
    let rows_affected = conn.execute(
        "UPDATE papers SET read_progress = ?1, updated_at = ?2 WHERE arxiv_id = ?3",
        params![progress, now, arxiv_id],
    )?;
    
    if rows_affected == 0 {
        return Err(crate::utils::ArxivError::Config(
            format!("未找到arXiv ID为 {} 的论文", arxiv_id)
        ));
    }
    
    Ok(())
}

/// 更新论文标签
pub fn update_paper_tags(
    conn: &Connection, 
    arxiv_id: &str, 
    tags: &[String]
) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    let tags_json = serde_json::to_string(tags)?;
    
    let rows_affected = conn.execute(
        "UPDATE papers SET tags = ?1, updated_at = ?2 WHERE arxiv_id = ?3",
        params![tags_json, now, arxiv_id],
    )?;
    
    if rows_affected == 0 {
        return Err(crate::utils::ArxivError::Config(
            format!("未找到arXiv ID为 {} 的论文", arxiv_id)
        ));
    }
    
    Ok(())
}

/// 添加标签到论文
pub fn add_tag_to_paper(
    conn: &Connection, 
    arxiv_id: &str, 
    tag: &str
) -> Result<()> {
    // 首先获取当前标签
    let mut stmt = conn.prepare("SELECT tags FROM papers WHERE arxiv_id = ?1")?;
    let tags_json: String = stmt.query_row([arxiv_id], |row| row.get(0))?;
    
    let mut tags: Vec<String> = serde_json::from_str(&tags_json)
        .unwrap_or_else(|_| Vec::new());
    
    // 避免重复标签
    if !tags.contains(&tag.to_string()) {
        tags.push(tag.to_string());
        update_paper_tags(conn, arxiv_id, &tags)?;
    }
    
    Ok(())
}

/// 从论文中移除标签
pub fn remove_tag_from_paper(
    conn: &Connection, 
    arxiv_id: &str, 
    tag: &str
) -> Result<()> {
    // 首先获取当前标签
    let mut stmt = conn.prepare("SELECT tags FROM papers WHERE arxiv_id = ?1")?;
    let tags_json: String = stmt.query_row([arxiv_id], |row| row.get(0))?;
    
    let mut tags: Vec<String> = serde_json::from_str(&tags_json)
        .unwrap_or_else(|_| Vec::new());
    
    // 移除标签
    tags.retain(|t| t != tag);
    update_paper_tags(conn, arxiv_id, &tags)?;
    
    Ok(())
}

/// 更新论文的基本信息
pub fn update_paper_info(
    conn: &Connection,
    arxiv_id: &str,
    title: Option<&str>,
    abstract_text: Option<&str>,
    authors: Option<&[String]>,
    categories: Option<&[String]>
) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    
    // 分别处理每个字段，避免生命周期问题
    if let Some(title) = title {
        conn.execute(
            "UPDATE papers SET title = ?1, updated_at = ?2 WHERE arxiv_id = ?3",
            params![title, now, arxiv_id],
        )?;
    }
    
    if let Some(abstract_text) = abstract_text {
        conn.execute(
            "UPDATE papers SET abstract = ?1, updated_at = ?2 WHERE arxiv_id = ?3",
            params![abstract_text, now, arxiv_id],
        )?;
    }
    
    if let Some(authors) = authors {
        let authors_json = serde_json::to_string(authors)?;
        conn.execute(
            "UPDATE papers SET authors = ?1, updated_at = ?2 WHERE arxiv_id = ?3",
            params![authors_json, now, arxiv_id],
        )?;
    }
    
    if let Some(categories) = categories {
        let categories_json = serde_json::to_string(categories)?;
        conn.execute(
            "UPDATE papers SET categories = ?1, updated_at = ?2 WHERE arxiv_id = ?3",
            params![categories_json, now, arxiv_id],
        )?;
    }
    
    Ok(())
}

/// 批量更新下载状态
pub fn batch_update_download_status(
    conn: &Connection,
    updates: &[(String, DownloadStatus, Option<String>)]
) -> Result<()> {
    let tx = conn.unchecked_transaction()?;
    
    for (arxiv_id, status, local_path) in updates {
        update_download_status(&tx, arxiv_id, status.clone(), local_path.as_deref())?;
    }
    
    tx.commit()?;
    Ok(())
}

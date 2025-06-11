// 删除操作模块 - 处理数据删除操作

use rusqlite::{Connection, params};
use crate::utils::Result;
use super::super::models::DownloadStatus;

/// 根据arXiv ID删除论文
pub fn delete_paper_by_arxiv_id(conn: &Connection, arxiv_id: &str) -> Result<bool> {
    let rows_affected = conn.execute(
        "DELETE FROM papers WHERE arxiv_id = ?1",
        [arxiv_id],
    )?;
    
    Ok(rows_affected > 0)
}

/// 根据ID删除论文
pub fn delete_paper_by_id(conn: &Connection, id: i64) -> Result<bool> {
    let rows_affected = conn.execute(
        "DELETE FROM papers WHERE id = ?1",
        [id],
    )?;
    
    Ok(rows_affected > 0)
}

/// 删除指定状态的所有论文
pub fn delete_papers_by_status(conn: &Connection, status: DownloadStatus) -> Result<usize> {
    let rows_affected = conn.execute(
        "DELETE FROM papers WHERE download_status = ?1",
        [status.to_db_value()],
    )?;
    
    Ok(rows_affected)
}

/// 删除旧论文（超过指定天数的）
pub fn delete_old_papers(conn: &Connection, days: u32) -> Result<usize> {
    let cutoff_date = chrono::Utc::now() - chrono::Duration::days(days as i64);
    let cutoff_str = cutoff_date.to_rfc3339();
    
    let rows_affected = conn.execute(
        "DELETE FROM papers WHERE created_at < ?1",
        [cutoff_str],
    )?;
    
    Ok(rows_affected)
}

/// 清理失败的下载
pub fn cleanup_failed_downloads(conn: &Connection) -> Result<usize> {
    let rows_affected = conn.execute(
        "UPDATE papers SET download_status = ?, local_path = NULL WHERE download_status = ?",
        params![
            DownloadStatus::Pending.to_db_value(),
            DownloadStatus::Failed("清理".to_string()).to_db_value()
        ],
    )?;
    
    Ok(rows_affected)
}

/// 删除无效的本地文件引用
pub fn cleanup_invalid_local_paths(conn: &Connection) -> Result<usize> {
    // 这里可以添加检查本地文件是否存在的逻辑
    // 目前只是示例实现
    let rows_affected = conn.execute(
        "UPDATE papers SET local_path = NULL, download_status = ? WHERE local_path IS NOT NULL AND download_status = ?",
        params![
            DownloadStatus::Pending.to_db_value(),
            DownloadStatus::Completed.to_db_value()
        ],
    )?;
    
    Ok(rows_affected)
}

/// 清空所有论文数据
pub fn clear_all_papers(conn: &Connection) -> Result<usize> {
    let rows_affected = conn.execute("DELETE FROM papers", [])?;
    
    // 重置自增ID
    conn.execute("DELETE FROM sqlite_sequence WHERE name='papers'", [])?;
    
    Ok(rows_affected)
}

/// 批量删除论文
pub fn batch_delete_papers(conn: &Connection, arxiv_ids: &[String]) -> Result<usize> {
    if arxiv_ids.is_empty() {
        return Ok(0);
    }
    
    let tx = conn.unchecked_transaction()?;
    let mut total_deleted = 0;
    
    for arxiv_id in arxiv_ids {
        if delete_paper_by_arxiv_id(&tx, arxiv_id)? {
            total_deleted += 1;
        }
    }
    
    tx.commit()?;
    Ok(total_deleted)
}

/// 删除指定分类的论文
pub fn delete_papers_by_category(conn: &Connection, category: &str) -> Result<usize> {
    let pattern = format!("%{}%", category);
    let rows_affected = conn.execute(
        "DELETE FROM papers WHERE categories LIKE ?1",
        [pattern],
    )?;
    
    Ok(rows_affected)
}

/// 删除未下载的论文
pub fn delete_undownloaded_papers(conn: &Connection) -> Result<usize> {
    let rows_affected = conn.execute(
        "DELETE FROM papers WHERE download_status IN (?, ?)",
        params![
            DownloadStatus::Pending.to_db_value(),
            DownloadStatus::Failed("".to_string()).to_db_value()
        ],
    )?;
    
    Ok(rows_affected)
}

/// 清理数据库（VACUUM操作）
pub fn vacuum_database(conn: &Connection) -> Result<()> {
    conn.execute("VACUUM", [])?;
    Ok(())
}

/// 分析数据库（ANALYZE操作）
pub fn analyze_database(conn: &Connection) -> Result<()> {
    conn.execute("ANALYZE", [])?;
    Ok(())
}

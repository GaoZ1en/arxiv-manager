// 搜索相关的数据库操作

use rusqlite::{Connection, params};
use crate::utils::Result;
use super::super::models::PaperRecord;

/// 搜索论文
pub fn search_papers(conn: &Connection, query: &str, limit: usize) -> Result<Vec<PaperRecord>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT * FROM papers 
        WHERE title LIKE ?1 OR abstract LIKE ?1 OR authors LIKE ?1
        ORDER BY updated DESC
        LIMIT ?2
        "#
    )?;
    
    let search_pattern = format!("%{}%", query);
    let rows = stmt.query_map(params![search_pattern, limit], |row| {
        PaperRecord::from_row(row)
    })?;
    
    let mut papers = Vec::new();
    for row in rows {
        papers.push(row?);
    }
    
    Ok(papers)
}

/// 按分类搜索论文
pub fn search_papers_by_category(conn: &Connection, category: &str, limit: usize) -> Result<Vec<PaperRecord>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT * FROM papers 
        WHERE categories LIKE ?1
        ORDER BY updated DESC
        LIMIT ?2
        "#
    )?;
    
    let category_pattern = format!("%{}%", category);
    let rows = stmt.query_map(params![category_pattern, limit], |row| {
        PaperRecord::from_row(row)
    })?;
    
    let mut papers = Vec::new();
    for row in rows {
        papers.push(row?);
    }
    
    Ok(papers)
}

/// 高级搜索 - 支持多个字段的组合搜索
pub fn advanced_search(
    conn: &Connection,
    title: Option<&str>,
    authors: Option<&str>,
    abstract_text: Option<&str>,
    categories: Option<&str>,
    limit: usize
) -> Result<Vec<PaperRecord>> {
    let mut where_clauses = Vec::new();
    let mut params_vec = Vec::new();
    let mut param_index = 1;
    
    if let Some(title) = title {
        where_clauses.push(format!("title LIKE ?{}", param_index));
        params_vec.push(format!("%{}%", title));
        param_index += 1;
    }
    
    if let Some(authors) = authors {
        where_clauses.push(format!("authors LIKE ?{}", param_index));
        params_vec.push(format!("%{}%", authors));
        param_index += 1;
    }
    
    if let Some(abstract_text) = abstract_text {
        where_clauses.push(format!("abstract LIKE ?{}", param_index));
        params_vec.push(format!("%{}%", abstract_text));
        param_index += 1;
    }
    
    if let Some(categories) = categories {
        where_clauses.push(format!("categories LIKE ?{}", param_index));
        params_vec.push(format!("%{}%", categories));
        param_index += 1;
    }
    
    if where_clauses.is_empty() {
        return Ok(Vec::new());
    }
    
    let where_clause = where_clauses.join(" AND ");
    let query = format!(
        "SELECT * FROM papers WHERE {} ORDER BY updated DESC LIMIT ?{}",
        where_clause, param_index
    );
    
    params_vec.push(limit.to_string());
    
    let mut stmt = conn.prepare(&query)?;
    let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter()
        .map(|p| p as &dyn rusqlite::ToSql)
        .collect();
    
    let rows = stmt.query_map(params_refs.as_slice(), |row| {
        PaperRecord::from_row(row)
    })?;
    
    let mut papers = Vec::new();
    for row in rows {
        papers.push(row?);
    }
    
    Ok(papers)
}

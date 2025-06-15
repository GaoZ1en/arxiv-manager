// 集合相关的数据库操作

use rusqlite::{Connection, params};
use chrono::Utc;
use crate::utils::Result;
use crate::core::models::Collection;

/// 插入新集合
pub fn insert_collection(conn: &Connection, collection: &Collection) -> Result<i64> {
    let mut stmt = conn.prepare(
        "INSERT INTO collections (name, description, parent_id, created_at) VALUES (?1, ?2, ?3, ?4)"
    )?;
    
    let created_at = collection.created_at.to_rfc3339();
    
    stmt.execute(params![
        collection.name,
        collection.description,
        collection.parent_id,
        created_at
    ])?;
    
    Ok(conn.last_insert_rowid())
}

/// 获取所有集合
pub fn get_all_collections(conn: &Connection) -> Result<Vec<Collection>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, parent_id, created_at FROM collections ORDER BY name"
    )?;
    
    let rows = stmt.query_map([], |row| {
        let created_at_str: String = row.get(4)?;
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|_e| rusqlite::Error::InvalidColumnType(4, "created_at".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&Utc);
            
        Ok(Collection {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            parent_id: row.get(3)?,
            created_at,
            is_expanded: true, // 默认展开
            color: None,
            icon: None,
        })
    })?;
    
    let mut collections = Vec::new();
    for row in rows {
        collections.push(row?);
    }
    
    Ok(collections)
}

/// 根据ID获取集合
pub fn get_collection_by_id(conn: &Connection, id: i64) -> Result<Option<Collection>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, parent_id, created_at FROM collections WHERE id = ?1"
    )?;
    
    let mut rows = stmt.query_map([id], |row| {
        let created_at_str: String = row.get(4)?;
        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at_str)
            .map_err(|_e| rusqlite::Error::InvalidColumnType(4, "created_at".to_string(), rusqlite::types::Type::Text))?
            .with_timezone(&Utc);
            
        Ok(Collection {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            parent_id: row.get(3)?,
            created_at,
            is_expanded: true,
            color: None,
            icon: None,
        })
    })?;
    
    match rows.next() {
        Some(collection) => Ok(Some(collection?)),
        None => Ok(None),
    }
}

/// 更新集合
pub fn update_collection(conn: &Connection, collection: &Collection) -> Result<()> {
    let mut stmt = conn.prepare(
        "UPDATE collections SET name = ?1, description = ?2, parent_id = ?3 WHERE id = ?4"
    )?;
    
    stmt.execute(params![
        collection.name,
        collection.description,
        collection.parent_id,
        collection.id
    ])?;
    
    Ok(())
}

/// 删除集合（级联删除子集合和关联关系）
pub fn delete_collection(conn: &Connection, id: i64) -> Result<()> {
    // 首先删除所有子集合
    delete_child_collections(conn, id)?;
    
    // 删除论文-集合关联
    conn.execute("DELETE FROM paper_collections WHERE collection_id = ?1", [id])?;
    
    // 删除集合本身
    conn.execute("DELETE FROM collections WHERE id = ?1", [id])?;
    
    Ok(())
}

/// 递归删除子集合
fn delete_child_collections(conn: &Connection, parent_id: i64) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id FROM collections WHERE parent_id = ?1")?;
    let child_ids: Vec<i64> = stmt.query_map([parent_id], |row| {
        Ok(row.get(0)?)
    })?.collect::<std::result::Result<Vec<_>, _>>()?;
    
    for child_id in child_ids {
        delete_collection(conn, child_id)?;
    }
    
    Ok(())
}

/// 将论文添加到集合
pub fn add_paper_to_collection(conn: &Connection, paper_id: i64, collection_id: i64) -> Result<()> {
    let added_at = Utc::now().to_rfc3339();
    
    conn.execute(
        "INSERT OR IGNORE INTO paper_collections (paper_id, collection_id, added_at) VALUES (?1, ?2, ?3)",
        params![paper_id, collection_id, added_at]
    )?;
    
    Ok(())
}

/// 从集合中移除论文
pub fn remove_paper_from_collection(conn: &Connection, paper_id: i64, collection_id: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM paper_collections WHERE paper_id = ?1 AND collection_id = ?2",
        params![paper_id, collection_id]
    )?;
    
    Ok(())
}

/// 获取集合中的论文ID列表
pub fn get_papers_in_collection(conn: &Connection, collection_id: i64) -> Result<Vec<i64>> {
    let mut stmt = conn.prepare(
        "SELECT paper_id FROM paper_collections WHERE collection_id = ?1 ORDER BY added_at DESC"
    )?;
    
    let paper_ids: Vec<i64> = stmt.query_map([collection_id], |row| {
        Ok(row.get(0)?)
    })?.collect::<std::result::Result<Vec<_>, _>>()?;
    
    Ok(paper_ids)
}

/// 获取论文所属的集合ID列表
pub fn get_collections_for_paper(conn: &Connection, paper_id: i64) -> Result<Vec<i64>> {
    let mut stmt = conn.prepare(
        "SELECT collection_id FROM paper_collections WHERE paper_id = ?1"
    )?;
    
    let collection_ids: Vec<i64> = stmt.query_map([paper_id], |row| {
        Ok(row.get(0)?)
    })?.collect::<std::result::Result<Vec<_>, _>>()?;
    
    Ok(collection_ids)
}

/// 获取集合的论文数量
pub fn get_collection_paper_count(conn: &Connection, collection_id: i64) -> Result<usize> {
    let mut stmt = conn.prepare(
        "SELECT COUNT(*) FROM paper_collections WHERE collection_id = ?1"
    )?;
    
    let count: i64 = stmt.query_row([collection_id], |row| row.get(0))?;
    Ok(count as usize)
}

/// 移动集合到新的父级
pub fn move_collection(conn: &Connection, collection_id: i64, new_parent_id: Option<i64>) -> Result<()> {
    conn.execute(
        "UPDATE collections SET parent_id = ?1 WHERE id = ?2",
        params![new_parent_id, collection_id]
    )?;
    
    Ok(())
}

/// 检查集合是否存在
pub fn collection_exists(conn: &Connection, id: i64) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM collections WHERE id = ?1")?;
    let count: i64 = stmt.query_row([id], |row| row.get(0))?;
    Ok(count > 0)
}

/// 检查集合名称是否已存在（在同一父级下）
pub fn collection_name_exists(conn: &Connection, name: &str, parent_id: Option<i64>) -> Result<bool> {
    let mut stmt = conn.prepare(
        "SELECT COUNT(*) FROM collections WHERE name = ?1 AND parent_id IS ?2"
    )?;
    
    let count: i64 = stmt.query_row(params![name, parent_id], |row| row.get(0))?;
    Ok(count > 0)
}

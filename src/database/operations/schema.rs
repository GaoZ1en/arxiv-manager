// 数据库模式初始化

use rusqlite::Connection;
use crate::utils::Result;

/// 初始化数据库模式
pub fn initialize_schema(conn: &Connection) -> Result<()> {
    // Papers table
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS papers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            arxiv_id TEXT UNIQUE NOT NULL,
            title TEXT NOT NULL,
            authors TEXT NOT NULL,
            abstract TEXT NOT NULL,
            categories TEXT NOT NULL,
            published TEXT NOT NULL,
            updated TEXT NOT NULL,
            pdf_url TEXT NOT NULL,
            abstract_url TEXT NOT NULL,
            doi TEXT,
            journal_ref TEXT,
            comments TEXT,
            local_path TEXT,
            download_status INTEGER NOT NULL DEFAULT 0,
            tags TEXT NOT NULL DEFAULT '[]',
            notes TEXT,
            rating INTEGER,
            read_progress REAL NOT NULL DEFAULT 0.0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#,
        [],
    )?;
    
    // Collections table for organizing papers
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS collections (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            description TEXT,
            created_at TEXT NOT NULL
        )
        "#,
        [],
    )?;
    
    // Paper-Collection mapping
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS paper_collections (
            paper_id INTEGER,
            collection_id INTEGER,
            added_at TEXT NOT NULL,
            PRIMARY KEY (paper_id, collection_id),
            FOREIGN KEY (paper_id) REFERENCES papers (id) ON DELETE CASCADE,
            FOREIGN KEY (collection_id) REFERENCES collections (id) ON DELETE CASCADE
        )
        "#,
        [],
    )?;
    
    // Download history
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS download_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            paper_id INTEGER NOT NULL,
            started_at TEXT NOT NULL,
            completed_at TEXT,
            status INTEGER NOT NULL,
            error_message TEXT,
            file_size INTEGER,
            FOREIGN KEY (paper_id) REFERENCES papers (id) ON DELETE CASCADE
        )
        "#,
        [],
    )?;
    
    // Create indexes for better performance
    create_indexes(conn)?;
    
    Ok(())
}

/// 创建数据库索引以提高性能
fn create_indexes(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_papers_arxiv_id ON papers (arxiv_id)",
        [],
    )?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_papers_categories ON papers (categories)",
        [],
    )?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_papers_published ON papers (published)",
        [],
    )?;
    
    Ok(())
}

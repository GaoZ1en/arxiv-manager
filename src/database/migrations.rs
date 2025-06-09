use anyhow::Result;
use rusqlite::Connection;

/// 运行数据库迁移
pub fn run_migrations(conn: &Connection) -> Result<()> {
    // 启用外键约束
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    
    // 创建版本表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER PRIMARY KEY
        )",
        [],
    )?;

    // 获取当前版本
    let current_version: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_version",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    // 运行迁移
    if current_version < 1 {
        migration_001_initial_schema(conn)?;
        conn.execute("INSERT INTO schema_version (version) VALUES (1)", [])?;
    }

    if current_version < 2 {
        migration_002_add_indexes(conn)?;
        conn.execute("INSERT INTO schema_version (version) VALUES (2)", [])?;
    }

    if current_version < 3 {
        migration_003_add_download_tasks(conn)?;
        conn.execute("INSERT INTO schema_version (version) VALUES (3)", [])?;
    }

    Ok(())
}

/// 迁移 001: 初始化数据库架构
fn migration_001_initial_schema(conn: &Connection) -> Result<()> {
    // 创建论文表
    conn.execute(
        "CREATE TABLE papers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            arxiv_id TEXT UNIQUE NOT NULL,
            title TEXT NOT NULL,
            authors TEXT NOT NULL,
            abstract TEXT NOT NULL,
            categories TEXT NOT NULL,
            primary_category TEXT NOT NULL,
            published TEXT NOT NULL,
            updated TEXT,
            doi TEXT,
            journal_ref TEXT,
            pdf_url TEXT NOT NULL,
            abs_url TEXT NOT NULL,
            downloaded BOOLEAN NOT NULL DEFAULT 0,
            download_path TEXT,
            tags TEXT NOT NULL DEFAULT '',
            notes TEXT NOT NULL DEFAULT '',
            read_status INTEGER NOT NULL DEFAULT 0,
            reading_progress REAL NOT NULL DEFAULT 0.0,
            favorite BOOLEAN NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    // 创建用户配置表
    conn.execute(
        "CREATE TABLE user_settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    // 创建搜索历史表
    conn.execute(
        "CREATE TABLE search_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            query TEXT NOT NULL,
            results_count INTEGER NOT NULL,
            created_at TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}

/// 迁移 002: 添加索引
fn migration_002_add_indexes(conn: &Connection) -> Result<()> {
    // 论文表索引
    conn.execute("CREATE INDEX idx_papers_arxiv_id ON papers(arxiv_id)", [])?;
    conn.execute("CREATE INDEX idx_papers_published ON papers(published)", [])?;
    conn.execute("CREATE INDEX idx_papers_primary_category ON papers(primary_category)", [])?;
    conn.execute("CREATE INDEX idx_papers_downloaded ON papers(downloaded)", [])?;
    conn.execute("CREATE INDEX idx_papers_favorite ON papers(favorite)", [])?;
    conn.execute("CREATE INDEX idx_papers_read_status ON papers(read_status)", [])?;
    conn.execute("CREATE INDEX idx_papers_updated_at ON papers(updated_at)", [])?;

    // 全文搜索索引（虚拟表）
    conn.execute(
        "CREATE VIRTUAL TABLE papers_fts USING fts5(
            title, authors, abstract, categories,
            content='papers',
            content_rowid='id'
        )",
        [],
    )?;

    // FTS 触发器
    conn.execute(
        "CREATE TRIGGER papers_fts_insert AFTER INSERT ON papers BEGIN
            INSERT INTO papers_fts(rowid, title, authors, abstract, categories)
            VALUES (new.id, new.title, new.authors, new.abstract, new.categories);
        END",
        [],
    )?;

    conn.execute(
        "CREATE TRIGGER papers_fts_delete AFTER DELETE ON papers BEGIN
            DELETE FROM papers_fts WHERE rowid = old.id;
        END",
        [],
    )?;

    conn.execute(
        "CREATE TRIGGER papers_fts_update AFTER UPDATE ON papers BEGIN
            DELETE FROM papers_fts WHERE rowid = old.id;
            INSERT INTO papers_fts(rowid, title, authors, abstract, categories)
            VALUES (new.id, new.title, new.authors, new.abstract, new.categories);
        END",
        [],
    )?;

    Ok(())
}

/// 迁移 003: 添加下载任务表
fn migration_003_add_download_tasks(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE download_tasks (
            id TEXT PRIMARY KEY,
            paper_id TEXT NOT NULL,
            url TEXT NOT NULL,
            file_path TEXT NOT NULL,
            status INTEGER NOT NULL DEFAULT 0,
            progress REAL NOT NULL DEFAULT 0.0,
            speed INTEGER NOT NULL DEFAULT 0,
            total_size INTEGER,
            downloaded_size INTEGER NOT NULL DEFAULT 0,
            error_message TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (paper_id) REFERENCES papers(arxiv_id)
        )",
        [],
    )?;

    // 下载任务索引
    conn.execute("CREATE INDEX idx_download_tasks_paper_id ON download_tasks(paper_id)", [])?;
    conn.execute("CREATE INDEX idx_download_tasks_status ON download_tasks(status)", [])?;
    conn.execute("CREATE INDEX idx_download_tasks_created_at ON download_tasks(created_at)", [])?;

    Ok(())
}

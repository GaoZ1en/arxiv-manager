use rusqlite::{Connection, params, Row};
use crate::core::ArxivPaper;
use crate::utils::Result;
use chrono::{DateTime, Utc};
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl std::fmt::Debug for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Database")
            .field("conn", &"<rusqlite::Connection>")
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct PaperRecord {
    pub id: i64,
    pub arxiv_id: String,
    pub title: String,
    pub authors: String, // JSON array as string
    pub abstract_text: String,
    pub categories: String, // JSON array as string
    pub published: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub pdf_url: String,
    pub abstract_url: String,
    pub doi: Option<String>,
    pub journal_ref: Option<String>,
    pub comments: Option<String>,
    pub local_path: Option<String>,
    pub download_status: DownloadStatus,
    pub tags: String, // JSON array as string
    pub notes: Option<String>,
    pub rating: Option<i32>,
    pub read_progress: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DownloadStatus {
    NotDownloaded,
    Downloading,
    Downloaded,
    Failed,
}

impl Database {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let db = Database { conn };
        db.initialize_schema()?;
        Ok(db)
    }
    
    fn initialize_schema(&self) -> Result<()> {
        // Papers table
        self.conn.execute(
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
        self.conn.execute(
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
        self.conn.execute(
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
        self.conn.execute(
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
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_papers_arxiv_id ON papers (arxiv_id)",
            [],
        )?;
        
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_papers_categories ON papers (categories)",
            [],
        )?;
        
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_papers_published ON papers (published)",
            [],
        )?;
        
        Ok(())
    }
    
    pub fn insert_paper(&self, paper: &ArxivPaper) -> Result<i64> {
        let now = Utc::now().to_rfc3339();
        let authors_json = serde_json::to_string(&paper.authors)?;
        let categories_json = serde_json::to_string(&paper.categories)?;
        
        let _id = self.conn.execute(
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
                DownloadStatus::NotDownloaded as i32,
                "[]",
                0.0,
                now,
                now
            ],
        )?;
        
        Ok(self.conn.last_insert_rowid())
    }
    
    pub fn get_paper_by_arxiv_id(&self, arxiv_id: &str) -> Result<Option<PaperRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM papers WHERE arxiv_id = ?1"
        )?;
        
        let mut rows = stmt.query_map([arxiv_id], |row| {
            self.row_to_paper_record(row)
        })?;
        
        match rows.next() {
            Some(Ok(paper)) => Ok(Some(paper)),
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
    }
    
    pub fn update_download_status(&self, arxiv_id: &str, status: DownloadStatus, local_path: Option<&str>) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        
        self.conn.execute(
            "UPDATE papers SET download_status = ?1, local_path = ?2, updated_at = ?3 WHERE arxiv_id = ?4",
            params![status as i32, local_path, now, arxiv_id],
        )?;
        
        Ok(())
    }
    
    pub fn search_papers(&self, query: &str, limit: usize) -> Result<Vec<PaperRecord>> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT * FROM papers 
            WHERE title LIKE ?1 OR abstract LIKE ?1 OR authors LIKE ?1
            ORDER BY updated DESC
            LIMIT ?2
            "#
        )?;
        
        let search_pattern = format!("%{}%", query);
        let rows = stmt.query_map(params![search_pattern, limit], |row| {
            self.row_to_paper_record(row)
        })?;
        
        let mut papers = Vec::new();
        for row in rows {
            papers.push(row?);
        }
        
        Ok(papers)
    }
    
    pub fn get_recent_papers(&self, limit: usize) -> Result<Vec<PaperRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM papers ORDER BY created_at DESC LIMIT ?1"
        )?;
        
        let rows = stmt.query_map([limit], |row| {
            self.row_to_paper_record(row)
        })?;
        
        let mut papers = Vec::new();
        for row in rows {
            papers.push(row?);
        }
        
        Ok(papers)
    }
    
    pub fn get_papers_by_status(&self, status: DownloadStatus) -> Result<Vec<PaperRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM papers WHERE download_status = ?1"
        )?;
        
        let rows = stmt.query_map([status as i32], |row| {
            self.row_to_paper_record(row)
        })?;
        
        let mut papers = Vec::new();
        for row in rows {
            papers.push(row?);
        }
        
        Ok(papers)
    }
    
    fn row_to_paper_record(&self, row: &Row) -> rusqlite::Result<PaperRecord> {
        Ok(PaperRecord {
            id: row.get(0)?,
            arxiv_id: row.get(1)?,
            title: row.get(2)?,
            authors: row.get(3)?,
            abstract_text: row.get(4)?,
            categories: row.get(5)?,
            published: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                .unwrap()
                .with_timezone(&Utc),
            updated: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                .unwrap()
                .with_timezone(&Utc),
            pdf_url: row.get(8)?,
            abstract_url: row.get(9)?,
            doi: row.get(10)?,
            journal_ref: row.get(11)?,
            comments: row.get(12)?,
            local_path: row.get(13)?,
            download_status: match row.get::<_, i32>(14)? {
                0 => DownloadStatus::NotDownloaded,
                1 => DownloadStatus::Downloading,
                2 => DownloadStatus::Downloaded,
                3 => DownloadStatus::Failed,
                _ => DownloadStatus::NotDownloaded,
            },
            tags: row.get(15)?,
            notes: row.get(16)?,
            rating: row.get(17)?,
            read_progress: row.get(18)?,
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(19)?)
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(20)?)
                .unwrap()
                .with_timezone(&Utc),
        })
    }
}

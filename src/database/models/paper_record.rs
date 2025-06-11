// 数据库中的论文记录模型

use chrono::{DateTime, Utc};
use super::DownloadStatus;

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

impl PaperRecord {
    /// 从数据库行创建PaperRecord
    pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
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
            download_status: DownloadStatus::from_db_value(row.get::<_, i32>(14)?),
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
    
    /// 解析作者JSON字符串为Vec<String>
    pub fn get_authors(&self) -> Result<Vec<String>, serde_json::Error> {
        serde_json::from_str(&self.authors)
    }
    
    /// 解析分类JSON字符串为Vec<String>
    pub fn get_categories(&self) -> Result<Vec<String>, serde_json::Error> {
        serde_json::from_str(&self.categories)
    }
    
    /// 解析标签JSON字符串为Vec<String>
    pub fn get_tags(&self) -> Result<Vec<String>, serde_json::Error> {
        serde_json::from_str(&self.tags)
    }
}

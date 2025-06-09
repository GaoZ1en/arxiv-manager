use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::Row;
use serde::{Deserialize, Serialize};

/// 论文模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paper {
    pub id: Option<i64>,
    pub arxiv_id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub abstract_text: String,
    pub categories: Vec<String>,
    pub primary_category: String,
    pub published: DateTime<Utc>,
    pub updated: Option<DateTime<Utc>>,
    pub doi: Option<String>,
    pub journal_ref: Option<String>,
    pub pdf_url: String,
    pub abs_url: String,
    pub downloaded: bool,
    pub download_path: Option<String>,
    pub tags: Vec<String>,
    pub notes: String,
    pub read_status: ReadStatus,
    pub reading_progress: f32,
    pub favorite: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// 阅读状态
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ReadStatus {
    Unread = 0,
    Reading = 1,
    Read = 2,
}

impl From<i32> for ReadStatus {
    fn from(value: i32) -> Self {
        match value {
            0 => ReadStatus::Unread,
            1 => ReadStatus::Reading,
            2 => ReadStatus::Read,
            _ => ReadStatus::Unread,
        }
    }
}

impl Paper {
    /// 从数据库行创建 Paper 实例
    pub fn from_row(row: &Row) -> Result<Self> {
        let authors_str: String = row.get("authors")?;
        let categories_str: String = row.get("categories")?;
        let tags_str: String = row.get("tags")?;
        let read_status_int: i32 = row.get("read_status")?;

        let published_str: String = row.get("published")?;
        let updated_str: Option<String> = row.get("updated")?;
        let created_at_str: Option<String> = row.get("created_at")?;
        let updated_at_str: Option<String> = row.get("updated_at")?;

        Ok(Self {
            id: Some(row.get("id")?),
            arxiv_id: row.get("arxiv_id")?,
            title: row.get("title")?,
            authors: if authors_str.is_empty() {
                Vec::new()
            } else {
                authors_str.split("; ").map(|s| s.to_string()).collect()
            },
            abstract_text: row.get("abstract")?,
            categories: if categories_str.is_empty() {
                Vec::new()
            } else {
                categories_str.split("; ").map(|s| s.to_string()).collect()
            },
            primary_category: row.get("primary_category")?,
            published: DateTime::parse_from_rfc3339(&published_str)?
                .with_timezone(&Utc),
            updated: updated_str
                .map(|s| DateTime::parse_from_rfc3339(&s))
                .transpose()?
                .map(|dt| dt.with_timezone(&Utc)),
            doi: row.get("doi")?,
            journal_ref: row.get("journal_ref")?,
            pdf_url: row.get("pdf_url")?,
            abs_url: row.get("abs_url")?,
            downloaded: row.get("downloaded")?,
            download_path: row.get("download_path")?,
            tags: if tags_str.is_empty() {
                Vec::new()
            } else {
                tags_str.split("; ").map(|s| s.to_string()).collect()
            },
            notes: row.get("notes")?,
            read_status: ReadStatus::from(read_status_int),
            reading_progress: row.get("reading_progress")?,
            favorite: row.get("favorite")?,
            created_at: created_at_str
                .map(|s| DateTime::parse_from_rfc3339(&s))
                .transpose()?
                .map(|dt| dt.with_timezone(&Utc)),
            updated_at: updated_at_str
                .map(|s| DateTime::parse_from_rfc3339(&s))
                .transpose()?
                .map(|dt| dt.with_timezone(&Utc)),
        })
    }

    /// 创建新的论文实例
    pub fn new(
        arxiv_id: String,
        title: String,
        authors: Vec<String>,
        abstract_text: String,
        categories: Vec<String>,
        primary_category: String,
        published: DateTime<Utc>,
    ) -> Self {
        Self {
            id: None,
            arxiv_id: arxiv_id.clone(),
            title,
            authors,
            abstract_text,
            categories,
            primary_category,
            published,
            updated: None,
            doi: None,
            journal_ref: None,
            pdf_url: format!("https://arxiv.org/pdf/{}.pdf", arxiv_id),
            abs_url: format!("https://arxiv.org/abs/{}", arxiv_id),
            downloaded: false,
            download_path: None,
            tags: Vec::new(),
            notes: String::new(),
            read_status: ReadStatus::Unread,
            reading_progress: 0.0,
            favorite: false,
            created_at: None,
            updated_at: None,
        }
    }

    /// 添加标签
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// 移除标签
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    /// 设置阅读状态
    pub fn set_read_status(&mut self, status: ReadStatus) {
        self.read_status = status;
        if status == ReadStatus::Read {
            self.reading_progress = 100.0;
        }
    }

    /// 更新阅读进度
    pub fn update_reading_progress(&mut self, progress: f32) {
        self.reading_progress = progress.clamp(0.0, 100.0);
        
        if self.reading_progress >= 100.0 {
            self.read_status = ReadStatus::Read;
        } else if self.reading_progress > 0.0 && self.read_status == ReadStatus::Unread {
            self.read_status = ReadStatus::Reading;
        }
    }

    /// 获取格式化的作者列表
    pub fn formatted_authors(&self) -> String {
        if self.authors.is_empty() {
            "Unknown".to_string()
        } else if self.authors.len() <= 3 {
            self.authors.join(", ")
        } else {
            format!("{} et al.", self.authors[0])
        }
    }

    /// 获取主要类别的显示名称
    pub fn category_display_name(&self) -> &str {
        match self.primary_category.as_str() {
            "cs.AI" => "Artificial Intelligence",
            "cs.CL" => "Computation and Language",
            "cs.CV" => "Computer Vision",
            "cs.LG" => "Machine Learning",
            "stat.ML" => "Machine Learning (Statistics)",
            "math.OC" => "Optimization and Control",
            "q-bio.NC" => "Neurons and Cognition",
            "physics.comp-ph" => "Computational Physics",
            _ => &self.primary_category,
        }
    }
}

/// 统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistics {
    pub total_papers: i64,
    pub downloaded_papers: i64,
    pub favorite_papers: i64,
    pub read_papers: i64,
}

/// 下载任务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadTask {
    pub id: String,
    pub paper_id: String,
    pub url: String,
    pub file_path: String,
    pub status: DownloadStatus,
    pub progress: f32,
    pub speed: u64,
    pub total_size: Option<u64>,
    pub downloaded_size: u64,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 下载状态
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DownloadStatus {
    Pending = 0,
    Downloading = 1,
    Paused = 2,
    Completed = 3,
    Failed = 4,
    Cancelled = 5,
}

impl From<i32> for DownloadStatus {
    fn from(value: i32) -> Self {
        match value {
            0 => DownloadStatus::Pending,
            1 => DownloadStatus::Downloading,
            2 => DownloadStatus::Paused,
            3 => DownloadStatus::Completed,
            4 => DownloadStatus::Failed,
            5 => DownloadStatus::Cancelled,
            _ => DownloadStatus::Pending,
        }
    }
}

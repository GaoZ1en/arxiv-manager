// 论文存储库接口 - Paper Repository Interfaces
// 定义数据访问的抽象接口，遵循依赖倒置原则

use super::models::*;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use crate::application::dto::PaperStatsResponse;

/// 论文存储库接口 - 定义论文数据访问的所有操作
#[async_trait]
pub trait PaperRepository: Send + Sync {
    /// 保存新论文
    async fn save(&self, paper: Paper) -> Result<(), RepositoryError>;
    
    /// 更新已存在的论文
    async fn update(&self, paper: Paper) -> Result<(), RepositoryError>;
    
    /// 通过ID查找论文
    async fn find_by_id(&self, id: &PaperId) -> Result<Option<Paper>, RepositoryError>;
    
    /// 检查论文是否存在
    async fn exists(&self, id: &PaperId) -> Result<bool, RepositoryError>;
    
    /// 删除论文
    async fn delete(&self, id: &PaperId) -> Result<(), RepositoryError>;
    
    /// 获取所有论文
    async fn find_all(&self) -> Result<Vec<Paper>, RepositoryError>;
    
    /// 分页获取论文
    async fn find_with_pagination(&self, page: u32, size: u32) -> Result<PaginatedResult<Paper>, RepositoryError>;
    
    /// 按分类查找论文
    async fn find_by_category(&self, category: &ArxivCategory) -> Result<Vec<Paper>, RepositoryError>;
    
    /// 按标签查找论文
    async fn find_by_tag(&self, tag_name: &str) -> Result<Vec<Paper>, RepositoryError>;
    
    /// 按阅读状态查找论文
    async fn find_by_reading_status(&self, status: ReadingStatus) -> Result<Vec<Paper>, RepositoryError>;
    
    /// 按作者查找论文
    async fn find_by_author(&self, author_name: &str) -> Result<Vec<Paper>, RepositoryError>;
    
    /// 全文搜索论文
    async fn search(&self, query: &SearchQuery) -> Result<Vec<Paper>, RepositoryError>;
    
    /// 获取最近添加的论文
    async fn find_recently_added(&self, limit: u32) -> Result<Vec<Paper>, RepositoryError>;
    
    /// 获取最近阅读的论文
    async fn find_recently_read(&self, limit: u32) -> Result<Vec<Paper>, RepositoryError>;
    
    /// 按评分查找论文
    async fn find_by_rating(&self, min_rating: u8) -> Result<Vec<Paper>, RepositoryError>;
    
    /// 获取统计信息
    async fn get_statistics(&self) -> Result<PaperStatistics, RepositoryError>;
    
    /// 批量保存论文
    async fn save_batch(&self, papers: Vec<Paper>) -> Result<(), RepositoryError>;
    
    /// 批量删除论文
    async fn delete_batch(&self, ids: Vec<&PaperId>) -> Result<(), RepositoryError>;
}

/// 论文查询接口 - 专门用于复杂查询操作
#[async_trait]
pub trait PaperQueryRepository: Send + Sync {
    /// 复合条件查询
    async fn find_by_criteria(&self, criteria: PaperSearchCriteria) -> Result<Vec<Paper>, RepositoryError>;
    
    /// 获取分类统计
    async fn get_category_statistics(&self) -> Result<Vec<CategoryStatistic>, RepositoryError>;
    
    /// 获取标签统计
    async fn get_tag_statistics(&self) -> Result<Vec<TagStatistic>, RepositoryError>;
    
    /// 获取作者统计
    async fn get_author_statistics(&self) -> Result<Vec<AuthorStatistic>, RepositoryError>;
    
    /// 获取阅读进度统计
    async fn get_reading_progress(&self) -> Result<ReadingProgressStatistics, RepositoryError>;
    
    /// 推荐相似论文
    async fn find_similar_papers(&self, paper_id: &PaperId, limit: u32) -> Result<Vec<Paper>, RepositoryError>;
    
    /// 获取趋势论文（最近热门）
    async fn find_trending_papers(&self, days: u32, limit: u32) -> Result<Vec<Paper>, RepositoryError>;
    
    /// 根据ID查找论文
    async fn find_by_id(&self, id: &PaperId) -> Result<Option<Paper>, RepositoryError>;
    
    /// 搜索论文
    async fn search_papers(
        &self,
        query: Option<&str>,
        authors: Option<&str>,
        categories: Option<&str>,
        date_from: Option<DateTime<Utc>>,
        date_to: Option<DateTime<Utc>>,
        reading_status: Option<ReadingStatus>,
        tags: Option<&str>,
        rating_min: Option<u8>,
        rating_max: Option<u8>,
        is_favorite: Option<bool>,
        local_state: Option<&str>,
        offset: u32,
        limit: u32,
    ) -> Result<(Vec<Paper>, u64), RepositoryError>;

    /// 获取统计信息
    async fn get_statistics(
        &self,
        date_from: Option<DateTime<Utc>>,
        date_to: Option<DateTime<Utc>>,
        categories: Option<&str>,
    ) -> Result<PaperStatsResponse, RepositoryError>;
}

/// 论文缓存接口 - 用于提高查询性能
#[async_trait]
pub trait PaperCacheRepository: Send + Sync {
    /// 缓存论文
    async fn cache_paper(&self, paper: &Paper) -> Result<(), RepositoryError>;
    
    /// 从缓存获取论文
    async fn get_cached_paper(&self, id: &PaperId) -> Result<Option<Paper>, RepositoryError>;
    
    /// 使缓存失效
    async fn invalidate_cache(&self, id: &PaperId) -> Result<(), RepositoryError>;
    
    /// 清空所有缓存
    async fn clear_cache(&self) -> Result<(), RepositoryError>;
    
    /// 预热缓存
    async fn warm_cache(&self, papers: Vec<Paper>) -> Result<(), RepositoryError>;
}

// 数据传输对象和查询条件

/// 分页结果
#[derive(Debug, Clone)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total_count: u64,
    pub page: u32,
    pub size: u32,
    pub total_pages: u32,
}

/// 搜索查询条件
#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub text: String,
    pub fields: Vec<SearchField>, // 搜索字段：标题、摘要、作者等
    pub fuzzy: bool,              // 是否模糊搜索
    pub boost: Option<FieldBoost>, // 字段权重
}

/// 搜索字段枚举
#[derive(Debug, Clone)]
pub enum SearchField {
    Title,
    Abstract,
    Authors,
    Keywords,
    All,
}

/// 字段权重配置
#[derive(Debug, Clone)]
pub struct FieldBoost {
    pub title: f32,
    pub abstract_text: f32,
    pub authors: f32,
    pub keywords: f32,
}

/// 论文搜索条件
#[derive(Debug, Clone, Default)]
pub struct PaperSearchCriteria {
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub authors: Vec<String>,
    pub reading_status: Option<ReadingStatus>,
    pub min_rating: Option<u8>,
    pub max_rating: Option<u8>,
    pub date_range: Option<DateRange>,
    pub has_local_files: Option<bool>,
    pub has_notes: Option<bool>,
    pub collections: Vec<CollectionId>,
    pub sort_by: Option<SortField>,
    pub sort_order: Option<SortOrder>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// 日期范围
#[derive(Debug, Clone)]
pub struct DateRange {
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
}

/// 排序字段
#[derive(Debug, Clone)]
pub enum SortField {
    CreatedAt,
    UpdatedAt,
    PublishedDate,
    Title,
    Rating,
    ReadingProgress,
}

/// 排序顺序
#[derive(Debug, Clone)]
pub enum SortOrder {
    Ascending,
    Descending,
}

/// 论文统计信息
#[derive(Debug, Clone)]
pub struct PaperStatistics {
    pub total_papers: u64,
    pub read_papers: u64,
    pub unread_papers: u64,
    pub reading_papers: u64,
    pub papers_with_notes: u64,
    pub papers_with_ratings: u64,
    pub total_categories: u64,
    pub total_tags: u64,
    pub total_authors: u64,
    pub average_rating: Option<f32>,
    pub storage_size: u64, // bytes
}

/// 分类统计
#[derive(Debug, Clone)]
pub struct CategoryStatistic {
    pub category: ArxivCategory,
    pub paper_count: u64,
    pub read_count: u64,
    pub average_rating: Option<f32>,
}

/// 标签统计
#[derive(Debug, Clone)]
pub struct TagStatistic {
    pub tag: Tag,
    pub paper_count: u64,
    pub usage_frequency: f32,
}

/// 作者统计
#[derive(Debug, Clone)]
pub struct AuthorStatistic {
    pub author: Author,
    pub paper_count: u64,
    pub average_rating: Option<f32>,
    pub most_recent_paper: Option<chrono::DateTime<chrono::Utc>>,
}

/// 阅读进度统计
#[derive(Debug, Clone)]
pub struct ReadingProgressStatistics {
    pub total_papers: u64,
    pub read_percentage: f32,
    pub current_reading: u64,
    pub want_to_read: u64,
    pub reading_velocity: f32, // 论文/天
    pub estimated_completion_days: Option<u32>,
}

/// 存储库错误类型
#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database connection error: {0}")]
    ConnectionError(String),
    
    #[error("Query execution error: {0}")]
    QueryError(String),
    
    #[error("Data serialization error: {0}")]
    SerializationError(String),
    
    #[error("Data not found")]
    NotFound,
    
    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),
    
    #[error("Transaction error: {0}")]
    TransactionError(String),
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("Permission denied")]
    PermissionDenied,
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

// 默认实现
impl Default for FieldBoost {
    fn default() -> Self {
        Self {
            title: 2.0,
            abstract_text: 1.0,
            authors: 1.5,
            keywords: 1.2,
        }
    }
}

impl<T> PaginatedResult<T> {
    pub fn new(items: Vec<T>, total_count: u64, page: u32, size: u32) -> Self {
        let total_pages = if size > 0 {
            (total_count as f64 / size as f64).ceil() as u32
        } else {
            0
        };
        
        Self {
            items,
            total_count,
            page,
            size,
            total_pages,
        }
    }
    
    pub fn has_next_page(&self) -> bool {
        self.page < self.total_pages
    }
    
    pub fn has_previous_page(&self) -> bool {
        self.page > 1
    }
}

impl PaperSearchCriteria {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_category(mut self, category: String) -> Self {
        self.categories.push(category);
        self
    }
    
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tags.push(tag);
        self
    }
    
    pub fn with_author(mut self, author: String) -> Self {
        self.authors.push(author);
        self
    }
    
    pub fn with_reading_status(mut self, status: ReadingStatus) -> Self {
        self.reading_status = Some(status);
        self
    }
    
    pub fn with_rating_range(mut self, min: u8, max: u8) -> Self {
        self.min_rating = Some(min);
        self.max_rating = Some(max);
        self
    }
    
    pub fn with_date_range(mut self, start: chrono::DateTime<chrono::Utc>, end: chrono::DateTime<chrono::Utc>) -> Self {
        self.date_range = Some(DateRange { start, end });
        self
    }
    
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
    
    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }
    
    pub fn sort_by(mut self, field: SortField, order: SortOrder) -> Self {
        self.sort_by = Some(field);
        self.sort_order = Some(order);
        self
    }
}

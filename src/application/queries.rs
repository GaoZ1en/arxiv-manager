// 查询处理 - Queries
// 实现只读数据查询操作

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domains::paper::*;
use crate::application::dto::{PaperResponse, PaperListResponse, PaperStatsResponse, SortOrder};

/// 查询特征 - 所有查询必须实现此特征
pub trait Query {
    type Response;
    
    /// 获取查询名称
    fn query_name(&self) -> &'static str;
    
    /// 验证查询参数
    fn validate(&self) -> Result<(), String>;
}

/// 根据ID获取论文查询
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPaperByIdQuery {
    pub paper_id: String,
}

impl Query for GetPaperByIdQuery {
    type Response = Option<PaperResponse>;
    
    fn query_name(&self) -> &'static str {
        "GetPaperById"
    }
    
    fn validate(&self) -> Result<(), String> {
        if self.paper_id.is_empty() {
            return Err("Paper ID cannot be empty".to_string());
        }
        Ok(())
    }
}

/// 搜索论文查询
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchPapersQuery {
    // 文本搜索
    pub query: Option<String>,
    pub search_fields: Vec<SearchField>, // 指定搜索哪些字段
    
    // 过滤条件
    pub authors: Option<Vec<String>>,
    pub categories: Option<Vec<ArxivCategory>>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub reading_status: Option<ReadingStatus>,
    pub tags: Option<Vec<String>>,
    pub rating_min: Option<u8>,
    pub rating_max: Option<u8>,
    pub is_favorite: Option<bool>,
    pub local_state: Option<LocalPaperState>,
    pub has_notes: Option<bool>,
    pub has_local_file: Option<bool>,
    
    // 分页和排序
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub sort_by: Option<SortField>,
    pub sort_order: Option<SortOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchField {
    Title,
    Authors,
    Abstract,
    Categories,
    Tags,
    Notes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortField {
    Title,
    PublishedDate,
    UpdatedDate,
    CreatedAt,
    UpdatedAt,
    Rating,
    ReadingStatus,
}

impl Query for SearchPapersQuery {
    type Response = PaperListResponse;
    
    fn query_name(&self) -> &'static str {
        "SearchPapers"
    }
    
    fn validate(&self) -> Result<(), String> {
        if let Some(page) = self.page {
            if page == 0 {
                return Err("Page number must be greater than 0".to_string());
            }
        }
        
        if let Some(page_size) = self.page_size {
            if page_size == 0 || page_size > 1000 {
                return Err("Page size must be between 1 and 1000".to_string());
            }
        }
        
        if let Some(rating_min) = self.rating_min {
            if rating_min > 5 {
                return Err("Minimum rating must be between 1 and 5".to_string());
            }
        }
        
        if let Some(rating_max) = self.rating_max {
            if rating_max > 5 {
                return Err("Maximum rating must be between 1 and 5".to_string());
            }
        }
        
        if let (Some(min), Some(max)) = (self.rating_min, self.rating_max) {
            if min > max {
                return Err("Minimum rating cannot be greater than maximum rating".to_string());
            }
        }
        
        if let (Some(from), Some(to)) = (self.date_from, self.date_to) {
            if from > to {
                return Err("Start date cannot be after end date".to_string());
            }
        }
        
        Ok(())
    }
}

/// 获取论文统计查询
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPaperStatsQuery {
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub categories: Option<Vec<ArxivCategory>>,
}

impl Query for GetPaperStatsQuery {
    type Response = PaperStatsResponse;
    
    fn query_name(&self) -> &'static str {
        "GetPaperStats"
    }
    
    fn validate(&self) -> Result<(), String> {
        if let (Some(from), Some(to)) = (self.date_from, self.date_to) {
            if from > to {
                return Err("Start date cannot be after end date".to_string());
            }
        }
        Ok(())
    }
}

/// 获取相似论文查询
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSimilarPapersQuery {
    pub paper_id: String,
    pub limit: Option<u32>,
    pub similarity_threshold: Option<f32>,
}

impl Query for GetSimilarPapersQuery {
    type Response = Vec<PaperResponse>;
    
    fn query_name(&self) -> &'static str {
        "GetSimilarPapers"
    }
    
    fn validate(&self) -> Result<(), String> {
        if self.paper_id.is_empty() {
            return Err("Paper ID cannot be empty".to_string());
        }
        
        if let Some(limit) = self.limit {
            if limit == 0 || limit > 100 {
                return Err("Limit must be between 1 and 100".to_string());
            }
        }
        
        if let Some(threshold) = self.similarity_threshold {
            if threshold < 0.0 || threshold > 1.0 {
                return Err("Similarity threshold must be between 0.0 and 1.0".to_string());
            }
        }
        
        Ok(())
    }
}

/// 获取最近添加的论文查询
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecentPapersQuery {
    pub limit: Option<u32>,
    pub days: Option<u32>, // 最近多少天
}

impl Query for GetRecentPapersQuery {
    type Response = Vec<PaperResponse>;
    
    fn query_name(&self) -> &'static str {
        "GetRecentPapers"
    }
    
    fn validate(&self) -> Result<(), String> {
        if let Some(limit) = self.limit {
            if limit == 0 || limit > 1000 {
                return Err("Limit must be between 1 and 1000".to_string());
            }
        }
        
        if let Some(days) = self.days {
            if days == 0 || days > 365 {
                return Err("Days must be between 1 and 365".to_string());
            }
        }
        
        Ok(())
    }
}

/// 获取收藏论文查询
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFavoritePapersQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub sort_by: Option<SortField>,
    pub sort_order: Option<SortOrder>,
}

impl Query for GetFavoritePapersQuery {
    type Response = PaperListResponse;
    
    fn query_name(&self) -> &'static str {
        "GetFavoritePapers"
    }
    
    fn validate(&self) -> Result<(), String> {
        if let Some(page) = self.page {
            if page == 0 {
                return Err("Page number must be greater than 0".to_string());
            }
        }
        
        if let Some(page_size) = self.page_size {
            if page_size == 0 || page_size > 1000 {
                return Err("Page size must be between 1 and 1000".to_string());
            }
        }
        
        Ok(())
    }
}

/// 根据标签获取论文查询
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPapersByTagsQuery {
    pub tags: Vec<String>,
    pub match_all: bool, // true: 匹配所有标签, false: 匹配任一标签
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

impl Query for GetPapersByTagsQuery {
    type Response = PaperListResponse;
    
    fn query_name(&self) -> &'static str {
        "GetPapersByTags"
    }
    
    fn validate(&self) -> Result<(), String> {
        if self.tags.is_empty() {
            return Err("Tags list cannot be empty".to_string());
        }
        
        for tag in &self.tags {
            if tag.is_empty() {
                return Err("Tag cannot be empty".to_string());
            }
        }
        
        if let Some(page) = self.page {
            if page == 0 {
                return Err("Page number must be greater than 0".to_string());
            }
        }
        
        if let Some(page_size) = self.page_size {
            if page_size == 0 || page_size > 1000 {
                return Err("Page size must be between 1 and 1000".to_string());
            }
        }
        
        Ok(())
    }
}

/// 根据分类获取论文查询
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPapersByCategoryQuery {
    pub category: ArxivCategory,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub sort_by: Option<SortField>,
    pub sort_order: Option<SortOrder>,
}

impl Query for GetPapersByCategoryQuery {
    type Response = PaperListResponse;
    
    fn query_name(&self) -> &'static str {
        "GetPapersByCategory"
    }
    
    fn validate(&self) -> Result<(), String> {
        if let Some(page) = self.page {
            if page == 0 {
                return Err("Page number must be greater than 0".to_string());
            }
        }
        
        if let Some(page_size) = self.page_size {
            if page_size == 0 || page_size > 1000 {
                return Err("Page size must be between 1 and 1000".to_string());
            }
        }
        
        Ok(())
    }
}

/// 获取阅读进度查询
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReadingProgressQuery {
    pub reading_status: Option<ReadingStatus>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

impl Query for GetReadingProgressQuery {
    type Response = PaperListResponse;
    
    fn query_name(&self) -> &'static str {
        "GetReadingProgress"
    }
    
    fn validate(&self) -> Result<(), String> {
        if let Some(page) = self.page {
            if page == 0 {
                return Err("Page number must be greater than 0".to_string());
            }
        }
        
        if let Some(page_size) = self.page_size {
            if page_size == 0 || page_size > 1000 {
                return Err("Page size must be between 1 and 1000".to_string());
            }
        }
        
        Ok(())
    }
}

/// 搜索建议查询
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSearchSuggestionsQuery {
    pub query: String,
    pub suggestion_type: SuggestionType,
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionType {
    Authors,
    Tags,
    Categories,
    Titles,
}

impl Query for GetSearchSuggestionsQuery {
    type Response = Vec<String>;
    
    fn query_name(&self) -> &'static str {
        "GetSearchSuggestions"
    }
    
    fn validate(&self) -> Result<(), String> {
        if self.query.is_empty() {
            return Err("Query cannot be empty".to_string());
        }
        
        if let Some(limit) = self.limit {
            if limit == 0 || limit > 100 {
                return Err("Limit must be between 1 and 100".to_string());
            }
        }
        
        Ok(())
    }
}

/// 查询执行结果
#[derive(Debug, Clone)]
pub enum QueryResult<T> {
    Success(T),
    ValidationError(String),
    NotFound,
    InfrastructureError(String),
}

impl<T> QueryResult<T> {
    pub fn is_success(&self) -> bool {
        matches!(self, QueryResult::Success(_))
    }
    
    pub fn unwrap(self) -> T {
        match self {
            QueryResult::Success(value) => value,
            QueryResult::ValidationError(msg) => panic!("Validation error: {}", msg),
            QueryResult::NotFound => panic!("Resource not found"),
            QueryResult::InfrastructureError(msg) => panic!("Infrastructure error: {}", msg),
        }
    }
    
    pub fn map<U, F>(self, f: F) -> QueryResult<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            QueryResult::Success(value) => QueryResult::Success(f(value)),
            QueryResult::ValidationError(msg) => QueryResult::ValidationError(msg),
            QueryResult::NotFound => QueryResult::NotFound,
            QueryResult::InfrastructureError(msg) => QueryResult::InfrastructureError(msg),
        }
    }
    
    pub fn ok(self) -> Option<T> {
        match self {
            QueryResult::Success(value) => Some(value),
            _ => None,
        }
    }
}

/// 查询构建器帮助类
#[derive(Debug, Default)]
pub struct SearchPapersQueryBuilder {
    query: SearchPapersQuery,
}

impl SearchPapersQueryBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_text_query(mut self, query: String) -> Self {
        self.query.query = Some(query);
        self
    }
    
    pub fn with_authors(mut self, authors: Vec<String>) -> Self {
        self.query.authors = Some(authors);
        self
    }
    
    pub fn with_categories(mut self, categories: Vec<ArxivCategory>) -> Self {
        self.query.categories = Some(categories);
        self
    }
    
    pub fn with_date_range(mut self, from: DateTime<Utc>, to: DateTime<Utc>) -> Self {
        self.query.date_from = Some(from);
        self.query.date_to = Some(to);
        self
    }
    
    pub fn with_reading_status(mut self, status: ReadingStatus) -> Self {
        self.query.reading_status = Some(status);
        self
    }
    
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.query.tags = Some(tags);
        self
    }
    
    pub fn with_rating_range(mut self, min: u8, max: u8) -> Self {
        self.query.rating_min = Some(min);
        self.query.rating_max = Some(max);
        self
    }
    
    pub fn favorites_only(mut self) -> Self {
        self.query.is_favorite = Some(true);
        self
    }
    
    pub fn with_pagination(mut self, page: u32, page_size: u32) -> Self {
        self.query.page = Some(page);
        self.query.page_size = Some(page_size);
        self
    }
    
    pub fn sort_by(mut self, field: SortField, order: SortOrder) -> Self {
        self.query.sort_by = Some(field);
        self.query.sort_order = Some(order);
        self
    }
    
    pub fn build(self) -> SearchPapersQuery {
        self.query
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_paper_by_id_query_validation() {
        let query = GetPaperByIdQuery {
            paper_id: "2301.12345".to_string(),
        };
        assert!(query.validate().is_ok());
        
        let invalid_query = GetPaperByIdQuery {
            paper_id: String::new(),
        };
        assert!(invalid_query.validate().is_err());
    }
    
    #[test]
    fn test_search_papers_query_validation() {
        let mut query = SearchPapersQuery::default();
        assert!(query.validate().is_ok());
        
        // 测试无效的页码
        query.page = Some(0);
        assert!(query.validate().is_err());
        
        // 测试无效的评分范围
        query.page = Some(1);
        query.rating_min = Some(5);
        query.rating_max = Some(3);
        assert!(query.validate().is_err());
    }
    
    #[test]
    fn test_search_query_builder() {
        let query = SearchPapersQueryBuilder::new()
            .with_text_query("machine learning".to_string())
            .with_reading_status(ReadingStatus::Unread)
            .with_pagination(1, 20)
            .favorites_only()
            .build();
        
        assert_eq!(query.query, Some("machine learning".to_string()));
        assert_eq!(query.reading_status, Some(ReadingStatus::Unread));
        assert_eq!(query.page, Some(1));
        assert_eq!(query.page_size, Some(20));
        assert_eq!(query.is_favorite, Some(true));
    }
    
    #[test]
    fn test_query_result() {
        let success: QueryResult<String> = QueryResult::Success("test".to_string());
        assert!(success.is_success());
        
        let mapped = success.map(|s| s.len());
        match mapped {
            QueryResult::Success(len) => assert_eq!(len, 4),
            _ => panic!("Expected success"),
        }
        
        let not_found: QueryResult<String> = QueryResult::NotFound;
        assert!(not_found.ok().is_none());
    }
}

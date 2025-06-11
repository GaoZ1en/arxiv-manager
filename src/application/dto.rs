// 数据传输对象 - Data Transfer Objects
// 定义应用层与外部交互的数据结构

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domains::paper::*;

/// 论文创建请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaperRequest {
    pub arxiv_id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub abstract_text: String,
    pub categories: Vec<String>,
    pub published_date: DateTime<Utc>,
    pub updated_date: Option<DateTime<Utc>>,
    pub pdf_url: Option<String>,
    pub arxiv_url: String,
    pub doi: Option<String>,
    pub journal_ref: Option<String>,
    pub comments: Option<String>,
}

/// 论文更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePaperRequest {
    pub paper_id: String,
    pub title: Option<String>,
    pub authors: Option<Vec<String>>,
    pub abstract_text: Option<String>,
    pub categories: Option<Vec<String>>,
    pub updated_date: Option<DateTime<Utc>>,
    pub pdf_url: Option<String>,
    pub doi: Option<String>,
    pub journal_ref: Option<String>,
    pub comments: Option<String>,
}

/// 论文状态更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePaperStatusRequest {
    pub paper_id: String,
    pub reading_status: Option<ReadingStatus>,
    pub tags: Option<Vec<String>>,
    pub rating: Option<u8>,
    pub notes: Option<String>,
    pub is_favorite: Option<bool>,
}

/// 论文查询请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaperSearchRequest {
    pub query: Option<String>,
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
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<SortOrder>,
}

/// 排序顺序
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}

/// 论文响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperResponse {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub abstract_text: String,
    pub categories: Vec<String>,
    pub published_date: DateTime<Utc>,
    pub updated_date: Option<DateTime<Utc>>,
    pub pdf_url: Option<String>,
    pub arxiv_url: String,
    pub doi: Option<String>,
    pub journal_ref: Option<String>,
    pub comments: Option<String>,
    
    // 本地状态
    pub reading_status: ReadingStatus,
    pub tags: Vec<String>,
    pub rating: Option<u8>,
    pub notes: Option<String>,
    pub is_favorite: bool,
    pub local_state: LocalPaperState,
    pub local_file_path: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 论文列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperListResponse {
    pub papers: Vec<PaperResponse>,
    pub total_count: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

/// 论文统计响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperStatsResponse {
    pub total_papers: u64,
    pub unread_papers: u64,
    pub reading_papers: u64,
    pub completed_papers: u64,
    pub favorite_papers: u64,
    pub downloaded_papers: u64,
    pub papers_by_category: Vec<CategoryStats>,
    pub papers_by_rating: Vec<RatingStats>,
}

/// 分类统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryStats {
    pub category: String,
    pub count: u64,
}

/// 评分统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingStats {
    pub rating: u8,
    pub count: u64,
}

/// 下载请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadPaperRequest {
    pub paper_id: String,
    pub download_path: Option<String>,
    pub force_redownload: bool,
}

/// 下载响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadPaperResponse {
    pub paper_id: String,
    pub file_path: String,
    pub file_size: u64,
    pub downloaded_at: DateTime<Utc>,
}

/// 批量操作请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOperationRequest {
    pub paper_ids: Vec<String>,
    pub operation: BatchOperation,
}

/// 批量操作类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BatchOperation {
    UpdateReadingStatus(ReadingStatus),
    AddTags(Vec<String>),
    RemoveTags(Vec<String>),
    SetRating(Option<u8>),
    SetFavorite(bool),
    Delete,
    Download,
}

/// 批量操作响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOperationResponse {
    pub succeeded: Vec<String>,
    pub failed: Vec<BatchOperationError>,
    pub total_processed: u32,
}

/// 批量操作错误
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOperationError {
    pub paper_id: String,
    pub error_message: String,
}

/// 导入请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportRequest {
    pub source: ImportSource,
    pub data: String,
    pub options: ImportOptions,
}

/// 导入源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportSource {
    ArxivApi,
    BibtexFile,
    JsonFile,
    CsvFile,
}

/// 导入选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportOptions {
    pub skip_duplicates: bool,
    pub update_existing: bool,
    pub auto_download: bool,
    pub default_tags: Vec<String>,
}

/// 导入响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResponse {
    pub imported_count: u32,
    pub skipped_count: u32,
    pub failed_count: u32,
    pub errors: Vec<ImportError>,
}

/// 导入错误
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportError {
    pub item_id: String,
    pub error_message: String,
}

/// 导出请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    pub format: ExportFormat,
    pub filter: Option<PaperSearchRequest>,
    pub include_files: bool,
    pub output_path: String,
}

/// 导出格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    Json,
    Csv,
    Bibtex,
    Pdf, // 将所有论文合并为一个PDF
}

/// 导出响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResponse {
    pub file_path: String,
    pub exported_count: u32,
    pub file_size: u64,
}

/// 收藏夹操作请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionRequest {
    pub name: String,
    pub description: Option<String>,
    pub paper_ids: Vec<String>,
}

/// 收藏夹响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub paper_count: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 应用错误
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationError {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl ApplicationError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: None,
            timestamp: Utc::now(),
        }
    }
    
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)?;
        if let Some(details) = &self.details {
            write!(f, ": {}", details)?;
        }
        Ok(())
    }
}

impl std::error::Error for ApplicationError {}

/// 应用结果类型
pub type ApplicationResult<T> = Result<T, ApplicationError>;

/// 分页信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u32,
    pub page_size: u32,
    pub total_items: u64,
    pub total_pages: u32,
}

impl Pagination {
    pub fn new(page: u32, page_size: u32, total_items: u64) -> Self {
        let total_pages = if total_items == 0 {
            0
        } else {
            ((total_items - 1) / page_size as u64 + 1) as u32
        };
        
        Self {
            page,
            page_size,
            total_items,
            total_pages,
        }
    }
    
    pub fn offset(&self) -> u64 {
        ((self.page - 1) * self.page_size) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_calculation() {
        let pagination = Pagination::new(1, 10, 25);
        assert_eq!(pagination.total_pages, 3);
        assert_eq!(pagination.offset(), 0);
        
        let pagination = Pagination::new(2, 10, 25);
        assert_eq!(pagination.offset(), 10);
        
        let pagination = Pagination::new(3, 10, 25);
        assert_eq!(pagination.offset(), 20);
    }
    
    #[test]
    fn test_application_error() {
        let error = ApplicationError::new("INVALID_INPUT", "Invalid paper ID")
            .with_details("Paper ID must be in format: YYYY.NNNNN");
        
        assert_eq!(error.code, "INVALID_INPUT");
        assert_eq!(error.message, "Invalid paper ID");
        assert!(error.details.is_some());
        
        let display_str = format!("{}", error);
        assert!(display_str.contains("INVALID_INPUT"));
        assert!(display_str.contains("Invalid paper ID"));
    }
    
    #[test]
    fn test_search_request_default() {
        let request = PaperSearchRequest::default();
        assert!(request.query.is_none());
        assert!(request.page.is_none());
        assert!(request.page_size.is_none());
    }
}

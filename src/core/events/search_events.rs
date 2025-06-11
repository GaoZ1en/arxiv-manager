
use crate::core::{ArxivPaper, SearchQuery};
use crate::core::state::search_state::{AdvancedSearchParams, SortBy, SearchType};

/// 搜索相关事件
#[derive(Debug, Clone)]
pub enum SearchEvent {
    /// 搜索查询改变
    QueryChanged(String),
    
    /// 搜索提交
    SearchSubmitted(SearchQuery),
    
    /// 搜索开始
    SearchStarted {
        query: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    
    /// 搜索结果接收
    ResultsReceived {
        results: Vec<ArxivPaper>,
        query: String,
        result_count: usize,
        search_time_ms: u64,
    },
    
    /// 搜索失败
    SearchFailed {
        query: String,
        error: String,
        error_type: SearchErrorType,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    
    /// 搜索完成（无论成功失败）
    SearchCompleted {
        query: String,
        success: bool,
        duration_ms: u64,
    },
    
    /// 搜索取消
    SearchCancelled(String),
    
    /// 清除搜索结果
    ResultsCleared,
    
    /// 论文选择
    PaperSelected {
        paper: ArxivPaper,
        index: usize,
    },
    
    /// 取消论文选择
    PaperDeselected,
    
    /// 高级搜索参数改变
    AdvancedParamsChanged(AdvancedSearchParams),
    
    /// 排序方式改变
    SortByChanged(SortBy),
    
    /// 搜索类型改变
    SearchTypeChanged(SearchType),
    
    /// 搜索历史相关事件
    History(SearchHistoryEvent),
    
    /// 搜索过滤器事件
    Filter(SearchFilterEvent),
    
    /// 搜索导出事件
    Export(SearchExportEvent),
    
    /// 搜索统计事件
    Statistics(SearchStatisticsEvent),
}

/// 搜索错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum SearchErrorType {
    /// 网络错误
    NetworkError,
    
    /// API错误
    ApiError,
    
    /// 查询格式错误
    InvalidQuery,
    
    /// 超时
    Timeout,
    
    /// 服务不可用
    ServiceUnavailable,
    
    /// 限制超出（API配额等）
    RateLimitExceeded,
    
    /// 解析错误
    ParseError,
    
    /// 未知错误
    Unknown,
}

/// 搜索历史事件
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SearchHistoryEvent {
    /// 添加到历史
    Added(String),
    
    /// 从历史中移除
    Removed(String),
    
    /// 清除历史
    Cleared,
    
    /// 历史查询选择
    QuerySelected(String),
    
    /// 历史导出
    Exported(Vec<String>),
    
    /// 历史导入
    Imported(Vec<String>),
}

/// 搜索过滤器事件
#[derive(Debug, Clone)]
pub enum SearchFilterEvent {
    /// 应用日期过滤器
    DateFilterApplied {
        start_date: chrono::NaiveDate,
        end_date: chrono::NaiveDate,
    },
    
    /// 移除日期过滤器
    DateFilterRemoved,
    
    /// 应用分类过滤器
    CategoryFilterApplied(Vec<String>),
    
    /// 移除分类过滤器
    CategoryFilterRemoved,
    
    /// 应用作者过滤器
    AuthorFilterApplied(Vec<String>),
    
    /// 移除作者过滤器
    AuthorFilterRemoved,
    
    /// 重置所有过滤器
    AllFiltersReset,
    
    /// 过滤器组合应用
    CombinedFiltersApplied {
        categories: Option<Vec<String>>,
        authors: Option<Vec<String>>,
        date_range: Option<(chrono::NaiveDate, chrono::NaiveDate)>,
    },
}

/// 搜索导出事件
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SearchExportEvent {
    /// 导出搜索结果请求
    ExportRequested {
        format: ExportFormat,
        papers: Vec<ArxivPaper>,
        include_metadata: bool,
    },
    
    /// 导出开始
    ExportStarted {
        format: ExportFormat,
        paper_count: usize,
    },
    
    /// 导出进度
    ExportProgress {
        processed: usize,
        total: usize,
        current_paper: Option<String>, // arxiv_id
    },
    
    /// 导出完成
    ExportCompleted {
        file_path: std::path::PathBuf,
        format: ExportFormat,
        paper_count: usize,
        duration_ms: u64,
    },
    
    /// 导出失败
    ExportFailed {
        error: String,
        format: ExportFormat,
    },
    
    /// 导出取消
    ExportCancelled,
}

/// 导出格式
#[derive(Debug, Clone, PartialEq)]
pub enum ExportFormat {
    Json,
    Csv,
    Bibtex,
    Xml,
    Pdf, // 合并成单个PDF
    Html,
    Markdown,
}

/// 搜索统计事件
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SearchStatisticsEvent {
    /// 统计数据更新
    StatsUpdated {
        total_searches: u64,
        successful_searches: u64,
        failed_searches: u64,
        average_search_time_ms: u64,
        most_searched_terms: Vec<(String, u64)>, // (查询词, 次数)
    },
    
    /// 搜索性能记录
    PerformanceLogged {
        query: String,
        result_count: usize,
        search_time_ms: u64,
        api_response_time_ms: u64,
        parse_time_ms: u64,
    },
    
    /// 热门搜索词更新
    PopularTermsUpdated(Vec<(String, u64)>),
    
    /// 搜索趋势更新
    TrendsUpdated {
        daily_searches: Vec<(chrono::NaiveDate, u64)>,
        popular_categories: Vec<(String, u64)>,
        peak_hours: Vec<(u8, u64)>, // (小时, 搜索次数)
    },
    
    /// 统计重置
    StatsReset,
    
    /// 统计导出
    StatsExported(std::path::PathBuf),
}

/// 搜索事件处理器特征
pub trait SearchEventHandler {
    /// 处理搜索事件
    fn handle_search_event(&mut self, event: &SearchEvent) -> Result<(), SearchEventError>;
}

/// 搜索事件错误
#[derive(Debug, thiserror::Error)]
pub enum SearchEventError {
    #[error("Invalid search query: {0}")]
    InvalidQuery(String),
    
    #[error("Search operation failed: {0}")]
    SearchFailed(String),
    
    #[error("Export operation failed: {0}")]
    ExportFailed(String),
    
    #[error("History operation failed: {0}")]
    HistoryFailed(String),
    
    #[error("Filter operation failed: {0}")]
    FilterFailed(String),
    
    #[error("Statistics operation failed: {0}")]
    StatisticsFailed(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

/// 搜索事件构建器
pub struct SearchEventBuilder {
    #[allow(dead_code)]
    query: Option<String>,
    #[allow(dead_code)]
    timestamp: Option<chrono::DateTime<chrono::Utc>>,
    #[allow(dead_code)]
    error_type: Option<SearchErrorType>,
    #[allow(dead_code)]
    results: Option<Vec<ArxivPaper>>,
}

impl SearchEventBuilder {
    /// 创建新的搜索事件构建器
    pub fn new() -> Self {
        Self {
            query: None,
            timestamp: None,
            error_type: None,
            results: None,
        }
    }
    
    /// 设置查询
    pub fn with_query(mut self, query: String) -> Self {
        self.query = Some(query);
        self
    }
    
    /// 设置时间戳
    pub fn with_timestamp(mut self, timestamp: chrono::DateTime<chrono::Utc>) -> Self {
        self.timestamp = Some(timestamp);
        self
    }
    
    /// 设置错误类型
    pub fn with_error_type(mut self, error_type: SearchErrorType) -> Self {
        self.error_type = Some(error_type);
        self
    }
    
    /// 设置结果
    pub fn with_results(mut self, results: Vec<ArxivPaper>) -> Self {
        self.results = Some(results);
        self
    }
    
    /// 构建搜索开始事件
    pub fn build_search_started(self) -> Option<SearchEvent> {
        if let (Some(query), Some(timestamp)) = (self.query, self.timestamp) {
            Some(SearchEvent::SearchStarted { query, timestamp })
        } else {
            None
        }
    }
    
    /// 构建搜索失败事件
    pub fn build_search_failed(self, error: String) -> Option<SearchEvent> {
        if let (Some(query), Some(error_type), Some(timestamp)) = 
            (self.query, self.error_type, self.timestamp) {
            Some(SearchEvent::SearchFailed {
                query,
                error,
                error_type,
                timestamp,
            })
        } else {
            None
        }
    }
    
    /// 构建搜索结果接收事件
    pub fn build_results_received(self, search_time_ms: u64) -> Option<SearchEvent> {
        if let (Some(query), Some(results)) = (self.query, self.results) {
            let result_count = results.len();
            Some(SearchEvent::ResultsReceived {
                results,
                query,
                result_count,
                search_time_ms,
            })
        } else {
            None
        }
    }
}

impl Default for SearchEventBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 搜索事件聚合器
pub struct SearchEventAggregator {
    #[allow(dead_code)]
    events: Vec<SearchEvent>,
    #[allow(dead_code)]
    start_time: Option<chrono::DateTime<chrono::Utc>>,
    #[allow(dead_code)]
    query: Option<String>,
}

impl SearchEventAggregator {
    /// 创建新的事件聚合器
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            start_time: None,
            query: None,
        }
    }
    
    /// 开始聚合搜索会话
    pub fn start_session(&mut self, query: String) {
        self.query = Some(query);
        self.start_time = Some(chrono::Utc::now());
        self.events.clear();
    }
    
    /// 添加事件到聚合器
    pub fn add_event(&mut self, event: SearchEvent) {
        self.events.push(event);
    }
    
    /// 结束聚合会话并返回摘要
    pub fn end_session(&mut self) -> Option<SearchSessionSummary> {
        if let (Some(query), Some(start_time)) = (self.query.take(), self.start_time.take()) {
            let duration = chrono::Utc::now().signed_duration_since(start_time);
            let events = std::mem::take(&mut self.events);
            let success = events.iter().any(|e| matches!(e, SearchEvent::ResultsReceived { .. }));
            
            Some(SearchSessionSummary {
                query,
                start_time,
                duration_ms: duration.num_milliseconds() as u64,
                events,
                success,
            })
        } else {
            None
        }
    }
}

impl Default for SearchEventAggregator {
    fn default() -> Self {
        Self::new()
    }
}

/// 搜索会话摘要
#[derive(Debug, Clone)]
pub struct SearchSessionSummary {
    pub query: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub duration_ms: u64,
    pub events: Vec<SearchEvent>,
    pub success: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_search_event_builder() {
        let builder = SearchEventBuilder::new()
            .with_query("quantum computing".to_string())
            .with_timestamp(chrono::Utc::now());
        
        let event = builder.build_search_started();
        assert!(event.is_some());
        
        if let Some(SearchEvent::SearchStarted { query, .. }) = event {
            assert_eq!(query, "quantum computing");
        }
    }
    
    #[test]
    fn test_search_event_aggregator() {
        let mut aggregator = SearchEventAggregator::new();
        aggregator.start_session("test query".to_string());
        
        aggregator.add_event(SearchEvent::SearchStarted {
            query: "test query".to_string(),
            timestamp: chrono::Utc::now(),
        });
        
        let summary = aggregator.end_session();
        assert!(summary.is_some());
        
        if let Some(summary) = summary {
            assert_eq!(summary.query, "test query");
            assert_eq!(summary.events.len(), 1);
        }
    }
    
    #[test]
    fn test_export_format() {
        let format = ExportFormat::Json;
        assert_eq!(format, ExportFormat::Json);
        assert_ne!(format, ExportFormat::Csv);
    }
    
    #[test]
    fn test_search_error_type() {
        let error = SearchErrorType::NetworkError;
        assert_eq!(error, SearchErrorType::NetworkError);
        assert_ne!(error, SearchErrorType::ApiError);
    }
}

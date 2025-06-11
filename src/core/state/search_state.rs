
use crate::core::{ArxivPaper, SearchQuery};

/// 搜索类型
#[derive(Debug, Clone, PartialEq)]
pub enum SearchType {
    All,
    Title,
    Author,
    Abstract,
    Category,
}

/// 排序方式（本地定义用于状态管理）
#[derive(Debug, Clone, PartialEq)]
pub enum SortBy {
    Relevance,
    Date,
    Title,
    Author,
}

/// 日期范围
#[derive(Debug, Clone)]
pub struct DateRange {
    pub start: chrono::NaiveDate,
    pub end: chrono::NaiveDate,
}

/// 搜索相关的状态管理
#[derive(Debug, Clone)]
pub struct SearchState {
    /// 当前搜索查询
    pub query: String,
    
    /// 搜索结果
    pub results: Vec<ArxivPaper>,
    
    /// 是否正在搜索
    pub is_searching: bool,
    
    /// 搜索历史记录
    pub search_history: Vec<String>,
    
    /// 最大历史记录数量
    pub max_history: usize,
    
    /// 当前选中的论文索引
    pub selected_index: Option<usize>,
    
    /// 搜索错误信息
    pub error_message: Option<String>,
    
    /// 高级搜索参数
    pub advanced_params: AdvancedSearchParams,
    
    /// 搜索统计信息
    pub stats: SearchStats,
}

/// 高级搜索参数
#[derive(Debug, Clone)]
pub struct AdvancedSearchParams {
    /// 最大结果数量
    pub max_results: usize,
    
    /// 排序方式
    pub sort_by: SortBy,
    
    /// 搜索类型
    pub search_type: SearchType,
    
    /// 日期范围过滤
    pub date_range: Option<DateRange>,
    
    /// 分类过滤
    pub categories: Vec<String>,
    
    /// 作者过滤
    pub authors: Vec<String>,
}

/// 搜索统计信息
#[derive(Debug, Clone, Default)]
pub struct SearchStats {
    /// 总搜索次数
    pub total_searches: u64,
    
    /// 成功搜索次数
    pub successful_searches: u64,
    
    /// 失败搜索次数
    pub failed_searches: u64,
    
    /// 平均搜索时间（毫秒）
    pub average_search_time_ms: u64,
    
    /// 最近搜索时间记录
    pub recent_search_times: Vec<u64>,
}

impl Default for SearchState {
    fn default() -> Self {
        Self {
            query: String::new(),
            results: Vec::new(),
            is_searching: false,
            search_history: Vec::new(),
            max_history: 50,
            selected_index: None,
            error_message: None,
            advanced_params: AdvancedSearchParams::default(),
            stats: SearchStats::default(),
        }
    }
}

impl Default for AdvancedSearchParams {
    fn default() -> Self {
        Self {
            max_results: 20,
            sort_by: SortBy::Relevance,
            search_type: SearchType::All,
            date_range: None,
            categories: Vec::new(),
            authors: Vec::new(),
        }
    }
}

impl SearchState {
    /// 创建新的搜索状态
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 设置搜索查询
    pub fn set_query(&mut self, query: String) {
        self.query = query;
        self.clear_error();
    }
    
    /// 开始搜索
    pub fn start_search(&mut self) {
        self.is_searching = true;
        self.error_message = None;
        self.results.clear();
        
        // 添加到搜索历史
        if !self.query.trim().is_empty() && !self.search_history.contains(&self.query) {
            self.add_to_history(self.query.clone());
        }
        
        self.stats.total_searches += 1;
    }
    
    /// 完成搜索
    pub fn complete_search(&mut self, results: Vec<ArxivPaper>, search_time_ms: u64) {
        self.is_searching = false;
        self.results = results;
        self.selected_index = None;
        self.stats.successful_searches += 1;
        self.update_search_time(search_time_ms);
    }
    
    /// 搜索失败
    pub fn fail_search(&mut self, error: String) {
        self.is_searching = false;
        self.error_message = Some(error);
        self.results.clear();
        self.selected_index = None;
        self.stats.failed_searches += 1;
    }
    
    /// 清除错误信息
    pub fn clear_error(&mut self) {
        self.error_message = None;
    }
    
    /// 选择论文
    pub fn select_paper(&mut self, index: usize) {
        if index < self.results.len() {
            self.selected_index = Some(index);
        }
    }
    
    /// 取消选择
    pub fn clear_selection(&mut self) {
        self.selected_index = None;
    }
    
    /// 获取当前选中的论文
    pub fn get_selected_paper(&self) -> Option<&ArxivPaper> {
        self.selected_index
            .and_then(|index| self.results.get(index))
    }
    
    /// 添加到搜索历史
    pub fn add_to_history(&mut self, query: String) {
        // 移除重复项
        self.search_history.retain(|q| q != &query);
        
        // 添加到开头
        self.search_history.insert(0, query);
        
        // 限制历史记录数量
        if self.search_history.len() > self.max_history {
            self.search_history.truncate(self.max_history);
        }
    }
    
    /// 清除搜索历史
    pub fn clear_history(&mut self) {
        self.search_history.clear();
    }
    
    /// 从历史记录中移除条目
    pub fn remove_from_history(&mut self, index: usize) {
        if index < self.search_history.len() {
            self.search_history.remove(index);
        }
    }
    
    /// 设置高级搜索参数
    pub fn set_advanced_params(&mut self, params: AdvancedSearchParams) {
        self.advanced_params = params;
    }
    
    /// 重置搜索状态
    pub fn reset(&mut self) {
        self.query.clear();
        self.results.clear();
        self.is_searching = false;
        self.selected_index = None;
        self.error_message = None;
    }
    
    /// 创建搜索查询对象
    pub fn create_search_query(&self) -> SearchQuery {
        use crate::core::types::{SortBy as TypesSortBy, SortOrder};
        
        let sort_by = match self.advanced_params.sort_by {
            SortBy::Relevance => TypesSortBy::Relevance,
            SortBy::Date => TypesSortBy::SubmittedDate,
            SortBy::Title => TypesSortBy::Relevance, // 回退到相关度排序
            SortBy::Author => TypesSortBy::Relevance, // 回退到相关度排序
        };
        
        SearchQuery {
            query: self.query.clone(),
            max_results: self.advanced_params.max_results,
            start: 0,
            sort_by,
            sort_order: SortOrder::Descending,
        }
    }
    
    /// 更新搜索时间统计
    fn update_search_time(&mut self, search_time_ms: u64) {
        self.stats.recent_search_times.push(search_time_ms);
        
        // 只保留最近20次搜索时间
        if self.stats.recent_search_times.len() > 20 {
            self.stats.recent_search_times.remove(0);
        }
        
        // 计算平均搜索时间
        if !self.stats.recent_search_times.is_empty() {
            let total: u64 = self.stats.recent_search_times.iter().sum();
            self.stats.average_search_time_ms = total / self.stats.recent_search_times.len() as u64;
        }
    }
    
    /// 获取搜索成功率
    pub fn get_success_rate(&self) -> f64 {
        if self.stats.total_searches == 0 {
            0.0
        } else {
            self.stats.successful_searches as f64 / self.stats.total_searches as f64
        }
    }
    
    /// 检查是否有搜索结果
    pub fn has_results(&self) -> bool {
        !self.results.is_empty()
    }
    
    /// 获取结果数量
    pub fn result_count(&self) -> usize {
        self.results.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_search_state_creation() {
        let state = SearchState::new();
        assert_eq!(state.query, "");
        assert_eq!(state.results.len(), 0);
        assert!(!state.is_searching);
        assert!(state.selected_index.is_none());
    }
    
    #[test]
    fn test_search_history() {
        let mut state = SearchState::new();
        
        state.add_to_history("quantum computing".to_string());
        state.add_to_history("machine learning".to_string());
        
        assert_eq!(state.search_history.len(), 2);
        assert_eq!(state.search_history[0], "machine learning");
        assert_eq!(state.search_history[1], "quantum computing");
        
        // 测试重复条目
        state.add_to_history("quantum computing".to_string());
        assert_eq!(state.search_history.len(), 2);
        assert_eq!(state.search_history[0], "quantum computing");
    }
    
    #[test]
    fn test_paper_selection() {
        let mut state = SearchState::new();
        
        // 没有结果时不应该能选择
        state.select_paper(0);
        assert!(state.selected_index.is_none());
        
        // 模拟有结果的情况
        state.results = vec![
            ArxivPaper {
                id: "1".to_string(),
                title: "Test Paper".to_string(),
                ..Default::default()
            }
        ];
        
        state.select_paper(0);
        assert_eq!(state.selected_index, Some(0));
        
        state.clear_selection();
        assert!(state.selected_index.is_none());
    }
    
    #[test]
    fn test_search_stats() {
        let mut state = SearchState::new();
        
        state.start_search();
        state.complete_search(vec![], 100);
        
        assert_eq!(state.stats.total_searches, 1);
        assert_eq!(state.stats.successful_searches, 1);
        assert_eq!(state.stats.failed_searches, 0);
        assert_eq!(state.stats.average_search_time_ms, 100);
        
        state.start_search();
        state.fail_search("Network error".to_string());
        
        assert_eq!(state.stats.total_searches, 2);
        assert_eq!(state.stats.successful_searches, 1);
        assert_eq!(state.stats.failed_searches, 1);
    }
    
    #[test]
    fn test_success_rate() {
        let mut state = SearchState::new();
        assert_eq!(state.get_success_rate(), 0.0);
        
        state.start_search();
        state.complete_search(vec![], 100);
        assert_eq!(state.get_success_rate(), 1.0);
        
        state.start_search();
        state.fail_search("Error".to_string());
        assert_eq!(state.get_success_rate(), 0.5);
    }
}

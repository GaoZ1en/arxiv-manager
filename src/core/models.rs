use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// arXiv 查询参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArxivQuery {
    pub search_query: Option<String>,
    pub id_list: Option<Vec<String>>,
    pub start: usize,
    pub max_results: usize,
    pub sort_by: SortBy,
    pub sort_order: SortOrder,
}

/// 排序方式
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SortBy {
    Relevance,
    LastUpdatedDate,
    SubmittedDate,
}

/// 排序顺序
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}

/// arXiv 搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArxivSearchResult {
    pub total_results: usize,
    pub start_index: usize,
    pub items_per_page: usize,
    pub entries: Vec<ArxivPaper>,
}

/// arXiv 论文
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArxivPaper {
    pub id: String,
    pub title: String,
    pub authors: Vec<ArxivAuthor>,
    pub summary: String,
    pub categories: Vec<ArxivCategory>,
    pub primary_category: ArxivCategory,
    pub published: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub doi: Option<String>,
    pub journal_ref: Option<String>,
    pub pdf_url: String,
    pub abs_url: String,
    pub comment: Option<String>,
}

/// arXiv 作者
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArxivAuthor {
    pub name: String,
    pub affiliation: Option<String>,
}

/// arXiv 类别
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArxivCategory {
    pub term: String,
    pub scheme: String,
    pub label: Option<String>,
}

/// 高级搜索参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSearchParams {
    pub title: Option<String>,
    pub author: Option<String>,
    pub abstract_text: Option<String>,
    pub category: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub date_range: Option<DateRange>,
    pub start: Option<usize>,
    pub max_results: Option<usize>,
    pub sort_by: Option<SortBy>,
    pub sort_order: Option<SortOrder>,
}

/// 日期范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DateRange {
    LastWeek,
    LastMonth,
    LastYear,
    Custom {
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    },
}

impl ArxivPaper {
    /// 提取 arXiv ID（去除版本号）
    pub fn get_arxiv_id(&self) -> String {
        let id = self.id.trim_start_matches("http://arxiv.org/abs/");
        // 移除版本号 (例如 1234.5678v2 -> 1234.5678)
        if let Some(pos) = id.find('v') {
            id[..pos].to_string()
        } else {
            id.to_string()
        }
    }

    /// 获取格式化的作者列表
    pub fn get_author_names(&self) -> Vec<String> {
        self.authors.iter().map(|a| a.name.clone()).collect()
    }

    /// 获取主要类别名称
    pub fn get_primary_category(&self) -> String {
        self.primary_category.term.clone()
    }

    /// 获取所有类别名称
    pub fn get_category_names(&self) -> Vec<String> {
        self.categories.iter().map(|c| c.term.clone()).collect()
    }

    /// 转换为数据库论文模型
    pub fn to_database_paper(&self) -> crate::database::Paper {
        crate::database::Paper::new(
            self.get_arxiv_id(),
            self.title.clone(),
            self.get_author_names(),
            self.summary.clone(),
            self.get_category_names(),
            self.get_primary_category(),
            self.published,
        )
    }
}

impl ArxivAuthor {
    /// 创建新的作者
    pub fn new(name: String) -> Self {
        Self {
            name,
            affiliation: None,
        }
    }

    /// 带机构信息的作者
    pub fn with_affiliation(name: String, affiliation: String) -> Self {
        Self {
            name,
            affiliation: Some(affiliation),
        }
    }
}

impl ArxivCategory {
    /// 创建新的类别
    pub fn new(term: String) -> Self {
        Self {
            term,
            scheme: "http://arxiv.org/schemas/atom".to_string(),
            label: None,
        }
    }

    /// 获取类别的显示名称
    pub fn display_name(&self) -> String {
        self.label.clone().unwrap_or_else(|| self.term.clone())
    }
}

impl Default for ArxivQuery {
    fn default() -> Self {
        Self {
            search_query: None,
            id_list: None,
            start: 0,
            max_results: 10,
            sort_by: SortBy::Relevance,
            sort_order: SortOrder::Descending,
        }
    }
}

impl Default for AdvancedSearchParams {
    fn default() -> Self {
        Self {
            title: None,
            author: None,
            abstract_text: None,
            category: None,
            keywords: None,
            date_range: None,
            start: Some(0),
            max_results: Some(50),
            sort_by: Some(SortBy::Relevance),
            sort_order: Some(SortOrder::Descending),
        }
    }
}

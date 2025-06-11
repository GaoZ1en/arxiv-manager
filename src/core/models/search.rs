// 搜索相关的数据模型

#[derive(Debug, Clone)]
pub struct SearchConfig {
    pub query: String,
    pub search_in: SearchField,
    pub categories: Vec<String>,
    pub date_range: DateRange,
    pub sort_by: SortBy,
    pub sort_order: SortOrder,
    pub max_results: u32,
    pub authors: Vec<String>,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            query: String::new(),
            search_in: SearchField::All,
            categories: Vec::new(),
            date_range: DateRange::Any,
            sort_by: SortBy::Relevance,
            sort_order: SortOrder::Descending,
            max_results: 20,
            authors: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchField {
    All,
    Title,
    Abstract,
    Authors,
    Comments,
}

impl std::fmt::Display for SearchField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl SearchField {
    pub fn as_str(&self) -> &'static str {
        match self {
            SearchField::All => "all",
            SearchField::Title => "ti",
            SearchField::Abstract => "abs",
            SearchField::Authors => "au",
            SearchField::Comments => "co",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            SearchField::All => "All Fields",
            SearchField::Title => "Title",
            SearchField::Abstract => "Abstract",
            SearchField::Authors => "Authors",
            SearchField::Comments => "Comments",
        }
    }

    pub fn all_variants() -> Vec<Self> {
        vec![
            SearchField::All,
            SearchField::Title,
            SearchField::Abstract,
            SearchField::Authors,
            SearchField::Comments,
        ]
    }
}

// 日期范围
#[derive(Debug, Clone, PartialEq)]
pub enum DateRange {
    Any,
    LastWeek,
    LastMonth,
    LastYear,
    Custom { from: String, to: String },
}

impl std::fmt::Display for DateRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl DateRange {
    pub fn display_name(&self) -> String {
        match self {
            DateRange::Any => "Any Date".to_string(),
            DateRange::LastWeek => "Last Week".to_string(),
            DateRange::LastMonth => "Last Month".to_string(),
            DateRange::LastYear => "Last Year".to_string(),
            DateRange::Custom { from, to } => format!("{} to {}", from, to),
        }
    }
}

// 排序方式
#[derive(Debug, Clone, PartialEq)]
pub enum SortBy {
    Relevance,
    SubmissionDate,
    LastUpdated,
}

impl std::fmt::Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortBy::Relevance => write!(f, "Relevance"),
            SortBy::SubmissionDate => write!(f, "Submission Date"),
            SortBy::LastUpdated => write!(f, "Last Updated"),
        }
    }
}

impl SortBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortBy::Relevance => "relevance",
            SortBy::SubmissionDate => "submittedDate",
            SortBy::LastUpdated => "lastUpdatedDate",
        }
    }

    pub fn all_variants() -> Vec<Self> {
        vec![
            SortBy::Relevance,
            SortBy::SubmissionDate,
            SortBy::LastUpdated,
        ]
    }
}

// 排序顺序
#[derive(Debug, Clone, PartialEq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOrder::Ascending => write!(f, "Ascending"),
            SortOrder::Descending => write!(f, "Descending"),
        }
    }
}

impl SortOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortOrder::Ascending => "ascending",
            SortOrder::Descending => "descending",
        }
    }

    pub fn all_variants() -> Vec<Self> {
        vec![
            SortOrder::Ascending,
            SortOrder::Descending,
        ]
    }
}

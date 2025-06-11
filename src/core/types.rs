use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArxivPaper {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub abstract_text: String,
    pub categories: Vec<String>,
    pub published: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub pdf_url: String,
    pub abstract_url: String,
    pub doi: Option<String>,
    pub journal_ref: Option<String>,
    pub comments: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SearchQuery {
    #[allow(dead_code)]
    pub query: String,
    #[allow(dead_code)]
    pub max_results: usize,
    #[allow(dead_code)]
    pub start: usize,
    #[allow(dead_code)]
    pub sort_by: SortBy,
    #[allow(dead_code)]
    pub sort_order: SortOrder,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SortBy {
    Relevance,
    LastUpdatedDate,
    SubmittedDate,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            query: String::new(),
            max_results: 10,
            start: 0,
            sort_by: SortBy::Relevance,
            sort_order: SortOrder::Descending,
        }
    }
}

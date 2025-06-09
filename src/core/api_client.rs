use anyhow::{anyhow, Result};
use reqwest::Client;
use url::Url;

use super::models::*;
use super::parser::ArxivParser;

/// arXiv API 客户端
pub struct ArxivClient {
    client: Client,
    base_url: String,
}

impl ArxivClient {
    /// 创建新的 arXiv 客户端
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "http://export.arxiv.org/api/query".to_string(),
        }
    }

    /// 根据 arXiv ID 查询论文
    pub async fn get_paper_by_id(&self, arxiv_id: &str) -> Result<Option<ArxivPaper>> {
        let query = ArxivQuery {
            search_query: Some(format!("id:{}", arxiv_id)),
            id_list: None,
            start: 0,
            max_results: 1,
            sort_by: SortBy::Relevance,
            sort_order: SortOrder::Descending,
        };

        let mut results = self.search(&query).await?;
        Ok(results.entries.pop())
    }

    /// 根据 ID 列表批量查询论文
    pub async fn get_papers_by_ids(&self, arxiv_ids: &[String]) -> Result<Vec<ArxivPaper>> {
        let query = ArxivQuery {
            search_query: None,
            id_list: Some(arxiv_ids.to_vec()),
            start: 0,
            max_results: arxiv_ids.len(),
            sort_by: SortBy::SubmittedDate,
            sort_order: SortOrder::Descending,
        };

        let results = self.search(&query).await?;
        Ok(results.entries)
    }

    /// 搜索论文
    pub async fn search(&self, query: &ArxivQuery) -> Result<ArxivSearchResult> {
        let url = self.build_query_url(query)?;
        
        let response = self.client
            .get(&url)
            .header("User-Agent", "ArxivManager/1.0")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("API request failed: {}", response.status()));
        }

        let xml_content = response.text().await?;
        ArxivParser::parse_search_results(&xml_content)
    }

    /// 按关键词搜索
    pub async fn search_by_keywords(
        &self,
        keywords: &[String],
        max_results: usize,
    ) -> Result<ArxivSearchResult> {
        let search_query = keywords.join(" AND ");
        let query = ArxivQuery {
            search_query: Some(search_query),
            id_list: None,
            start: 0,
            max_results,
            sort_by: SortBy::Relevance,
            sort_order: SortOrder::Descending,
        };

        self.search(&query).await
    }

    /// 按作者搜索
    pub async fn search_by_author(
        &self,
        author: &str,
        max_results: usize,
    ) -> Result<ArxivSearchResult> {
        let search_query = format!("au:{}", author);
        let query = ArxivQuery {
            search_query: Some(search_query),
            id_list: None,
            start: 0,
            max_results,
            sort_by: SortBy::SubmittedDate,
            sort_order: SortOrder::Descending,
        };

        self.search(&query).await
    }

    /// 按类别搜索
    pub async fn search_by_category(
        &self,
        category: &str,
        max_results: usize,
    ) -> Result<ArxivSearchResult> {
        let search_query = format!("cat:{}", category);
        let query = ArxivQuery {
            search_query: Some(search_query),
            id_list: None,
            start: 0,
            max_results,
            sort_by: SortBy::SubmittedDate,
            sort_order: SortOrder::Descending,
        };

        self.search(&query).await
    }

    /// 按标题搜索
    pub async fn search_by_title(
        &self,
        title: &str,
        max_results: usize,
    ) -> Result<ArxivSearchResult> {
        let search_query = format!("ti:{}", title);
        let query = ArxivQuery {
            search_query: Some(search_query),
            id_list: None,
            start: 0,
            max_results,
            sort_by: SortBy::Relevance,
            sort_order: SortOrder::Descending,
        };

        self.search(&query).await
    }

    /// 高级搜索
    pub async fn advanced_search(&self, params: &AdvancedSearchParams) -> Result<ArxivSearchResult> {
        let mut query_parts = Vec::new();

        if let Some(title) = &params.title {
            query_parts.push(format!("ti:{}", title));
        }

        if let Some(author) = &params.author {
            query_parts.push(format!("au:{}", author));
        }

        if let Some(abstract_text) = &params.abstract_text {
            query_parts.push(format!("abs:{}", abstract_text));
        }

        if let Some(category) = &params.category {
            query_parts.push(format!("cat:{}", category));
        }

        if let Some(keywords) = &params.keywords {
            query_parts.push(format!("all:{}", keywords.join(" ")));
        }

        if let Some(date_range) = &params.date_range {
            let date_query = match date_range {
                DateRange::LastWeek => {
                    let week_ago = chrono::Utc::now() - chrono::Duration::days(7);
                    format!("submittedDate:[{}0000 TO *]", week_ago.format("%Y%m%d"))
                }
                DateRange::LastMonth => {
                    let month_ago = chrono::Utc::now() - chrono::Duration::days(30);
                    format!("submittedDate:[{}0000 TO *]", month_ago.format("%Y%m%d"))
                }
                DateRange::LastYear => {
                    let year_ago = chrono::Utc::now() - chrono::Duration::days(365);
                    format!("submittedDate:[{}0000 TO *]", year_ago.format("%Y%m%d"))
                }
                DateRange::Custom { start, end } => {
                    format!(
                        "submittedDate:[{}0000 TO {}2359]",
                        start.format("%Y%m%d"),
                        end.format("%Y%m%d")
                    )
                }
            };
            query_parts.push(date_query);
        }

        let search_query = if query_parts.is_empty() {
            "all:*".to_string()
        } else {
            query_parts.join(" AND ")
        };

        let query = ArxivQuery {
            search_query: Some(search_query),
            id_list: None,
            start: params.start.unwrap_or(0),
            max_results: params.max_results.unwrap_or(100),
            sort_by: params.sort_by.unwrap_or(SortBy::Relevance),
            sort_order: params.sort_order.unwrap_or(SortOrder::Descending),
        };

        self.search(&query).await
    }

    /// 构建查询 URL
    fn build_query_url(&self, query: &ArxivQuery) -> Result<String> {
        let mut url = Url::parse(&self.base_url)?;

        {
            let mut query_pairs = url.query_pairs_mut();

            if let Some(search_query) = &query.search_query {
                query_pairs.append_pair("search_query", search_query);
            }

            if let Some(id_list) = &query.id_list {
                query_pairs.append_pair("id_list", &id_list.join(","));
            }

            query_pairs.append_pair("start", &query.start.to_string());
            query_pairs.append_pair("max_results", &query.max_results.to_string());

            let sort_by_str = match query.sort_by {
                SortBy::Relevance => "relevance",
                SortBy::LastUpdatedDate => "lastUpdatedDate",
                SortBy::SubmittedDate => "submittedDate",
            };
            query_pairs.append_pair("sortBy", sort_by_str);

            let sort_order_str = match query.sort_order {
                SortOrder::Ascending => "ascending",
                SortOrder::Descending => "descending",
            };
            query_pairs.append_pair("sortOrder", sort_order_str);
        }

        Ok(url.to_string())
    }

    /// 获取论文的 PDF 下载链接
    pub fn get_pdf_url(&self, arxiv_id: &str) -> String {
        format!("https://arxiv.org/pdf/{}.pdf", arxiv_id)
    }

    /// 获取论文的摘要页面链接
    pub fn get_abs_url(&self, arxiv_id: &str) -> String {
        format!("https://arxiv.org/abs/{}", arxiv_id)
    }
}

impl Default for ArxivClient {
    fn default() -> Self {
        Self::new()
    }
}

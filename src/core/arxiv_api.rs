use crate::core::types::{ArxivPaper, SearchQuery, SortBy, SortOrder};
use crate::utils::{ArxivError, Result};
use reqwest::Client;
use chrono::DateTime;

#[derive(Debug)]
pub struct ArxivClient {
    client: Client,
    base_url: String,
}

impl ArxivClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "http://export.arxiv.org/api/query".to_string(),
        }
    }
    
    pub async fn search(&self, query: &SearchQuery) -> Result<Vec<ArxivPaper>> {
        let url = self.build_search_url(query);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(ArxivError::ArxivApi(format!(
                "API request failed with status: {}", 
                response.status()
            )));
        }
        
        let xml_content = response.text().await?;
        self.parse_search_response(&xml_content)
    }
    
    pub async fn get_paper_by_id(&self, arxiv_id: &str) -> Result<Option<ArxivPaper>> {
        let query = SearchQuery {
            query: format!("id:{}", arxiv_id),
            max_results: 1,
            start: 0,
            sort_by: SortBy::Relevance,
            sort_order: SortOrder::Descending,
        };
        
        let mut papers = self.search(&query).await?;
        Ok(papers.pop())
    }
    
    fn build_search_url(&self, query: &SearchQuery) -> String {
        let mut params = vec![
            ("search_query", query.query.clone()),
            ("start", query.start.to_string()),
            ("max_results", query.max_results.to_string()),
        ];
        
        let sort_by = match query.sort_by {
            SortBy::Relevance => "relevance",
            SortBy::LastUpdatedDate => "lastUpdatedDate",
            SortBy::SubmittedDate => "submittedDate",
        };
        
        let sort_order = match query.sort_order {
            SortOrder::Ascending => "ascending",
            SortOrder::Descending => "descending",
        };
        
        params.push(("sortBy", sort_by.to_string()));
        params.push(("sortOrder", sort_order.to_string()));
        
        let query_string = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");
        
        format!("{}?{}", self.base_url, query_string)
    }
    
    fn parse_search_response(&self, xml_content: &str) -> Result<Vec<ArxivPaper>> {
        let doc = roxmltree::Document::parse(xml_content)
            .map_err(|e| ArxivError::Xml(e.to_string()))?;
        
        let mut papers = Vec::new();
        
        for entry in doc.descendants().filter(|n| n.has_tag_name("entry")) {
            if let Some(paper) = self.parse_entry_node(&entry)? {
                papers.push(paper);
            }
        }
        
        Ok(papers)
    }
    
    fn parse_entry_node(&self, entry: &roxmltree::Node) -> Result<Option<ArxivPaper>> {
        let id = self.extract_text(entry, "id")?;
        let title = self.extract_text(entry, "title")?;
        let abstract_text = self.extract_text(entry, "summary")?;
        let published = self.extract_date(entry, "published")?;
        let updated = self.extract_date(entry, "updated")?;
        
        // Extract authors
        let mut authors = Vec::new();
        for author in entry.descendants().filter(|n| n.has_tag_name("author")) {
            if let Some(name) = author.descendants().find(|n| n.has_tag_name("name")) {
                if let Some(text) = name.text() {
                    authors.push(text.trim().to_string());
                }
            }
        }
        
        // Extract categories
        let mut categories = Vec::new();
        for category in entry.descendants().filter(|n| n.has_tag_name("category")) {
            if let Some(term) = category.attribute("term") {
                categories.push(term.to_string());
            }
        }
        
        // Extract PDF URL
        let pdf_url = entry
            .descendants()
            .filter(|n| n.has_tag_name("link"))
            .find(|n| n.attribute("type") == Some("application/pdf"))
            .and_then(|n| n.attribute("href"))
            .unwrap_or("")
            .to_string();
        
        // Extract abstract URL
        let abstract_url = entry
            .descendants()
            .filter(|n| n.has_tag_name("link"))
            .find(|n| n.attribute("type") == Some("text/html"))
            .and_then(|n| n.attribute("href"))
            .unwrap_or("")
            .to_string();
        
        // Extract optional fields
        let doi = self.extract_text_optional(entry, "arxiv:doi");
        let journal_ref = self.extract_text_optional(entry, "arxiv:journal_ref");
        let comments = self.extract_text_optional(entry, "arxiv:comment");
        
        if id.is_empty() || title.is_empty() {
            return Ok(None);
        }
        
        Ok(Some(ArxivPaper {
            id: self.clean_arxiv_id(&id),
            title: title.trim().to_string(),
            authors,
            abstract_text: abstract_text.trim().to_string(),
            categories,
            published,
            updated,
            pdf_url,
            abstract_url,
            doi,
            journal_ref,
            comments,
        }))
    }
    
    fn extract_text(&self, entry: &roxmltree::Node, tag: &str) -> Result<String> {
        entry
            .descendants()
            .find(|n| n.has_tag_name(tag))
            .and_then(|n| n.text())
            .map(|s| s.to_string())
            .ok_or_else(|| ArxivError::Xml(format!("Missing required field: {}", tag)))
    }
    
    fn extract_text_optional(&self, entry: &roxmltree::Node, tag: &str) -> Option<String> {
        entry
            .descendants()
            .find(|n| n.has_tag_name(tag))
            .and_then(|n| n.text())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    }
    
    fn extract_date(&self, entry: &roxmltree::Node, tag: &str) -> Result<DateTime<chrono::Utc>> {
        let date_str = self.extract_text(entry, tag)?;
        DateTime::parse_from_rfc3339(&date_str)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .map_err(|e| ArxivError::Xml(format!("Invalid date format for {}: {}", tag, e)))
    }
    
    fn clean_arxiv_id(&self, id: &str) -> String {
        // Remove the arXiv URL prefix if present
        id.replace("http://arxiv.org/abs/", "")
    }
}

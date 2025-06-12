// ArXiv API客户端实现

use crate::core::models::{ArxivPaper, SearchConfig};
use super::query_builder::build_search_url;
use crate::search::parsers::xml_parser::parse_arxiv_xml;

/// 基础搜索功能 - 简单查询接口
#[allow(dead_code)]
pub async fn search_arxiv_papers(query: String) -> Result<Vec<ArxivPaper>, String> {
    if query.trim().is_empty() {
        return Ok(vec![]);
    }

    // 构建arXiv API查询URL
    let encoded_query = urlencoding::encode(&query);
    let url = format!(
        "https://export.arxiv.org/api/query?search_query=all:{}&start=0&max_results=10&sortBy=submittedDate&sortOrder=descending",
        encoded_query
    );

    execute_search_request(&url).await
}

/// 高级搜索功能 - 支持完整配置
pub async fn search_arxiv_papers_advanced(config: SearchConfig) -> Result<Vec<ArxivPaper>, String> {
    // 检查是否有任何搜索条件
    let has_search_criteria = !config.query.trim().is_empty() 
        || !config.authors.is_empty()
        || !config.categories.is_empty()
        || config.exact_phrase.as_ref().map_or(false, |p| !p.trim().is_empty())
        || !config.exclude_words.is_empty()
        || config.journal_ref.as_ref().map_or(false, |j| !j.trim().is_empty())
        || config.subject_class.as_ref().map_or(false, |s| !s.trim().is_empty())
        || config.report_number.as_ref().map_or(false, |r| !r.trim().is_empty())
        || !config.id_list.is_empty();
    
    if !has_search_criteria {
        return Ok(vec![]);
    }

    let url = build_search_url(&config)?;
    let mut papers = execute_search_request(&url).await?;
    
    // 应用日期过滤
    papers = crate::search::filters::date_filter::filter_papers_by_date(papers, &config.date_range);
    
    Ok(papers)
}

/// 执行HTTP请求并解析结果的内部函数
async fn execute_search_request(url: &str) -> Result<Vec<ArxivPaper>, String> {
    // 创建具有超时和重试的HTTP客户端
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30)) // 30秒超时
        .user_agent("ArxivManager/1.0 (https://github.com/user/arxiv-manager)")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    // 执行请求，最多重试3次
    let mut last_error = None;
    
    for attempt in 0..3 {
        if attempt > 0 {
            // 指数退避：等待 2^attempt 秒
            let delay = std::time::Duration::from_secs(2_u64.pow(attempt));
            tokio::time::sleep(delay).await;
        }
        
        match client
            .get(url)
            .send()
            .await
        {
            Ok(response) => {
                if !response.status().is_success() {
                    let status = response.status();
                    let error_msg = match status.as_u16() {
                        429 => "Rate limit exceeded. Please try again later.".to_string(),
                        500..=599 => "ArXiv server error. Please try again later.".to_string(),
                        _ => format!("API request failed with status: {}", status),
                    };
                    
                    if attempt == 2 { // 最后一次尝试
                        return Err(error_msg);
                    } else {
                        last_error = Some(error_msg);
                        continue;
                    }
                }

                match response.text().await {
                    Ok(xml_content) => {
                        if xml_content.trim().is_empty() {
                            return Err("Received empty response from ArXiv API".to_string());
                        }
                        
                        // 解析XML响应
                        return parse_arxiv_xml(&xml_content);
                    }
                    Err(e) => {
                        let error_msg = format!("Failed to read response content: {}", e);
                        if attempt == 2 {
                            return Err(error_msg);
                        } else {
                            last_error = Some(error_msg);
                        }
                    }
                }
            }
            Err(e) => {
                let error_msg = if e.is_timeout() {
                    "Request timed out. Please check your internet connection.".to_string()
                } else if e.is_connect() {
                    "Connection failed. Please check your internet connection.".to_string()
                } else {
                    format!("Network request failed: {}", e)
                };
                
                if attempt == 2 {
                    return Err(error_msg);
                } else {
                    last_error = Some(error_msg);
                }
            }
        }
    }
    
    Err(last_error.unwrap_or_else(|| "Unknown network error".to_string()))
}

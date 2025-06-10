// ArXiv API客户端实现

use crate::core::models::{ArxivPaper, SearchConfig};
use super::query_builder::build_search_url;
use crate::search::parsers::xml_parser::parse_arxiv_xml;

/// 基础搜索功能 - 简单查询接口
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
    if config.query.trim().is_empty() {
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
    // 发送HTTP请求
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "ArxivManager/1.0")
        .send()
        .await
        .map_err(|e| format!("网络请求失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("API请求失败，状态码: {}", response.status()));
    }

    let xml_content = response
        .text()
        .await
        .map_err(|e| format!("读取响应内容失败: {}", e))?;

    // 解析XML响应
    parse_arxiv_xml(&xml_content)
}

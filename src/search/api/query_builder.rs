// ArXiv查询URL构建器

use crate::core::models::{SearchConfig, DateRange};

/// 根据搜索配置构建完整的ArXiv API查询URL
pub fn build_search_url(config: &SearchConfig) -> Result<String, String> {
    let mut query_parts = Vec::new();
    
    // 基础查询
    let search_field = config.search_in.as_str();
    query_parts.push(format!("{}:{}", search_field, config.query));
    
    // 添加作者过滤
    for author in &config.authors {
        query_parts.push(format!("au:{}", author));
    }
    
    // 添加分类过滤
    for category in &config.categories {
        query_parts.push(format!("cat:{}", category));
    }
    
    // 构建最终查询字符串
    let search_query = query_parts.join(" AND ");
    let encoded_query = urlencoding::encode(&search_query);
    
    // 构建URL
    let mut url = format!(
        "https://export.arxiv.org/api/query?search_query={}&start=0&max_results={}",
        encoded_query, config.max_results
    );
    
    // 添加排序参数
    url.push_str(&format!(
        "&sortBy={}&sortOrder={}",
        config.sort_by.as_str(),
        config.sort_order.as_str()
    ));
    
    // 添加日期过滤（如果需要）
    if let DateRange::Custom { from, to } = &config.date_range {
        // arXiv API对日期过滤的支持有限，这里可以在结果中进行后处理
        url.push_str(&format!("&submittedDate:[{}+TO+{}]", from, to));
    }
    
    Ok(url)
}

/// 构建简单查询URL的辅助函数
#[allow(dead_code)]
pub fn build_simple_query_url(query: &str, max_results: u32) -> String {
    let encoded_query = urlencoding::encode(query);
    format!(
        "https://export.arxiv.org/api/query?search_query=all:{}&start=0&max_results={}&sortBy=submittedDate&sortOrder=descending",
        encoded_query, max_results
    )
}

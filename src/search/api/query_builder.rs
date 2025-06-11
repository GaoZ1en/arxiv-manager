// ArXiv查询URL构建器

use crate::core::models::{SearchConfig, DateRange};

/// 根据搜索配置构建完整的ArXiv API查询URL
pub fn build_search_url(config: &SearchConfig) -> Result<String, String> {
    let mut query_parts = Vec::new();
    
    // 基础查询 - 只有当查询不为空时才添加
    if !config.query.trim().is_empty() {
        let search_field = config.search_in.as_str();
        query_parts.push(format!("{}:{}", search_field, config.query.trim()));
    }
    
    // 精确短语搜索
    if let Some(exact_phrase) = &config.exact_phrase {
        if !exact_phrase.trim().is_empty() {
            query_parts.push(format!("all:\"{}\"", exact_phrase.trim()));
        }
    }
    
    // 排除词汇
    for exclude_word in &config.exclude_words {
        if !exclude_word.trim().is_empty() {
            query_parts.push(format!("NOT all:{}", exclude_word.trim()));
        }
    }
    
    // 添加作者过滤
    for author in &config.authors {
        if !author.trim().is_empty() {
            query_parts.push(format!("au:\"{}\"", author.trim()));
        }
    }
    
    // 期刊引用
    if let Some(journal_ref) = &config.journal_ref {
        if !journal_ref.trim().is_empty() {
            query_parts.push(format!("jr:\"{}\"", journal_ref.trim()));
        }
    }
    
    // 学科分类
    if let Some(subject_class) = &config.subject_class {
        if !subject_class.trim().is_empty() {
            query_parts.push(format!("subj-class:{}", subject_class.trim()));
        }
    }
    
    // 报告编号
    if let Some(report_number) = &config.report_number {
        if !report_number.trim().is_empty() {
            query_parts.push(format!("rn:{}", report_number.trim()));
        }
    }
    
    // ID列表 - arXiv ID的直接查询
    for id in &config.id_list {
        if !id.trim().is_empty() {
            query_parts.push(format!("id:{}", id.trim()));
        }
    }
    
    // 添加分类过滤
    for category in &config.categories {
        query_parts.push(format!("cat:{}", category.code()));
    }
    
    // 如果没有任何查询条件，返回错误
    if query_parts.is_empty() {
        return Err("At least one search criterion must be specified".to_string());
    }
    
    // 构建最终查询字符串
    let search_query = query_parts.join(" AND ");
    let encoded_query = urlencoding::encode(&search_query);
    
    // 构建URL
    let mut url = format!(
        "https://export.arxiv.org/api/query?search_query={}&start={}&max_results={}",
        encoded_query, config.start_index, config.max_results
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

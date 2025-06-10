// 外部服务和异步函数

use std::path::PathBuf;
use crate::models::{ArxivPaper, SearchConfig, DateRange};

// 高级搜索 arXiv 论文
pub async fn search_arxiv_papers_advanced(config: SearchConfig) -> Result<Vec<ArxivPaper>, String> {
    if config.query.trim().is_empty() {
        return Ok(vec![]);
    }

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

    // 发送HTTP请求
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
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
    let mut papers = parse_arxiv_xml(&xml_content)?;
    
    // 应用日期过滤（后处理）
    if let DateRange::LastWeek | DateRange::LastMonth | DateRange::LastYear = config.date_range {
        papers = filter_papers_by_date(papers, &config.date_range);
    }
    
    Ok(papers)
}

// 根据日期范围过滤论文
fn filter_papers_by_date(papers: Vec<ArxivPaper>, date_range: &DateRange) -> Vec<ArxivPaper> {
    use chrono::{DateTime, Utc, Duration};
    
    let cutoff_date = match date_range {
        DateRange::LastWeek => Utc::now() - Duration::weeks(1),
        DateRange::LastMonth => Utc::now() - Duration::weeks(4),
        DateRange::LastYear => Utc::now() - Duration::weeks(52),
        _ => return papers,
    };
    
    papers.into_iter().filter(|paper| {
        if let Ok(published_date) = DateTime::parse_from_rfc3339(&paper.published) {
            published_date.with_timezone(&Utc) > cutoff_date
        } else {
            true // 如果无法解析日期，保留论文
        }
    }).collect()
}

// 异步搜索 arXiv 论文 - 真实API实现
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

    // 发送HTTP请求
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
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

// 解析arXiv API返回的XML格式数据 - 简化版本
fn parse_arxiv_xml(xml_content: &str) -> Result<Vec<ArxivPaper>, String> {
    let mut papers = Vec::new();
    
    // 查找所有 <entry> 标签
    let entries: Vec<&str> = xml_content.split("<entry>").skip(1).collect();
    
    for entry in entries {
        if let Some(end_pos) = entry.find("</entry>") {
            let entry_content = &entry[..end_pos];
            
            let mut paper = ArxivPaper {
                id: String::new(),
                title: String::new(),
                authors: Vec::new(),
                abstract_text: String::new(),
                published: String::new(),
                updated: String::new(),
                categories: Vec::new(),
                pdf_url: String::new(),
                entry_url: String::new(),
            };
            
            // 提取ID
            if let Some(id_content) = extract_xml_content(entry_content, "id") {
                paper.entry_url = id_content.trim().to_string();
                
                // 从URL中提取arXiv ID
                let clean_url = id_content.trim();
                if let Some(id_part) = clean_url.split('/').last() {
                    paper.id = id_part.to_string();
                    // 构建标准的PDF URL
                    paper.pdf_url = format!("https://arxiv.org/pdf/{}.pdf", paper.id);
                }
            }
            
            // 提取标题
            if let Some(title) = extract_xml_content(entry_content, "title") {
                paper.title = title.trim().to_string();
            }
            
            // 提取摘要
            if let Some(summary) = extract_xml_content(entry_content, "summary") {
                paper.abstract_text = summary.trim().to_string();
            }
            
            // 提取发布日期
            if let Some(published) = extract_xml_content(entry_content, "published") {
                paper.published = published.trim().to_string();
            }
            
            // 提取更新日期
            if let Some(updated) = extract_xml_content(entry_content, "updated") {
                paper.updated = updated.trim().to_string();
            }
            
            // 提取作者
            let mut current_pos = 0;
            while let Some(author_start) = entry_content[current_pos..].find("<name>") {
                let start = current_pos + author_start + 6;
                if let Some(author_end) = entry_content[start..].find("</name>") {
                    let author = entry_content[start..start + author_end].trim().to_string();
                    if !author.is_empty() && !paper.authors.contains(&author) {
                        paper.authors.push(author);
                    }
                    current_pos = start + author_end + 7;
                } else {
                    break;
                }
            }
            
            // 提取分类
            current_pos = 0;
            while let Some(cat_start) = entry_content[current_pos..].find("category term=\"") {
                let start = current_pos + cat_start + 15;
                if let Some(cat_end) = entry_content[start..].find("\"") {
                    let category = entry_content[start..start + cat_end].trim().to_string();
                    if !category.is_empty() && !paper.categories.contains(&category) {
                        paper.categories.push(category);
                    }
                    current_pos = start + cat_end + 1;
                } else {
                    break;
                }
            }
            
            // 只添加有效的论文
            if !paper.id.is_empty() && !paper.title.is_empty() {
                papers.push(paper);
            }
        }
    }
    
    Ok(papers)
}

// 辅助函数：从XML内容中提取指定标签的内容
fn extract_xml_content(xml: &str, tag: &str) -> Option<String> {
    let start_tag = format!("<{}>", tag);
    let end_tag = format!("</{}>", tag);
    
    if let Some(start_pos) = xml.find(&start_tag) {
        let content_start = start_pos + start_tag.len();
        if let Some(end_pos) = xml[content_start..].find(&end_tag) {
            return Some(xml[content_start..content_start + end_pos].to_string());
        }
    }
    
    None
}

// 异步下载 PDF - 真实下载实现
pub async fn download_pdf(paper: ArxivPaper) -> Result<(String, PathBuf), (String, String)> {
    use tokio::fs;
    
    // 创建下载目录
    let downloads_dir = PathBuf::from("downloads");
    if let Err(e) = fs::create_dir_all(&downloads_dir).await {
        return Err((paper.id.clone(), format!("创建下载目录失败: {}", e)));
    }
    
    // 构建文件路径
    let file_path = downloads_dir.join(format!("{}.pdf", paper.id));
    
    // 构建正确的PDF下载URL
    let pdf_url = if paper.pdf_url.starts_with("http") {
        paper.pdf_url.clone()
    } else {
        format!("https://arxiv.org/pdf/{}.pdf", paper.id)
    };
    
    println!("正在下载PDF: {}", pdf_url);
    
    // 下载PDF文件
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| (paper.id.clone(), format!("创建HTTP客户端失败: {}", e)))?;
        
    let response = match client
        .get(&pdf_url)
        .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36")
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => return Err((paper.id.clone(), format!("下载请求失败: {}", e))),
    };
    
    if !response.status().is_success() {
        return Err((paper.id.clone(), format!("下载失败，状态码: {} - URL: {}", response.status(), pdf_url)));
    }
    
    // 获取文件内容
    let content = match response.bytes().await {
        Ok(bytes) => bytes,
        Err(e) => return Err((paper.id.clone(), format!("读取文件内容失败: {}", e))),
    };
    
    // 验证是否为PDF文件
    if content.len() < 4 || &content[0..4] != b"%PDF" {
        return Err((paper.id.clone(), "下载的文件不是有效的PDF格式".to_string()));
    }
    
    // 保存文件
    match fs::write(&file_path, content).await {
        Ok(_) => {
            println!("PDF下载成功: {:?}", file_path);
            Ok((paper.id, file_path))
        },
        Err(e) => Err((paper.id.clone(), format!("保存文件失败: {}", e))),
    }
}

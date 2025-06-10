// 外部服务和异步函数

use std::path::PathBuf;
use crate::models::ArxivPaper;

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

// ArXiv XML响应解析器

use crate::core::models::ArxivPaper;

/// 解析arXiv API返回的XML格式数据
pub fn parse_arxiv_xml(xml_content: &str) -> Result<Vec<ArxivPaper>, String> {
    if xml_content.trim().is_empty() {
        return Err("Empty XML content received".to_string());
    }
    
    // 检查是否包含错误信息
    if xml_content.contains("<error>") || xml_content.contains("error") {
        // 尝试提取错误信息
        if let Some(error_msg) = extract_xml_content(xml_content, "error") {
            return Err(format!("ArXiv API error: {}", error_msg));
        }
        // 如果找不到具体错误信息，但包含error标签
        if xml_content.to_lowercase().contains("malformed") {
            return Err("Malformed query. Please check your search terms.".to_string());
        }
    }
    
    // 检查是否为有效的Atom feed
    if !xml_content.contains("<feed") && !xml_content.contains("<entry") {
        return Err("Invalid XML format: Expected Atom feed from ArXiv API".to_string());
    }
    
    let mut papers = Vec::new();
    
    // 查找所有 <entry> 标签
    let entries: Vec<&str> = xml_content.split("<entry>").skip(1).collect();
    
    for entry in entries {
        // 结束于 </entry> 标签
        let entry = entry.split("</entry>").next().unwrap_or(entry);
        
        // 提取各个字段
        let id = extract_xml_content(entry, "id")
            .and_then(|id| id.split('/').last().map(|s| s.to_string()))
            .unwrap_or_else(|| format!("unknown_{}", papers.len()));
            
        let title = extract_xml_content(entry, "title")
            .map(|s| s.trim().replace('\n', " ").replace("  ", " "))
            .unwrap_or_else(|| "Unknown Title".to_string());
            
        let summary = extract_xml_content(entry, "summary")
            .map(|s| s.trim().replace('\n', " ").replace("  ", " "))
            .unwrap_or_else(|| "No abstract available".to_string());
            
        let published = extract_xml_content(entry, "published")
            .unwrap_or_else(|| "Unknown".to_string());
            
        let updated = extract_xml_content(entry, "updated")
            .unwrap_or_else(|| published.clone());
        
        // 提取作者信息
        let authors = extract_authors_from_entry(entry);
        
        // 提取分类信息
        let categories = extract_categories_from_entry(entry);
        
        // 构建PDF URL - 先尝试从link元素中找到PDF链接
        let pdf_url = {
            // 查找包含"pdf"的链接
            if let Some(pdf_link) = extract_links(entry).into_iter()
                .find(|link| link.contains("pdf")) {
                pdf_link
            } else {
                // 回退到基于ID构建URL，处理旧格式的ID
                let clean_id = if id.contains('/') {
                    id.clone()
                } else {
                    // 对于纯数字ID，从完整的entry ID中提取类别
                    extract_xml_content(entry, "id")
                        .and_then(|full_id| {
                            // full_id 格式通常是 "http://arxiv.org/abs/math/0311136v1"
                            full_id.split("/abs/").nth(1).map(|s| s.to_string())
                        })
                        .unwrap_or(id.clone())
                };
                format!("https://arxiv.org/pdf/{}.pdf", clean_id)
            }
        };
        let entry_url = format!("https://arxiv.org/abs/{}", id);
        
        // 提取可选字段
        let doi = extract_xml_content(entry, "arxiv:doi");
        let journal_ref = extract_xml_content(entry, "arxiv:journal_ref");
        let comments = extract_xml_content(entry, "arxiv:comment");
        
        let paper = ArxivPaper {
            id,
            title,
            authors,
            abstract_text: summary,
            published,
            updated,
            categories,
            pdf_url,
            entry_url,
            doi,
            journal_ref,
            comments,
            // 新增的库管理字段
            is_favorite: false,
            added_at: None, // 论文保存时才设置
            collection_ids: Vec::new(),
            tags: Vec::new(),
            notes: None,
            read_status: crate::core::models::ReadingStatus::Unread,
            rating: None,
            local_file_path: None,
        };
        
        papers.push(paper);
    }
    
    Ok(papers)
}

/// 从XML条目中提取作者信息
fn extract_authors_from_entry(entry: &str) -> Vec<String> {
    let mut authors = Vec::new();
    
    // 查找所有作者标签
    let author_sections: Vec<&str> = entry.split("<author>").skip(1).collect();
    for section in author_sections {
        if let Some(author_content) = section.split("</author>").next() {
            if let Some(name) = extract_xml_content(author_content, "name") {
                authors.push(name.trim().to_string());
            }
        }
    }
    
    // 如果没有找到作者，尝试备用方法
    if authors.is_empty() {
        if let Some(author_text) = extract_xml_content(entry, "author") {
            authors.push(author_text.trim().to_string());
        }
    }
    
    authors
}

/// 从XML条目中提取分类信息
fn extract_categories_from_entry(entry: &str) -> Vec<String> {
    let mut categories = Vec::new();
    
    // 查找所有分类标签
    let category_sections: Vec<&str> = entry.split("<category ").skip(1).collect();
    for section in category_sections {
        if let Some(term_start) = section.find("term=\"") {
            let term_content = &section[term_start + 6..];
            if let Some(term_end) = term_content.find('"') {
                let category = term_content[..term_end].to_string();
                categories.push(category);
            }
        }
    }
    
    // 如果没有找到分类，添加默认分类
    if categories.is_empty() {
        categories.push("general".to_string());
    }
    
    categories
}

/// 从XML字符串中提取指定标签的内容
fn extract_xml_content(xml: &str, tag: &str) -> Option<String> {
    let start_tag = format!("<{}>", tag);
    let end_tag = format!("</{}>", tag);
    
    if let Some(start) = xml.find(&start_tag) {
        let content_start = start + start_tag.len();
        if let Some(end) = xml[content_start..].find(&end_tag) {
            let content = &xml[content_start..content_start + end];
            return Some(content.trim().to_string());
        }
    }
    
    None
}

/// 从entry中提取所有链接
fn extract_links(entry: &str) -> Vec<String> {
    let mut links = Vec::new();
    
    // 查找所有 <link> 标签
    let mut remaining = entry;
    while let Some(start) = remaining.find("<link") {
        if let Some(end) = remaining[start..].find("/>") {
            let link_tag = &remaining[start..start + end + 2];
            
            // 提取href属性
            if let Some(href_start) = link_tag.find("href=\"") {
                let href_content_start = href_start + 6; // len("href=\"")
                if let Some(href_end) = link_tag[href_content_start..].find("\"") {
                    let href = &link_tag[href_content_start..href_content_start + href_end];
                    links.push(href.to_string());
                }
            }
            
            remaining = &remaining[start + end + 2..];
        } else {
            break;
        }
    }
    
    links
}

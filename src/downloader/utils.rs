// 下载器工具函数模块

use crate::core::models::ArxivPaper;
use std::path::{Path, PathBuf};

/// 生成文件路径
pub fn generate_file_path(paper: &ArxivPaper, base_dir: &Path, pattern: &str) -> PathBuf {
    // Extract year from published date string (assuming format like "2023-01-15T12:00:00Z")
    let year = paper.published.split('-').next().unwrap_or("unknown").to_string();
    let category = paper.categories.first()
        .map(|c| c.replace(".", "_"))
        .unwrap_or_else(|| "unknown".to_string());
    
    // Sanitize title for filename
    let title = sanitize_filename(&paper.title);
    let filename = format!("{}.pdf", paper.id);
    
    let path_str = pattern
        .replace("{year}", &year)
        .replace("{category}", &category)
        .replace("{title}", &title)
        .replace("{id}", &paper.id);
    
    let mut path = base_dir.to_path_buf();
    
    // Split path by '/' and build the directory structure
    for component in path_str.split('/') {
        if !component.is_empty() {
            path.push(component);
        }
    }
    
    // Ensure we end with the filename
    if !path.to_string_lossy().ends_with(".pdf") {
        path.push(filename);
    }
    
    path
}

/// 清理文件名中的非法字符
pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect::<String>()
        .trim()
        .to_string()
}

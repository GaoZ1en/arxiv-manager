// PDF文件下载功能

use std::path::PathBuf;
use tokio::fs;
use crate::core::models::ArxivPaper;

/// 下载arXiv论文的PDF文件
#[allow(dead_code)]
pub async fn download_pdf(paper: ArxivPaper) -> Result<(String, PathBuf), (String, String)> {
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

/// 检查PDF文件是否已经下载
#[allow(dead_code)]
pub async fn is_pdf_downloaded(paper_id: &str) -> bool {
    let file_path = PathBuf::from("downloads").join(format!("{}.pdf", paper_id));
    file_path.exists()
}

/// 获取已下载PDF文件的路径
#[allow(dead_code)]
pub fn get_pdf_path(paper_id: &str) -> PathBuf {
    PathBuf::from("downloads").join(format!("{}.pdf", paper_id))
}

/// 删除已下载的PDF文件
#[allow(dead_code)]
pub async fn delete_pdf(paper_id: &str) -> Result<(), String> {
    let file_path = get_pdf_path(paper_id);
    if file_path.exists() {
        fs::remove_file(file_path).await
            .map_err(|e| format!("删除文件失败: {}", e))?;
    }
    Ok(())
}

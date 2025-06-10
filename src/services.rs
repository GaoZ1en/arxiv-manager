// 外部服务和异步函数

use std::path::PathBuf;
use crate::models::ArxivPaper;

// 异步搜索 arXiv 论文
pub async fn search_arxiv_papers(query: String) -> Result<Vec<ArxivPaper>, String> {
    // 模拟 API 调用
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    
    // 返回模拟结果
    Ok(vec![
        ArxivPaper {
            id: "2301.00001".to_string(),
            title: format!("Sample Paper about {}", query),
            authors: vec!["John Doe".to_string(), "Jane Smith".to_string()],
            abstract_text: "This is a sample abstract for demonstration purposes. It contains detailed information about the research methodology and findings.".to_string(),
            published: "2023-01-01".to_string(),
            updated: "2023-01-01".to_string(),
            categories: vec!["cs.LG".to_string(), "stat.ML".to_string()],
            pdf_url: "https://arxiv.org/pdf/2301.00001.pdf".to_string(),
            entry_url: "https://arxiv.org/abs/2301.00001".to_string(),
        },
        ArxivPaper {
            id: "2301.00002".to_string(),
            title: format!("Another Research on {}", query),
            authors: vec!["Alice Johnson".to_string(), "Bob Wilson".to_string()],
            abstract_text: "Another comprehensive study exploring different aspects of the research topic with novel approaches and insights.".to_string(),
            published: "2023-01-02".to_string(),
            updated: "2023-01-02".to_string(),
            categories: vec!["cs.AI".to_string(), "cs.CV".to_string()],
            pdf_url: "https://arxiv.org/pdf/2301.00002.pdf".to_string(),
            entry_url: "https://arxiv.org/abs/2301.00002".to_string(),
        }
    ])
}

// 异步下载 PDF
pub async fn download_pdf(paper: ArxivPaper) -> Result<(String, PathBuf), (String, String)> {
    // 模拟下载延迟
    tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
    
    let file_path = PathBuf::from(format!("downloads/{}.pdf", paper.id));
    Ok((paper.id, file_path))
}

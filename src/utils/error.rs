use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ArxivError {
    #[error("网络请求失败: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("数据库错误: {0}")]
    Database(#[from] rusqlite::Error),
    
    #[error("JSON 解析错误: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("XML 解析错误: {0}")]
    Xml(String),
    
    #[error("文件系统错误: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("配置错误: {0}")]
    Config(String),
    
    #[error("arXiv API 错误: {0}")]
    ArxivApi(String),
    
    #[error("PDF 处理错误: {0}")]
    Pdf(String),
    
    #[error("搜索引擎错误: {0}")]
    Search(String),
    
    #[error("下载错误: {0}")]
    Download(String),
    
    #[error("未知错误: {0}")]
    Unknown(String),
}

#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, ArxivError>;

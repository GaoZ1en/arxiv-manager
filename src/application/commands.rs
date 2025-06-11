// 命令处理 - Commands
// 实现修改数据状态的操作命令

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domains::paper::*;

/// 命令特征 - 所有命令必须实现此特征
pub trait Command {
    type Response;
    
    /// 获取命令名称
    fn command_name(&self) -> &'static str;
    
    /// 验证命令参数
    fn validate(&self) -> Result<(), String>;
}

/// 创建论文命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaperCommand {
    pub arxiv_id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub abstract_text: String,
    pub categories: Vec<String>,
    pub published_date: DateTime<Utc>,
    pub updated_date: Option<DateTime<Utc>>,
    pub pdf_url: Option<String>,
    pub arxiv_url: String,
    pub doi: Option<String>,
    pub journal_ref: Option<String>,
    pub comments: Option<String>,
}

impl Command for CreatePaperCommand {
    type Response = String; // 返回创建的论文ID
    
    fn command_name(&self) -> &'static str {
        "CreatePaper"
    }
    
    fn validate(&self) -> Result<(), String> {
        if self.arxiv_id.is_empty() {
            return Err("ArXiv ID cannot be empty".to_string());
        }
        if self.title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        if self.authors.is_empty() {
            return Err("Authors list cannot be empty".to_string());
        }
        if self.abstract_text.is_empty() {
            return Err("Abstract cannot be empty".to_string());
        }
        if self.categories.is_empty() {
            return Err("Categories list cannot be empty".to_string());
        }
        if self.arxiv_url.is_empty() {
            return Err("ArXiv URL cannot be empty".to_string());
        }
        Ok(())
    }
}

/// 更新论文元数据命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePaperMetadataCommand {
    pub paper_id: String,
    pub title: Option<String>,
    pub authors: Option<Vec<String>>,
    pub abstract_text: Option<String>,
    pub categories: Option<Vec<String>>,
    pub updated_date: Option<DateTime<Utc>>,
    pub pdf_url: Option<String>,
    pub doi: Option<String>,
    pub journal_ref: Option<String>,
    pub comments: Option<String>,
}

impl Command for UpdatePaperMetadataCommand {
    type Response = ();
    
    fn command_name(&self) -> &'static str {
        "UpdatePaperMetadata"
    }
    
    fn validate(&self) -> Result<(), String> {
        if self.paper_id.is_empty() {
            return Err("Paper ID cannot be empty".to_string());
        }
        
        // 检查是否至少有一个字段要更新
        let has_update = self.title.is_some()
            || self.authors.is_some()
            || self.abstract_text.is_some()
            || self.categories.is_some()
            || self.updated_date.is_some()
            || self.pdf_url.is_some()
            || self.doi.is_some()
            || self.journal_ref.is_some()
            || self.comments.is_some();
            
        if !has_update {
            return Err("At least one field must be specified for update".to_string());
        }
        
        Ok(())
    }
}

/// 更新阅读状态命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateReadingStatusCommand {
    pub paper_id: String,
    pub reading_status: ReadingStatus,
}

impl Command for UpdateReadingStatusCommand {
    type Response = ();
    
    fn command_name(&self) -> &'static str {
        "UpdateReadingStatus"
    }
    
    fn validate(&self) -> Result<(), String> {
        if self.paper_id.is_empty() {
            return Err("Paper ID cannot be empty".to_string());
        }
        Ok(())
    }
}

/// 更新论文标签命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePaperTagsCommand {
    pub paper_id: String,
    pub tags: Vec<String>,
    pub operation: TagOperation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TagOperation {
    Set,    // 设置标签（替换现有标签）
    Add,    // 添加标签
    Remove, // 移除标签
}

impl Command for UpdatePaperTagsCommand {
    type Response = ();
    
    fn command_name(&self) -> &'static str {
        "UpdatePaperTags"
    }
    
    fn validate(&self) -> Result<(), String> {
        if self.paper_id.is_empty() {
            return Err("Paper ID cannot be empty".to_string());
        }
        if self.tags.is_empty() {
            return Err("Tags list cannot be empty".to_string());
        }
        Ok(())
    }
}

/// 更新论文评分命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePaperRatingCommand {
    pub paper_id: String,
    pub rating: Option<u8>, // None 表示移除评分
}

impl Command for UpdatePaperRatingCommand {
    type Response = ();
    
    fn command_name(&self) -> &'static str {
        "UpdatePaperRating"
    }
    
    fn validate(&self) -> Result<(), String> {
        if self.paper_id.is_empty() {
            return Err("Paper ID cannot be empty".to_string());
        }
        if let Some(rating) = self.rating {
            if rating > 5 {
                return Err("Rating must be between 1 and 5".to_string());
            }
        }
        Ok(())
    }
}

/// 更新论文笔记命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePaperNotesCommand {
    pub paper_id: String,
    pub notes: Option<String>, // None 表示删除笔记
}

impl Command for UpdatePaperNotesCommand {
    type Response = ();
    
    fn command_name(&self) -> &'static str {
        "UpdatePaperNotes"
    }
    
    fn validate(&self) -> Result<(), String> {
        if self.paper_id.is_empty() {
            return Err("Paper ID cannot be empty".to_string());
        }
        Ok(())
    }
}

/// 设置收藏状态命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetFavoriteCommand {
    pub paper_id: String,
    pub is_favorite: bool,
}

impl Command for SetFavoriteCommand {
    type Response = ();
    
    fn command_name(&self) -> &'static str {
        "SetFavorite"
    }
    
    fn validate(&self) -> Result<(), String> {
        if self.paper_id.is_empty() {
            return Err("Paper ID cannot be empty".to_string());
        }
        Ok(())
    }
}

/// 删除论文命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePaperCommand {
    pub paper_id: String,
    pub delete_local_file: bool, // 是否同时删除本地文件
}

impl Command for DeletePaperCommand {
    type Response = ();
    
    fn command_name(&self) -> &'static str {
        "DeletePaper"
    }
    
    fn validate(&self) -> Result<(), String> {
        if self.paper_id.is_empty() {
            return Err("Paper ID cannot be empty".to_string());
        }
        Ok(())
    }
}

/// 下载论文命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadPaperCommand {
    pub paper_id: String,
    pub download_path: Option<String>,
    pub force_redownload: bool,
}

impl Command for DownloadPaperCommand {
    type Response = String; // 返回下载的文件路径
    
    fn command_name(&self) -> &'static str {
        "DownloadPaper"
    }
    
    fn validate(&self) -> Result<(), String> {
        if self.paper_id.is_empty() {
            return Err("Paper ID cannot be empty".to_string());
        }
        Ok(())
    }
}

/// 批量更新阅读状态命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateReadingStatusCommand {
    pub paper_ids: Vec<String>,
    pub reading_status: ReadingStatus,
}

impl Command for BatchUpdateReadingStatusCommand {
    type Response = Vec<String>; // 返回成功更新的论文ID列表
    
    fn command_name(&self) -> &'static str {
        "BatchUpdateReadingStatus"
    }
    
    fn validate(&self) -> Result<(), String> {
        if self.paper_ids.is_empty() {
            return Err("Paper IDs list cannot be empty".to_string());
        }
        for paper_id in &self.paper_ids {
            if paper_id.is_empty() {
                return Err("Paper ID cannot be empty".to_string());
            }
        }
        Ok(())
    }
}

/// 批量添加标签命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchAddTagsCommand {
    pub paper_ids: Vec<String>,
    pub tags: Vec<String>,
}

impl Command for BatchAddTagsCommand {
    type Response = Vec<String>; // 返回成功更新的论文ID列表
    
    fn command_name(&self) -> &'static str {
        "BatchAddTags"
    }
    
    fn validate(&self) -> Result<(), String> {
        if self.paper_ids.is_empty() {
            return Err("Paper IDs list cannot be empty".to_string());
        }
        if self.tags.is_empty() {
            return Err("Tags list cannot be empty".to_string());
        }
        for paper_id in &self.paper_ids {
            if paper_id.is_empty() {
                return Err("Paper ID cannot be empty".to_string());
            }
        }
        Ok(())
    }
}

/// 批量删除论文命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeletePapersCommand {
    pub paper_ids: Vec<String>,
    pub delete_local_files: bool,
}

impl Command for BatchDeletePapersCommand {
    type Response = Vec<String>; // 返回成功删除的论文ID列表
    
    fn command_name(&self) -> &'static str {
        "BatchDeletePapers"
    }
    
    fn validate(&self) -> Result<(), String> {
        if self.paper_ids.is_empty() {
            return Err("Paper IDs list cannot be empty".to_string());
        }
        for paper_id in &self.paper_ids {
            if paper_id.is_empty() {
                return Err("Paper ID cannot be empty".to_string());
            }
        }
        Ok(())
    }
}

/// 创建收藏夹命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCollectionCommand {
    pub name: String,
    pub description: Option<String>,
    pub paper_ids: Vec<String>,
}

impl Command for CreateCollectionCommand {
    type Response = String; // 返回创建的收藏夹ID
    
    fn command_name(&self) -> &'static str {
        "CreateCollection"
    }
    
    fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Collection name cannot be empty".to_string());
        }
        if self.name.len() > 100 {
            return Err("Collection name cannot exceed 100 characters".to_string());
        }
        Ok(())
    }
}

/// 导入论文命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportPapersCommand {
    pub source: ImportSource,
    pub data: String,
    pub skip_duplicates: bool,
    pub update_existing: bool,
    pub auto_download: bool,
    pub default_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportSource {
    ArxivApi,
    BibtexFile,
    JsonFile,
    CsvFile,
}

impl Command for ImportPapersCommand {
    type Response = ImportResult;
    
    fn command_name(&self) -> &'static str {
        "ImportPapers"
    }
    
    fn validate(&self) -> Result<(), String> {
        if self.data.is_empty() {
            return Err("Import data cannot be empty".to_string());
        }
        Ok(())
    }
}

/// 导入结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResult {
    pub imported_count: u32,
    pub skipped_count: u32,
    pub failed_count: u32,
    pub errors: Vec<String>,
}

/// 命令执行结果
#[derive(Debug, Clone)]
pub enum CommandResult<T> {
    Success(T),
    ValidationError(String),
    DomainError(String),
    InfrastructureError(String),
}

impl<T> CommandResult<T> {
    pub fn is_success(&self) -> bool {
        matches!(self, CommandResult::Success(_))
    }
    
    pub fn unwrap(self) -> T {
        match self {
            CommandResult::Success(value) => value,
            CommandResult::ValidationError(msg) => panic!("Validation error: {}", msg),
            CommandResult::DomainError(msg) => panic!("Domain error: {}", msg),
            CommandResult::InfrastructureError(msg) => panic!("Infrastructure error: {}", msg),
        }
    }
    
    pub fn map<U, F>(self, f: F) -> CommandResult<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            CommandResult::Success(value) => CommandResult::Success(f(value)),
            CommandResult::ValidationError(msg) => CommandResult::ValidationError(msg),
            CommandResult::DomainError(msg) => CommandResult::DomainError(msg),
            CommandResult::InfrastructureError(msg) => CommandResult::InfrastructureError(msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_create_paper_command_validation() {
        let mut command = CreatePaperCommand {
            arxiv_id: "2301.12345".to_string(),
            title: "Test Paper".to_string(),
            authors: vec!["Author 1".to_string()],
            abstract_text: "Test abstract".to_string(),
            categories: vec!["cs.AI".to_string()],
            published_date: Utc::now(),
            updated_date: None,
            pdf_url: Some("http://example.com/test.pdf".to_string()),
            arxiv_url: "http://arxiv.org/abs/2301.12345".to_string(),
            doi: None,
            journal_ref: None,
            comments: None,
        };
        
        assert!(command.validate().is_ok());
        
        // 测试空标题
        command.title = String::new();
        assert!(command.validate().is_err());
    }
    
    #[test]
    fn test_update_rating_command_validation() {
        let command = UpdatePaperRatingCommand {
            paper_id: "2301.12345".to_string(),
            rating: Some(3),
        };
        assert!(command.validate().is_ok());
        
        let invalid_command = UpdatePaperRatingCommand {
            paper_id: "2301.12345".to_string(),
            rating: Some(10), // 无效评分
        };
        assert!(invalid_command.validate().is_err());
    }
    
    #[test]
    fn test_command_result() {
        let success: CommandResult<String> = CommandResult::Success("test".to_string());
        assert!(success.is_success());
        
        let mapped = success.map(|s| s.len());
        match mapped {
            CommandResult::Success(len) => assert_eq!(len, 4),
            _ => panic!("Expected success"),
        }
    }
}

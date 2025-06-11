// 论文领域服务 - Paper Domain Services
// 包含不属于单个实体的业务逻辑和复杂操作

use super::models::*;
use crate::domains::paper::repositories::PaperRepository;
use chrono::Utc;
use std::collections::HashSet;

/// 论文管理服务 - 核心业务逻辑
pub struct PaperService<R: PaperRepository> {
    repository: R,
}

impl<R: PaperRepository> PaperService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// 保存论文到个人库
    pub async fn save_paper(&self, paper: Paper) -> Result<(), PaperServiceError> {
        // 业务规则：检查论文是否已存在
        if self.repository.exists(&paper.id).await? {
            return Err(PaperServiceError::PaperAlreadyExists(paper.id.arxiv_id));
        }

        // 验证论文数据完整性
        self.validate_paper(&paper)?;

        // 保存到存储库
        self.repository.save(paper).await?;
        
        Ok(())
    }

    /// 更新论文信息
    pub async fn update_paper(&self, paper: Paper) -> Result<(), PaperServiceError> {
        // 业务规则：论文必须已存在
        if !self.repository.exists(&paper.id).await? {
            return Err(PaperServiceError::PaperNotFound(paper.id.arxiv_id));
        }

        // 验证更新数据
        self.validate_paper(&paper)?;

        // 更新存储库
        self.repository.update(paper).await?;
        
        Ok(())
    }

    /// 删除论文
    pub async fn delete_paper(&self, paper_id: &PaperId) -> Result<(), PaperServiceError> {
        // 业务规则：论文必须存在
        if !self.repository.exists(paper_id).await? {
            return Err(PaperServiceError::PaperNotFound(paper_id.arxiv_id.clone()));
        }

        self.repository.delete(paper_id).await?;
        Ok(())
    }

    /// 按分类获取论文
    pub async fn get_papers_by_category(&self, category: &ArxivCategory) -> Result<Vec<Paper>, PaperServiceError> {
        self.repository.find_by_category(category).await
            .map_err(|e| e.into())
    }

    /// 按标签获取论文
    pub async fn get_papers_by_tag(&self, tag_name: &str) -> Result<Vec<Paper>, PaperServiceError> {
        self.repository.find_by_tag(tag_name).await
            .map_err(|e| e.into())
    }

    /// 获取阅读列表
    pub async fn get_reading_list(&self, status: ReadingStatus) -> Result<Vec<Paper>, PaperServiceError> {
        self.repository.find_by_reading_status(status).await
            .map_err(|e| e.into())
    }

    /// 批量导入论文
    pub async fn batch_import(&self, papers: Vec<Paper>) -> Result<ImportResult, PaperServiceError> {
        let mut successful = 0;
        let mut failed = Vec::new();
        let mut skipped = 0;

        for paper in papers {
            match self.save_paper(paper.clone()).await {
                Ok(_) => successful += 1,
                Err(PaperServiceError::PaperAlreadyExists(_)) => skipped += 1,
                Err(e) => failed.push(ImportFailure {
                    paper_id: paper.id.arxiv_id,
                    reason: e.to_string(),
                }),
            }
        }

        Ok(ImportResult {
            successful,
            skipped,
            failed,
        })
    }

    /// 验证论文数据完整性
    fn validate_paper(&self, paper: &Paper) -> Result<(), PaperServiceError> {
        // 标题不能为空
        if paper.metadata.title.trim().is_empty() {
            return Err(PaperServiceError::InvalidData("Title cannot be empty".to_string()));
        }

        // 至少需要一个作者
        if paper.metadata.authors.primary.is_empty() {
            return Err(PaperServiceError::InvalidData("At least one author is required".to_string()));
        }

        // 摘要不能为空
        if paper.metadata.abstract_text.trim().is_empty() {
            return Err(PaperServiceError::InvalidData("Abstract cannot be empty".to_string()));
        }

        // 必须有主要分类
        if paper.classification.primary_category.code.is_empty() {
            return Err(PaperServiceError::InvalidData("Primary category is required".to_string()));
        }

        Ok(())
    }
}

/// 元数据提取服务 - 从外部源提取和规范化论文元数据
pub struct MetadataExtractorService;

impl MetadataExtractorService {
    pub fn new() -> Self {
        Self
    }

    /// 从ArXiv数据创建Paper实体
    pub fn from_arxiv_data(&self, arxiv_data: ArxivData) -> Result<Paper, MetadataExtractionError> {
        let paper_id = PaperId::new(&arxiv_data.id)?;
        
        let metadata = PaperMetadata {
            title: self.clean_title(&arxiv_data.title),
            authors: self.parse_authors(&arxiv_data.authors)?,
            abstract_text: self.clean_abstract(&arxiv_data.abstract_text),
            keywords: self.extract_keywords(&arxiv_data.abstract_text),
            language: "en".to_string(), // 默认英语
        };

        let content = PaperContent {
            pdf_url: Url::new(arxiv_data.pdf_url)?,
            entry_url: Url::new(arxiv_data.entry_url)?,
            source_files: Vec::new(), // 需要额外查询
            supplementary: Vec::new(),
        };

        let classification = PaperClassification {
            primary_category: self.parse_category(&arxiv_data.primary_category)?,
            secondary_categories: self.parse_categories(&arxiv_data.categories)?,
            subject_classes: Vec::new(), // 需要映射
            msc_classes: Vec::new(),
            acm_classes: Vec::new(),
        };

        let publication = PublicationInfo {
            published_date: arxiv_data.published,
            updated_date: arxiv_data.updated,
            version: 1, // 默认版本
            doi: arxiv_data.doi,
            journal_reference: arxiv_data.journal_ref.map(|j| self.parse_journal_ref(&j)),
            comments: arxiv_data.comments,
            license: None, // 需要查询
        };

        Ok(Paper::new(
            paper_id.arxiv_id,
            metadata,
            content,
            classification,
            publication,
        ))
    }

    /// 清理标题格式
    fn clean_title(&self, title: &str) -> String {
        title.trim()
            .replace('\n', " ")
            .replace("  ", " ")
            .to_string()
    }

    /// 解析作者信息
    fn parse_authors(&self, authors_str: &str) -> Result<Authors, MetadataExtractionError> {
        let authors: Vec<Author> = authors_str
            .split(',')
            .map(|name| Author {
                name: name.trim().to_string(),
                orcid: None, // 需要额外查询
            })
            .collect();

        if authors.is_empty() {
            return Err(MetadataExtractionError::InvalidAuthorData);
        }

        Ok(Authors {
            primary: authors,
            affiliations: std::collections::HashMap::new(),
        })
    }

    /// 清理摘要格式
    fn clean_abstract(&self, abstract_text: &str) -> String {
        abstract_text.trim()
            .replace('\n', " ")
            .replace("  ", " ")
            .to_string()
    }

    /// 从摘要提取关键词
    fn extract_keywords(&self, abstract_text: &str) -> Vec<String> {
        // 简单的关键词提取逻辑
        // 在实际应用中，这里可以使用更复杂的NLP技术
        let stop_words: HashSet<&str> = ["the", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with", "by", "from", "this", "that", "these", "those", "a", "an"]
            .iter().cloned().collect();

        abstract_text
            .to_lowercase()
            .split_whitespace()
            .filter(|word| word.len() > 3 && !stop_words.contains(word))
            .take(10) // 取前10个词作为关键词
            .map(|s| s.to_string())
            .collect()
    }

    /// 解析分类信息
    fn parse_category(&self, category_str: &str) -> Result<ArxivCategory, MetadataExtractionError> {
        // 这里应该有一个分类映射表
        let (code, name, group) = match category_str {
            "hep-th" => ("hep-th", "High Energy Physics - Theory", "Physics"),
            "hep-ph" => ("hep-ph", "High Energy Physics - Phenomenology", "Physics"),
            "gr-qc" => ("gr-qc", "General Relativity and Quantum Cosmology", "Physics"),
            "cs.AI" => ("cs.AI", "Artificial Intelligence", "Computer Science"),
            "cs.LG" => ("cs.LG", "Machine Learning", "Computer Science"),
            "math.AG" => ("math.AG", "Algebraic Geometry", "Mathematics"),
            _ => (category_str, "Unknown Category", "Unknown"),
        };

        Ok(ArxivCategory {
            code: code.to_string(),
            name: name.to_string(),
            group: group.to_string(),
        })
    }

    /// 解析多个分类
    fn parse_categories(&self, categories: &[String]) -> Result<Vec<ArxivCategory>, MetadataExtractionError> {
        categories.iter()
            .skip(1) // 跳过主分类
            .map(|cat| self.parse_category(cat))
            .collect()
    }

    /// 解析期刊引用
    fn parse_journal_ref(&self, journal_ref: &str) -> JournalReference {
        // 简单的期刊引用解析
        // 实际应用中需要更复杂的解析逻辑
        JournalReference {
            journal_name: journal_ref.to_string(),
            volume: None,
            issue: None,
            pages: None,
            year: None,
        }
    }
}

/// 论文收藏管理服务
pub struct PaperCollectionService<R: PaperRepository> {
    repository: R,
}

impl<R: PaperRepository> PaperCollectionService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// 创建新收藏夹
    pub async fn create_collection(&self, _name: String, _description: Option<String>) -> Result<CollectionId, PaperServiceError> {
        let collection_id = CollectionId {
            value: uuid::Uuid::new_v4().to_string(),
        };
        
        // 这里应该调用收藏夹存储库
        // 暂时简化实现
        Ok(collection_id)
    }

    /// 将论文添加到收藏夹
    pub async fn add_to_collection(&self, paper_id: &PaperId, collection_id: &CollectionId) -> Result<(), PaperServiceError> {
        let mut paper = self.repository.find_by_id(paper_id).await?
            .ok_or_else(|| PaperServiceError::PaperNotFound(paper_id.arxiv_id.clone()))?;

        if !paper.local_state.collections.contains(collection_id) {
            paper.local_state.collections.push(collection_id.clone());
            paper.updated_at = Utc::now();
            self.repository.update(paper).await?;
        }

        Ok(())
    }

    /// 从收藏夹移除论文
    pub async fn remove_from_collection(&self, paper_id: &PaperId, collection_id: &CollectionId) -> Result<(), PaperServiceError> {
        let mut paper = self.repository.find_by_id(paper_id).await?
            .ok_or_else(|| PaperServiceError::PaperNotFound(paper_id.arxiv_id.clone()))?;

        paper.local_state.collections.retain(|c| c != collection_id);
        paper.updated_at = Utc::now();
        self.repository.update(paper).await?;

        Ok(())
    }
}

// 数据传输对象和错误类型

/// ArXiv原始数据
pub struct ArxivData {
    pub id: String,
    pub title: String,
    pub authors: String,
    pub abstract_text: String,
    pub published: chrono::DateTime<Utc>,
    pub updated: chrono::DateTime<Utc>,
    pub primary_category: String,
    pub categories: Vec<String>,
    pub pdf_url: String,
    pub entry_url: String,
    pub doi: Option<String>,
    pub journal_ref: Option<String>,
    pub comments: Option<String>,
}

/// 批量导入结果
pub struct ImportResult {
    pub successful: usize,
    pub skipped: usize,
    pub failed: Vec<ImportFailure>,
}

/// 导入失败信息
pub struct ImportFailure {
    pub paper_id: String,
    pub reason: String,
}

/// 论文服务错误
#[derive(Debug, thiserror::Error)]
pub enum PaperServiceError {
    #[error("Paper already exists: {0}")]
    PaperAlreadyExists(String),
    
    #[error("Paper not found: {0}")]
    PaperNotFound(String),
    
    #[error("Invalid data: {0}")]
    InvalidData(String),
    
    #[error("Repository error: {0}")]
    RepositoryError(String),
}

/// 元数据提取错误
#[derive(Debug, thiserror::Error)]
pub enum MetadataExtractionError {
    #[error("Invalid ArXiv ID format")]
    InvalidArxivId,
    
    #[error("Invalid URL format")]
    InvalidUrl,
    
    #[error("Invalid author data")]
    InvalidAuthorData,
    
    #[error("Invalid category data")]
    InvalidCategoryData,
}

impl From<String> for PaperServiceError {
    fn from(s: String) -> Self {
        PaperServiceError::InvalidData(s)
    }
}

impl From<MetadataExtractionError> for PaperServiceError {
    fn from(e: MetadataExtractionError) -> Self {
        PaperServiceError::InvalidData(e.to_string())
    }
}

impl From<crate::domains::paper::repositories::RepositoryError> for PaperServiceError {
    fn from(e: crate::domains::paper::repositories::RepositoryError) -> Self {
        PaperServiceError::RepositoryError(e.to_string())
    }
}

impl From<String> for MetadataExtractionError {
    fn from(_s: String) -> Self {
        MetadataExtractionError::InvalidAuthorData
    }
}

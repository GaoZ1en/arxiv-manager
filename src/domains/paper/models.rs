// 论文领域模型 - Domain Models for Paper Management
// 基于领域驱动设计原则重新设计的论文实体和值对象

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 论文实体 - Paper Entity
/// 代表一篇学术论文的完整信息和状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Paper {
    // 实体标识
    pub id: PaperId,
    
    // 基础信息
    pub metadata: PaperMetadata,
    
    // 内容信息
    pub content: PaperContent,
    
    // 分类信息
    pub classification: PaperClassification,
    
    // 发布信息
    pub publication: PublicationInfo,
    
    // 本地状态
    pub local_state: LocalPaperState,
    
    // 时间戳
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 论文ID值对象 - Paper ID Value Object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PaperId {
    pub arxiv_id: String,  // 如 "2301.07041"
}

/// 论文元数据值对象 - Paper Metadata Value Object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaperMetadata {
    pub title: String,
    pub authors: Authors,
    pub abstract_text: String,
    pub keywords: Vec<String>,
    pub language: String, // 默认 "en"
}

/// 作者集合值对象 - Authors Value Object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Authors {
    pub primary: Vec<Author>,
    pub affiliations: HashMap<String, String>, // author_name -> affiliation
}

/// 单个作者值对象 - Author Value Object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub orcid: Option<String>, // ORCID ID if available
}

/// 论文内容值对象 - Paper Content Value Object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaperContent {
    pub pdf_url: Url,
    pub entry_url: Url,
    pub source_files: Vec<Url>, // LaTeX source, data files etc.
    pub supplementary: Vec<SupplementaryMaterial>,
}

/// URL值对象 - URL Value Object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Url {
    pub value: String,
}

/// 补充材料值对象 - Supplementary Material Value Object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupplementaryMaterial {
    pub material_type: MaterialType,
    pub url: Url,
    pub description: Option<String>,
}

/// 材料类型枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MaterialType {
    Dataset,
    Code,
    Video,
    Audio,
    Image,
    Document,
    Other(String),
}

/// 论文分类值对象 - Paper Classification Value Object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaperClassification {
    pub primary_category: ArxivCategory,
    pub secondary_categories: Vec<ArxivCategory>,
    pub subject_classes: Vec<SubjectClass>,
    pub msc_classes: Vec<String>, // Mathematics Subject Classification
    pub acm_classes: Vec<String>, // ACM Computing Classification
}

/// ArXiv分类值对象 - ArXiv Category Value Object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArxivCategory {
    pub code: String,    // 如 "hep-th"
    pub name: String,    // 如 "High Energy Physics - Theory"
    pub group: String,   // 如 "Physics"
}

/// 学科分类值对象 - Subject Class Value Object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubjectClass {
    pub code: String,
    pub name: String,
    pub hierarchy: Vec<String>, // 分层结构，如 ["Physics", "Theoretical Physics", "String Theory"]
}

/// 发布信息值对象 - Publication Info Value Object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicationInfo {
    pub published_date: DateTime<Utc>,
    pub updated_date: DateTime<Utc>,
    pub version: u32, // ArXiv版本号
    pub doi: Option<String>,
    pub journal_reference: Option<JournalReference>,
    pub comments: Option<String>,
    pub license: Option<License>,
}

/// 期刊引用值对象 - Journal Reference Value Object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JournalReference {
    pub journal_name: String,
    pub volume: Option<String>,
    pub issue: Option<String>,
    pub pages: Option<String>,
    pub year: Option<u32>,
}

/// 许可证值对象 - License Value Object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct License {
    pub name: String,
    pub url: Option<String>,
}

/// 本地论文状态值对象 - Local Paper State Value Object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalPaperState {
    pub reading_status: ReadingStatus,
    pub rating: Option<Rating>,
    pub notes: Vec<Note>,
    pub tags: Vec<Tag>,
    pub collections: Vec<CollectionId>,
    pub local_files: Vec<LocalFile>,
    pub bookmarks: Vec<Bookmark>,
    pub is_favorite: bool,
}

/// 阅读状态枚举 - Reading Status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReadingStatus {
    Unread,
    Reading { progress: f32 }, // 0.0 to 1.0
    Read,
    Skipped,
    WantToRead,
}

/// 评分值对象 - Rating Value Object (1-5 stars)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rating {
    pub value: u8, // 1-5
    pub created_at: DateTime<Utc>,
}

/// 笔记值对象 - Note Value Object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub content: String,
    pub note_type: NoteType,
    pub page_reference: Option<u32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 笔记类型枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NoteType {
    Summary,
    Question,
    Insight,
    Todo,
    Reference,
    Critique,
}

/// 标签值对象 - Tag Value Object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub color: Option<String>, // 十六进制颜色代码
    pub created_at: DateTime<Utc>,
}

/// 集合ID值对象 - Collection ID Value Object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CollectionId {
    pub value: String,
}

/// 本地文件值对象 - Local File Value Object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalFile {
    pub file_type: LocalFileType,
    pub path: String,
    pub size: u64, // bytes
    pub checksum: String, // SHA-256
    pub created_at: DateTime<Utc>,
}

/// 本地文件类型枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LocalFileType {
    Pdf,
    Source, // LaTeX源文件
    Data,   // 数据文件
    Note,   // 笔记文件
    Annotation, // 注释文件
}

/// 书签值对象 - Bookmark Value Object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: String,
    pub page: u32,
    pub position: Option<BookmarkPosition>, // PDF中的位置
    pub title: String,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// 书签位置值对象 - Bookmark Position Value Object
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BookmarkPosition {
    pub x: f32,
    pub y: f32,
}

// 实现基础方法
impl Paper {
    /// 创建新论文实体
    pub fn new(
        arxiv_id: String,
        metadata: PaperMetadata,
        content: PaperContent,
        classification: PaperClassification,
        publication: PublicationInfo,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: PaperId { arxiv_id },
            metadata,
            content,
            classification,
            publication,
            local_state: LocalPaperState::default(),
            created_at: now,
            updated_at: now,
        }
    }

    /// 更新论文元数据
    pub fn update_metadata(&mut self, metadata: PaperMetadata) {
        self.metadata = metadata;
        self.updated_at = Utc::now();
    }

    /// 添加标签
    pub fn add_tag(&mut self, tag: Tag) {
        if !self.local_state.tags.iter().any(|t| t.name == tag.name) {
            self.local_state.tags.push(tag);
            self.updated_at = Utc::now();
        }
    }

    /// 移除标签
    pub fn remove_tag(&mut self, tag_name: &str) {
        if let Some(pos) = self.local_state.tags.iter().position(|t| t.name == tag_name) {
            self.local_state.tags.remove(pos);
            self.updated_at = Utc::now();
        }
    }

    /// 设置标签
    pub fn set_tags(&mut self, tags: Vec<Tag>) {
        self.local_state.tags = tags;
        self.updated_at = Utc::now();
    }

    /// 更新阅读状态
    pub fn update_reading_status(&mut self, status: ReadingStatus) {
        self.local_state.reading_status = status;
        self.updated_at = Utc::now();
    }

    /// 添加笔记
    pub fn add_note(&mut self, note: Note) {
        self.local_state.notes.push(note);
        self.updated_at = Utc::now();
    }

    /// 设置评分
    pub fn set_rating(&mut self, value: u8) -> Result<(), String> {
        if value < 1 || value > 5 {
            return Err("Rating must be between 1 and 5".to_string());
        }
        self.local_state.rating = Some(Rating {
            value,
            created_at: Utc::now(),
        });
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 设置收藏状态
    pub fn set_favorite(&mut self, is_favorite: bool) {
        self.local_state.is_favorite = is_favorite;
        self.updated_at = Utc::now();
    }

    /// 更新标题
    pub fn update_title(&mut self, title: String) {
        self.metadata.title = title;
        self.updated_at = Utc::now();
    }

    /// 更新作者
    pub fn update_authors(&mut self, authors: Authors) {
        self.metadata.authors = authors;
        self.updated_at = Utc::now();
    }

    /// 更新摘要
    pub fn update_abstract(&mut self, abstract_text: String) {
        self.metadata.abstract_text = abstract_text;
        self.updated_at = Utc::now();
    }

    /// 更新分类
    pub fn update_categories(&mut self, categories: Vec<ArxivCategory>) {
        if let Some(primary) = categories.first() {
            self.classification.primary_category = primary.clone();
            self.classification.secondary_categories = categories.into_iter().skip(1).collect();
        }
        self.updated_at = Utc::now();
    }

    /// 设置更新日期
    pub fn set_updated_date(&mut self, date: DateTime<Utc>) {
        self.publication.updated_date = date;
        self.updated_at = Utc::now();
    }

    /// 设置PDF URL
    pub fn set_pdf_url(&mut self, url: Option<String>) {
        if let Some(url_str) = url {
            if let Ok(url) = Url::new(url_str) {
                self.content.pdf_url = url;
                self.updated_at = Utc::now();
            }
        }
    }

    /// 设置DOI
    pub fn set_doi(&mut self, doi: Option<String>) {
        self.publication.doi = doi;
        self.updated_at = Utc::now();
    }

    /// 设置期刊引用
    pub fn set_journal_ref(&mut self, journal_ref: Option<String>) {
        self.publication.journal_reference = journal_ref.map(|jr| JournalReference {
            journal_name: jr,
            volume: None,
            issue: None,
            pages: None,
            year: None,
        });
        self.updated_at = Utc::now();
    }

    /// 设置评论
    pub fn set_comments(&mut self, comments: Option<String>) {
        self.publication.comments = comments;
        self.updated_at = Utc::now();
    }

    /// 设置阅读状态（替代方法）
    pub fn set_reading_status(&mut self, status: ReadingStatus) {
        self.update_reading_status(status);
    }

    /// 检查是否已读
    pub fn is_read(&self) -> bool {
        matches!(self.local_state.reading_status, ReadingStatus::Read)
    }

    /// 检查是否收藏
    pub fn is_favorite(&self) -> bool {
        self.local_state.is_favorite
    }

    /// 获取主要作者
    pub fn primary_author(&self) -> Option<&Author> {
        self.metadata.authors.primary.first()
    }

    /// 获取主要分类
    pub fn primary_category(&self) -> &ArxivCategory {
        &self.classification.primary_category
    }

    /// 获取ID
    pub fn id(&self) -> &PaperId {
        &self.id
    }

    /// 获取元数据
    pub fn metadata(&self) -> &PaperMetadata {
        &self.metadata
    }

    /// 获取标签
    pub fn tags(&self) -> &Vec<Tag> {
        &self.local_state.tags
    }

    /// 获取评分
    pub fn rating(&self) -> Option<u8> {
        self.local_state.rating.as_ref().map(|r| r.value)
    }

    /// 获取笔记
    pub fn notes(&self) -> &Vec<Note> {
        &self.local_state.notes
    }

    /// 获取阅读状态
    pub fn reading_status(&self) -> &ReadingStatus {
        &self.local_state.reading_status
    }

    /// 获取本地状态
    pub fn local_state(&self) -> &LocalPaperState {
        &self.local_state
    }

    /// 获取本地文件路径
    pub fn local_file_path(&self) -> Option<&str> {
        self.local_state.local_files
            .iter()
            .find(|f| matches!(f.file_type, LocalFileType::Pdf))
            .map(|f| f.path.as_str())
    }

    /// 获取创建时间
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// 获取更新时间
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl Default for LocalPaperState {
    fn default() -> Self {
        Self {
            reading_status: ReadingStatus::Unread,
            rating: None,
            notes: Vec::new(),
            tags: Vec::new(),
            collections: Vec::new(),
            local_files: Vec::new(),
            bookmarks: Vec::new(),
            is_favorite: false,
        }
    }
}

impl Authors {
    /// 从字符串向量创建作者集合
    pub fn from_vec(author_names: Vec<String>) -> Self {
        let authors = author_names.into_iter()
            .map(|name| Author { name, orcid: None })
            .collect();
        
        Self {
            primary: authors,
            affiliations: HashMap::new(),
        }
    }
    
    /// 将作者转换为字符串向量
    pub fn as_vec(&self) -> Vec<String> {
        self.primary.iter()
            .map(|author| author.name.clone())
            .collect()
    }
}

impl PaperId {
    pub fn new(arxiv_id: &str) -> Result<Self, String> {
        // 验证ArXiv ID格式
        if arxiv_id.is_empty() {
            return Err("ArXiv ID cannot be empty".to_string());
        }
        // 这里可以添加更复杂的验证逻辑
        Ok(Self { arxiv_id: arxiv_id.to_string() })
    }

    pub fn as_str(&self) -> &str {
        &self.arxiv_id
    }

    pub fn into_string(self) -> String {
        self.arxiv_id
    }
}

impl ArxivCategory {
    pub fn try_from(code: &str) -> Result<Self, String> {
        // 这里可以实现基于代码的分类映射
        // 暂时返回基础实现
        Ok(Self {
            code: code.to_string(),
            name: format!("Category {}", code),
            group: "Unknown".to_string(),
        })
    }
}

impl std::fmt::Display for ArxivCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl Url {
    pub fn new(value: String) -> Result<Self, String> {
        // 基础URL验证
        if value.is_empty() {
            return Err("URL cannot be empty".to_string());
        }
        if !value.starts_with("http://") && !value.starts_with("https://") {
            return Err("URL must start with http:// or https://".to_string());
        }
        Ok(Self { value })
    }
}

impl From<String> for Tag {
    fn from(name: String) -> Self {
        Self {
            name,
            color: None,
            created_at: Utc::now(),
        }
    }
}

impl From<&str> for Tag {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
            color: None,
            created_at: Utc::now(),
        }
    }
}

// 用例实现 - Use Cases
// 实现应用程序的核心业务用例，协调领域服务和基础设施

use std::sync::Arc;
use chrono::{DateTime, Utc};

// Temporarily comment out to check for circular dependencies
// use crate::domains::paper::*;
use crate::domains::paper::{
    PaperRepository, PaperQueryRepository, PaperEventPublisher, PaperEventBuilder,
    Paper, PaperMetadata, PaperContent, PaperClassification, PublicationInfo,
    Authors, ArxivCategory, Url, Tag, JournalReference,
    PaperId
};

use crate::application::{
    commands::*, queries::*, dto::*,
    ApplicationResult, ApplicationError
};

/// 论文管理用例
pub struct PaperManagementUseCase {
    #[allow(dead_code)]
    paper_repository: Arc<dyn PaperRepository>,
    paper_query_repository: Arc<dyn PaperQueryRepository>,
    #[allow(dead_code)]
    event_publisher: Arc<tokio::sync::Mutex<PaperEventPublisher>>,
}

impl PaperManagementUseCase {
    /// 创建新的论文管理用例
    pub fn new(
        paper_repository: Arc<dyn PaperRepository>,
        paper_query_repository: Arc<dyn PaperQueryRepository>,
        event_publisher: Arc<tokio::sync::Mutex<PaperEventPublisher>>,
    ) -> Self {
        Self {
            paper_repository,
            paper_query_repository,
            event_publisher,
        }
    }
    
    /// 创建论文
    pub async fn create_paper(&self, command: CreatePaperCommand) -> ApplicationResult<String> {
        // 验证命令
        command.validate()
            .map_err(|e| ApplicationError::new("VALIDATION_ERROR", e))?;
        
        // 构建论文元数据
        let metadata = PaperMetadata {
            title: command.title,
            authors: Authors::from_vec(command.authors),
            abstract_text: command.abstract_text,
            keywords: Vec::new(),
            language: "en".to_string(),
        };

        // 构建内容信息
        let content = PaperContent {
            pdf_url: Url::new(command.pdf_url.unwrap_or_else(|| command.arxiv_url.clone()))
                .map_err(|e| ApplicationError::new("INVALID_URL", e))?,
            entry_url: Url::new(command.arxiv_url.clone())
                .map_err(|e| ApplicationError::new("INVALID_URL", e))?,
            source_files: Vec::new(),
            supplementary: Vec::new(),
        };

        // 构建分类信息
        let classification = PaperClassification {
            primary_category: ArxivCategory::try_from(command.categories.first()
                .ok_or_else(|| ApplicationError::new("INVALID_CATEGORY", "No categories provided".to_string()))?
                .as_str())
                .map_err(|e| ApplicationError::new("INVALID_CATEGORY", e))?,
            secondary_categories: command.categories.iter().skip(1)
                .map(|c| ArxivCategory::try_from(c.as_str()))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| ApplicationError::new("INVALID_CATEGORY", e))?,
            subject_classes: Vec::new(),
            msc_classes: Vec::new(),
            acm_classes: Vec::new(),
        };

        // 构建发布信息
        let publication = PublicationInfo {
            published_date: command.published_date,
            updated_date: command.updated_date.unwrap_or(command.published_date),
            version: 1, // 默认版本
            doi: command.doi,
            journal_reference: command.journal_ref.map(|jr| JournalReference {
                journal_name: jr,
                volume: None,
                issue: None,
                pages: None,
                year: None,
            }),
            comments: command.comments,
            license: None,
        };
        
        // 创建论文实体
        let paper_id = PaperId::new(&command.arxiv_id)
            .map_err(|e| ApplicationError::new("INVALID_PAPER_ID", e))?;
        
        let paper = Paper::new(command.arxiv_id, metadata.clone(), content, classification, publication);
        
        // 检查是否已存在
        if self.paper_repository.exists(&paper_id).await
            .map_err(|e| ApplicationError::new("REPOSITORY_ERROR", e.to_string()))? 
        {
            return Err(ApplicationError::new("PAPER_ALREADY_EXISTS", 
                format!("Paper with ID {} already exists", paper_id.as_str())));
        }
        
        // 保存论文
        self.paper_repository.save(paper).await
            .map_err(|e| ApplicationError::new("REPOSITORY_ERROR", e.to_string()))?;
        
        // 发布事件
        let event = PaperEventBuilder::new()
            .paper_created(paper_id.clone(), metadata);
        
        if let Ok(mut publisher) = self.event_publisher.try_lock() {
            if let Err(e) = publisher.publish(event) {
                log::warn!("Failed to publish paper created event: {}", e);
            }
        }
        
        Ok(paper_id.into_string())
    }
    
    /// 更新论文元数据
    pub async fn update_paper_metadata(&self, command: UpdatePaperMetadataCommand) -> ApplicationResult<()> {
        // 验证命令
        command.validate()
            .map_err(|e| ApplicationError::new("VALIDATION_ERROR", e))?;
        
        let paper_id = PaperId::new(&command.paper_id)
            .map_err(|e| ApplicationError::new("INVALID_PAPER_ID", e))?;
        
        // 获取现有论文
        let mut paper = self.paper_repository.find_by_id(&paper_id).await
            .map_err(|e| ApplicationError::new("REPOSITORY_ERROR", e.to_string()))?
            .ok_or_else(|| ApplicationError::new("PAPER_NOT_FOUND", 
                format!("Paper with ID {} not found", paper_id.as_str())))?;
        
        let old_metadata = paper.metadata().clone();
        
        // 更新字段
        if let Some(title) = command.title {
            paper.update_title(title);
        }
        if let Some(authors) = command.authors {
            paper.update_authors(Authors::from_vec(authors));
        }
        if let Some(abstract_text) = command.abstract_text {
            paper.update_abstract(abstract_text);
        }
        if let Some(categories) = command.categories {
            let categories = categories.into_iter()
                .map(|c| ArxivCategory::try_from(c.as_str()))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| ApplicationError::new("INVALID_CATEGORY", e))?;
            paper.update_categories(categories);
        }
        if let Some(updated_date) = command.updated_date {
            paper.set_updated_date(updated_date);
        }
        if let Some(pdf_url) = command.pdf_url {
            paper.set_pdf_url(Some(pdf_url));
        }
        if let Some(doi) = command.doi {
            paper.set_doi(Some(doi));
        }
        if let Some(journal_ref) = command.journal_ref {
            paper.set_journal_ref(Some(journal_ref));
        }
        if let Some(comments) = command.comments {
            paper.set_comments(Some(comments));
        }
        
        // 在保存前获取新的元数据
        let new_metadata = paper.metadata().clone();
        
        // 保存更新
        self.paper_repository.save(paper).await
            .map_err(|e| ApplicationError::new("REPOSITORY_ERROR", e.to_string()))?;
        
        // 发布事件
        let event = PaperEventBuilder::new()
            .metadata_updated(paper_id, old_metadata, new_metadata);
        
        if let Ok(mut publisher) = self.event_publisher.try_lock() {
            if let Err(e) = publisher.publish(event) {
                log::warn!("Failed to publish metadata updated event: {}", e);
            }
        }
        
        Ok(())
    }
    
    /// 更新阅读状态
    pub async fn update_reading_status(&self, command: UpdateReadingStatusCommand) -> ApplicationResult<()> {
        command.validate()
            .map_err(|e| ApplicationError::new("VALIDATION_ERROR", e))?;
        
        let paper_id = PaperId::new(&command.paper_id)
            .map_err(|e| ApplicationError::new("INVALID_PAPER_ID", e))?;
        
        let mut paper = self.paper_repository.find_by_id(&paper_id).await
            .map_err(|e| ApplicationError::new("REPOSITORY_ERROR", e.to_string()))?
            .ok_or_else(|| ApplicationError::new("PAPER_NOT_FOUND", 
                format!("Paper with ID {} not found", paper_id.as_str())))?;
        
        let old_status = paper.reading_status().clone();
        
        if old_status != command.reading_status {
            paper.set_reading_status(command.reading_status.clone());
            
            self.paper_repository.save(paper).await
                .map_err(|e| ApplicationError::new("REPOSITORY_ERROR", e.to_string()))?;
            
            // 发布事件
            let event = PaperEventBuilder::new()
                .reading_status_changed(paper_id, old_status, command.reading_status);
            
            if let Ok(mut publisher) = self.event_publisher.try_lock() {
                if let Err(e) = publisher.publish(event) {
                    log::warn!("Failed to publish reading status changed event: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    /// 更新论文标签
    pub async fn update_paper_tags(&self, command: UpdatePaperTagsCommand) -> ApplicationResult<()> {
        command.validate()
            .map_err(|e| ApplicationError::new("VALIDATION_ERROR", e))?;
        
        let paper_id = PaperId::new(&command.paper_id)
            .map_err(|e| ApplicationError::new("INVALID_PAPER_ID", e))?;
        
        let mut paper = self.paper_repository.find_by_id(&paper_id).await
            .map_err(|e| ApplicationError::new("REPOSITORY_ERROR", e.to_string()))?
            .ok_or_else(|| ApplicationError::new("PAPER_NOT_FOUND", 
                format!("Paper with ID {} not found", paper_id.as_str())))?;
        
        let old_tags = paper.tags().iter().map(|t| t.name.clone()).collect::<Vec<String>>();
        
        match command.operation {
            TagOperation::Set => {
                let tags = command.tags.into_iter()
                    .map(|name| Tag::from(name))
                    .collect();
                paper.set_tags(tags);
            }
            TagOperation::Add => {
                for tag_name in command.tags {
                    paper.add_tag(Tag::from(tag_name));
                }
            }
            TagOperation::Remove => {
                for tag_name in command.tags {
                    paper.remove_tag(&tag_name);
                }
            }
        }
        
        // 在保存前获取新标签
        let new_tags = paper.tags().iter().map(|t| t.name.clone()).collect::<Vec<String>>();
        
        self.paper_repository.save(paper).await
            .map_err(|e| ApplicationError::new("REPOSITORY_ERROR", e.to_string()))?;
        
        // 发布事件
        let event = PaperEventBuilder::new()
            .tags_updated(paper_id, old_tags, new_tags);
        
        if let Ok(mut publisher) = self.event_publisher.try_lock() {
            if let Err(e) = publisher.publish(event) {
                log::warn!("Failed to publish tags updated event: {}", e);
            }
        }
        
        Ok(())
    }
    
    /// 更新论文评分
    pub async fn update_paper_rating(&self, command: UpdatePaperRatingCommand) -> ApplicationResult<()> {
        command.validate()
            .map_err(|e| ApplicationError::new("VALIDATION_ERROR", e))?;
        
        let paper_id = PaperId::new(&command.paper_id)
            .map_err(|e| ApplicationError::new("INVALID_PAPER_ID", e))?;
        
        let mut paper = self.paper_repository.find_by_id(&paper_id).await
            .map_err(|e| ApplicationError::new("REPOSITORY_ERROR", e.to_string()))?
            .ok_or_else(|| ApplicationError::new("PAPER_NOT_FOUND", 
                format!("Paper with ID {} not found", paper_id.as_str())))?;
        
        let old_rating = paper.rating();
        
        if let Some(rating) = command.rating {
            paper.set_rating(rating)
                .map_err(|e| ApplicationError::new("INVALID_RATING", e))?;
        }
        
        self.paper_repository.save(paper).await
            .map_err(|e| ApplicationError::new("REPOSITORY_ERROR", e.to_string()))?;
        
        // 发布事件
        let event = PaperEventBuilder::new()
            .rating_updated(paper_id, old_rating, command.rating);
        
        if let Ok(mut publisher) = self.event_publisher.try_lock() {
            if let Err(e) = publisher.publish(event) {
                log::warn!("Failed to publish rating updated event: {}", e);
            }
        }
        
        Ok(())
    }
    
    /// 设置收藏状态
    pub async fn set_favorite(&self, command: SetFavoriteCommand) -> ApplicationResult<()> {
        command.validate()
            .map_err(|e| ApplicationError::new("VALIDATION_ERROR", e))?;
        
        let paper_id = PaperId::new(&command.paper_id)
            .map_err(|e| ApplicationError::new("INVALID_PAPER_ID", e))?;
        
        let mut paper = self.paper_repository.find_by_id(&paper_id).await
            .map_err(|e| ApplicationError::new("REPOSITORY_ERROR", e.to_string()))?
            .ok_or_else(|| ApplicationError::new("PAPER_NOT_FOUND", 
                format!("Paper with ID {} not found", paper_id.as_str())))?;
        
        if paper.is_favorite() != command.is_favorite {
            paper.set_favorite(command.is_favorite);
            
            self.paper_repository.save(paper).await
                .map_err(|e| ApplicationError::new("REPOSITORY_ERROR", e.to_string()))?;
            
            // 发布事件
            let event = PaperEventBuilder::new()
                .favorite_status_changed(paper_id, command.is_favorite);
            
            if let Ok(mut publisher) = self.event_publisher.try_lock() {
                if let Err(e) = publisher.publish(event) {
                    log::warn!("Failed to publish favorite status changed event: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    /// 删除论文
    pub async fn delete_paper(&self, command: DeletePaperCommand) -> ApplicationResult<()> {
        command.validate()
            .map_err(|e| ApplicationError::new("VALIDATION_ERROR", e))?;
        
        let paper_id = PaperId::new(&command.paper_id)
            .map_err(|e| ApplicationError::new("INVALID_PAPER_ID", e))?;
        
        // 获取论文信息用于事件
        let paper = self.paper_repository.find_by_id(&paper_id).await
            .map_err(|e| ApplicationError::new("REPOSITORY_ERROR", e.to_string()))?
            .ok_or_else(|| ApplicationError::new("PAPER_NOT_FOUND", 
                format!("Paper with ID {} not found", paper_id.as_str())))?;
        
        let final_metadata = paper.metadata().clone();
        
        // 删除本地文件（如果存在）
        if let Some(file_path) = paper.local_file_path() {
            if command.delete_local_file {
                if let Err(e) = tokio::fs::remove_file(file_path).await {
                    log::warn!("Failed to delete local file {}: {}", file_path, e);
                }
            }
        }
        
        // 删除论文记录
        self.paper_repository.delete(&paper_id).await
            .map_err(|e| ApplicationError::new("REPOSITORY_ERROR", e.to_string()))?;
        
        // 发布事件
        let event = PaperEventBuilder::new()
            .paper_deleted(paper_id, final_metadata);
        
        if let Ok(mut publisher) = self.event_publisher.try_lock() {
            if let Err(e) = publisher.publish(event) {
                log::warn!("Failed to publish paper deleted event: {}", e);
            }
        }
        
        Ok(())
    }
    
    /// 批量更新阅读状态
    pub async fn batch_update_reading_status(&self, command: BatchUpdateReadingStatusCommand) -> ApplicationResult<Vec<String>> {
        command.validate()
            .map_err(|e| ApplicationError::new("VALIDATION_ERROR", e))?;
        
        let mut updated_ids = Vec::new();
        
        for paper_id_str in command.paper_ids {
            let update_command = UpdateReadingStatusCommand {
                paper_id: paper_id_str.clone(),
                reading_status: command.reading_status.clone(),
            };
            
            match self.update_reading_status(update_command).await {
                Ok(_) => updated_ids.push(paper_id_str),
                Err(e) => {
                    log::warn!("Failed to update reading status for paper {}: {}", paper_id_str, e);
                }
            }
        }
        
        Ok(updated_ids)
    }
}

/// 论文查询用例
pub struct PaperQueryUseCase {
    paper_query_repository: Arc<dyn PaperQueryRepository>,
}

impl PaperQueryUseCase {
    pub fn new(paper_query_repository: Arc<dyn PaperQueryRepository>) -> Self {
        Self {
            paper_query_repository,
        }
    }
    
    /// 根据ID获取论文
    pub async fn get_paper_by_id(&self, query: GetPaperByIdQuery) -> ApplicationResult<Option<PaperResponse>> {
        query.validate()
            .map_err(|e| ApplicationError::new("VALIDATION_ERROR", e))?;
        
        let paper_id = PaperId::new(&query.paper_id)
            .map_err(|e| ApplicationError::new("INVALID_PAPER_ID", e))?;
        
        let paper = self.paper_query_repository.find_by_id(&paper_id).await
            .map_err(|e| ApplicationError::new("REPOSITORY_ERROR", e.to_string()))?;
        
        Ok(paper.map(|p| self.paper_to_response(&p)))
    }
    
    /// 搜索论文
    pub async fn search_papers(&self, query: SearchPapersQuery) -> ApplicationResult<PaperListResponse> {
        query.validate()
            .map_err(|e| ApplicationError::new("VALIDATION_ERROR", e))?;
        
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(20);
        let offset = (page - 1) * page_size;
        
        let (papers, total_count) = self.paper_query_repository.search_papers(
                query.query.as_deref(),
                query.authors.as_ref().map(|authors| authors.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(",")).as_deref(),
                query.categories.as_ref().map(|cats| cats.iter().map(|c| c.code.as_str()).collect::<Vec<_>>().join(",")).as_deref(),
                query.date_from,
                query.date_to,
                query.reading_status,
                query.tags.as_ref().map(|tags| tags.join(",")).as_deref(),
                query.rating_min,
                query.rating_max,
                query.is_favorite,
                None, // 简化 local_state 处理
                offset,
                page_size,
            ).await
            .map_err(|e| ApplicationError::new("REPOSITORY_ERROR", e.to_string()))?;
        
        let paper_responses: Vec<PaperResponse> = papers.into_iter()
            .map(|p| self.paper_to_response(&p))
            .collect();
        
        let total_pages = (total_count as f64 / page_size as f64).ceil() as u32;
        
        Ok(PaperListResponse {
            papers: paper_responses,
            total_count,
            page,
            page_size,
            total_pages,
        })
    }
    
    /// 获取论文统计信息
    pub async fn get_paper_stats(&self, query: GetPaperStatsQuery) -> ApplicationResult<PaperStatsResponse> {
        query.validate()
            .map_err(|e| ApplicationError::new("VALIDATION_ERROR", e))?;
        
        let stats = self.paper_query_repository.get_statistics(
            query.date_from,
            query.date_to,
            query.categories.as_ref().map(|cats| cats.iter().map(|c| c.code.as_str()).collect::<Vec<_>>().join(",")).as_deref(),
        ).await
        .map_err(|e| ApplicationError::new("REPOSITORY_ERROR", e.to_string()))?;
        
        Ok(stats)
    }
    
    /// 将 Paper 实体转换为响应 DTO
    fn paper_to_response(&self, paper: &Paper) -> PaperResponse {
        let metadata = paper.metadata();
        
        PaperResponse {
            id: paper.id().arxiv_id.clone(),
            title: metadata.title.clone(),
            authors: metadata.authors.primary.iter().map(|a| a.name.clone()).collect(),
            abstract_text: metadata.abstract_text.clone(),
            categories: vec![paper.classification.primary_category.code.clone()], // 简化处理
            published_date: paper.publication.published_date,
            updated_date: Some(paper.publication.updated_date),
            pdf_url: Some(paper.content.pdf_url.value.clone()),
            arxiv_url: paper.content.entry_url.value.clone(),
            doi: paper.publication.doi.clone(),
            journal_ref: paper.publication.journal_reference.as_ref().map(|jr| jr.journal_name.clone()),
            comments: paper.publication.comments.clone(),
            reading_status: paper.reading_status().clone(),
            tags: paper.tags().iter().map(|t| t.name.clone()).collect(),
            rating: paper.rating(),
            notes: Some(format!("{} notes", paper.notes().len())), // 简化处理
            is_favorite: paper.is_favorite(),
            local_state: paper.local_state().clone(),
            local_file_path: paper.local_file_path().map(|s| s.to_string()),
            created_at: paper.created_at(),
            updated_at: paper.updated_at(),
        }
    }
}

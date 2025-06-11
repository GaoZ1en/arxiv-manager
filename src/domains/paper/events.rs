// 论文领域事件 - Paper Domain Events
// 实现论文生命周期中的重要事件，支持事件驱动架构

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

use super::models::*;

/// 论文领域事件的基础特征
pub trait DomainEvent {
    /// 事件发生的时间戳
    fn occurred_at(&self) -> DateTime<Utc>;
    
    /// 事件的唯一标识符
    fn event_id(&self) -> String;
    
    /// 事件类型名称
    fn event_type(&self) -> &'static str;
    
    /// 相关的论文ID
    fn paper_id(&self) -> &PaperId;
}

/// 论文生命周期事件
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PaperEvent {
    /// 论文已创建
    PaperCreated {
        paper_id: PaperId,
        metadata: PaperMetadata,
        occurred_at: DateTime<Utc>,
        event_id: String,
    },
    
    /// 论文元数据已更新
    MetadataUpdated {
        paper_id: PaperId,
        old_metadata: PaperMetadata,
        new_metadata: PaperMetadata,
        occurred_at: DateTime<Utc>,
        event_id: String,
    },
    
    /// 论文阅读状态已改变
    ReadingStatusChanged {
        paper_id: PaperId,
        old_status: ReadingStatus,
        new_status: ReadingStatus,
        occurred_at: DateTime<Utc>,
        event_id: String,
    },
    
    /// 论文标签已更新
    TagsUpdated {
        paper_id: PaperId,
        old_tags: Vec<String>,
        new_tags: Vec<String>,
        occurred_at: DateTime<Utc>,
        event_id: String,
    },
    
    /// 论文评分已更新
    RatingUpdated {
        paper_id: PaperId,
        old_rating: Option<u8>,
        new_rating: Option<u8>,
        occurred_at: DateTime<Utc>,
        event_id: String,
    },
    
    /// 论文笔记已更新
    NotesUpdated {
        paper_id: PaperId,
        notes_added: bool, // true表示添加，false表示删除
        occurred_at: DateTime<Utc>,
        event_id: String,
    },
    
    /// 论文本地文件状态已更新
    LocalFileUpdated {
        paper_id: PaperId,
        old_state: LocalPaperState,
        new_state: LocalPaperState,
        file_path: Option<String>,
        occurred_at: DateTime<Utc>,
        event_id: String,
    },
    
    /// 论文已删除
    PaperDeleted {
        paper_id: PaperId,
        final_metadata: PaperMetadata,
        occurred_at: DateTime<Utc>,
        event_id: String,
    },
    
    /// 论文收藏状态已改变
    FavoriteStatusChanged {
        paper_id: PaperId,
        is_favorite: bool,
        occurred_at: DateTime<Utc>,
        event_id: String,
    },
    
    /// 论文下载开始
    DownloadStarted {
        paper_id: PaperId,
        download_url: String,
        occurred_at: DateTime<Utc>,
        event_id: String,
    },
    
    /// 论文下载完成
    DownloadCompleted {
        paper_id: PaperId,
        file_path: String,
        file_size: u64,
        occurred_at: DateTime<Utc>,
        event_id: String,
    },
    
    /// 论文下载失败
    DownloadFailed {
        paper_id: PaperId,
        error_message: String,
        occurred_at: DateTime<Utc>,
        event_id: String,
    },
}

impl DomainEvent for PaperEvent {
    fn occurred_at(&self) -> DateTime<Utc> {
        match self {
            Self::PaperCreated { occurred_at, .. } => *occurred_at,
            Self::MetadataUpdated { occurred_at, .. } => *occurred_at,
            Self::ReadingStatusChanged { occurred_at, .. } => *occurred_at,
            Self::TagsUpdated { occurred_at, .. } => *occurred_at,
            Self::RatingUpdated { occurred_at, .. } => *occurred_at,
            Self::NotesUpdated { occurred_at, .. } => *occurred_at,
            Self::LocalFileUpdated { occurred_at, .. } => *occurred_at,
            Self::PaperDeleted { occurred_at, .. } => *occurred_at,
            Self::FavoriteStatusChanged { occurred_at, .. } => *occurred_at,
            Self::DownloadStarted { occurred_at, .. } => *occurred_at,
            Self::DownloadCompleted { occurred_at, .. } => *occurred_at,
            Self::DownloadFailed { occurred_at, .. } => *occurred_at,
        }
    }
    
    fn event_id(&self) -> String {
        match self {
            Self::PaperCreated { event_id, .. } => event_id.clone(),
            Self::MetadataUpdated { event_id, .. } => event_id.clone(),
            Self::ReadingStatusChanged { event_id, .. } => event_id.clone(),
            Self::TagsUpdated { event_id, .. } => event_id.clone(),
            Self::RatingUpdated { event_id, .. } => event_id.clone(),
            Self::NotesUpdated { event_id, .. } => event_id.clone(),
            Self::LocalFileUpdated { event_id, .. } => event_id.clone(),
            Self::PaperDeleted { event_id, .. } => event_id.clone(),
            Self::FavoriteStatusChanged { event_id, .. } => event_id.clone(),
            Self::DownloadStarted { event_id, .. } => event_id.clone(),
            Self::DownloadCompleted { event_id, .. } => event_id.clone(),
            Self::DownloadFailed { event_id, .. } => event_id.clone(),
        }
    }
    
    fn event_type(&self) -> &'static str {
        match self {
            Self::PaperCreated { .. } => "PaperCreated",
            Self::MetadataUpdated { .. } => "MetadataUpdated",
            Self::ReadingStatusChanged { .. } => "ReadingStatusChanged",
            Self::TagsUpdated { .. } => "TagsUpdated",
            Self::RatingUpdated { .. } => "RatingUpdated",
            Self::NotesUpdated { .. } => "NotesUpdated",
            Self::LocalFileUpdated { .. } => "LocalFileUpdated",
            Self::PaperDeleted { .. } => "PaperDeleted",
            Self::FavoriteStatusChanged { .. } => "FavoriteStatusChanged",
            Self::DownloadStarted { .. } => "DownloadStarted",
            Self::DownloadCompleted { .. } => "DownloadCompleted",
            Self::DownloadFailed { .. } => "DownloadFailed",
        }
    }
    
    fn paper_id(&self) -> &PaperId {
        match self {
            Self::PaperCreated { paper_id, .. } => paper_id,
            Self::MetadataUpdated { paper_id, .. } => paper_id,
            Self::ReadingStatusChanged { paper_id, .. } => paper_id,
            Self::TagsUpdated { paper_id, .. } => paper_id,
            Self::RatingUpdated { paper_id, .. } => paper_id,
            Self::NotesUpdated { paper_id, .. } => paper_id,
            Self::LocalFileUpdated { paper_id, .. } => paper_id,
            Self::PaperDeleted { paper_id, .. } => paper_id,
            Self::FavoriteStatusChanged { paper_id, .. } => paper_id,
            Self::DownloadStarted { paper_id, .. } => paper_id,
            Self::DownloadCompleted { paper_id, .. } => paper_id,
            Self::DownloadFailed { paper_id, .. } => paper_id,
        }
    }
}

impl fmt::Display for PaperEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PaperCreated { paper_id, .. } => {
                write!(f, "Paper {} created", paper_id.as_str())
            }
            Self::MetadataUpdated { paper_id, .. } => {
                write!(f, "Paper {} metadata updated", paper_id.as_str())
            }
            Self::ReadingStatusChanged { paper_id, new_status, .. } => {
                write!(f, "Paper {} reading status changed to {:?}", paper_id.as_str(), new_status)
            }
            Self::TagsUpdated { paper_id, .. } => {
                write!(f, "Paper {} tags updated", paper_id.as_str())
            }
            Self::RatingUpdated { paper_id, new_rating, .. } => {
                write!(f, "Paper {} rating updated to {:?}", paper_id.as_str(), new_rating)
            }
            Self::NotesUpdated { paper_id, notes_added, .. } => {
                let action = if *notes_added { "added" } else { "removed" };
                write!(f, "Paper {} notes {}", paper_id.as_str(), action)
            }
            Self::LocalFileUpdated { paper_id, new_state, .. } => {
                write!(f, "Paper {} local file state updated to {:?}", paper_id.as_str(), new_state)
            }
            Self::PaperDeleted { paper_id, .. } => {
                write!(f, "Paper {} deleted", paper_id.as_str())
            }
            Self::FavoriteStatusChanged { paper_id, is_favorite, .. } => {
                let status = if *is_favorite { "favorited" } else { "unfavorited" };
                write!(f, "Paper {} {}", paper_id.as_str(), status)
            }
            Self::DownloadStarted { paper_id, .. } => {
                write!(f, "Paper {} download started", paper_id.as_str())
            }
            Self::DownloadCompleted { paper_id, .. } => {
                write!(f, "Paper {} download completed", paper_id.as_str())
            }
            Self::DownloadFailed { paper_id, error_message, .. } => {
                write!(f, "Paper {} download failed: {}", paper_id.as_str(), error_message)
            }
        }
    }
}

/// 论文事件构建器
#[derive(Debug, Default)]
pub struct PaperEventBuilder {
    occurred_at: Option<DateTime<Utc>>,
    event_id: Option<String>,
}

impl PaperEventBuilder {
    /// 创建新的事件构建器
    pub fn new() -> Self {
        Self {
            occurred_at: Some(Utc::now()),
            event_id: Some(uuid::Uuid::new_v4().to_string()),
        }
    }
    
    /// 设置事件发生时间
    pub fn occurred_at(mut self, time: DateTime<Utc>) -> Self {
        self.occurred_at = Some(time);
        self
    }
    
    /// 设置事件ID
    pub fn event_id(mut self, id: String) -> Self {
        self.event_id = Some(id);
        self
    }
    
    /// 构建论文创建事件
    pub fn paper_created(self, paper_id: PaperId, metadata: PaperMetadata) -> PaperEvent {
        PaperEvent::PaperCreated {
            paper_id,
            metadata,
            occurred_at: self.occurred_at.unwrap_or_else(Utc::now),
            event_id: self.event_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
        }
    }
    
    /// 构建元数据更新事件
    pub fn metadata_updated(
        self,
        paper_id: PaperId,
        old_metadata: PaperMetadata,
        new_metadata: PaperMetadata,
    ) -> PaperEvent {
        PaperEvent::MetadataUpdated {
            paper_id,
            old_metadata,
            new_metadata,
            occurred_at: self.occurred_at.unwrap_or_else(Utc::now),
            event_id: self.event_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
        }
    }
    
    /// 构建阅读状态改变事件
    pub fn reading_status_changed(
        self,
        paper_id: PaperId,
        old_status: ReadingStatus,
        new_status: ReadingStatus,
    ) -> PaperEvent {
        PaperEvent::ReadingStatusChanged {
            paper_id,
            old_status,
            new_status,
            occurred_at: self.occurred_at.unwrap_or_else(Utc::now),
            event_id: self.event_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
        }
    }
    
    /// 构建标签更新事件
    pub fn tags_updated(
        self,
        paper_id: PaperId,
        old_tags: Vec<String>,
        new_tags: Vec<String>,
    ) -> PaperEvent {
        PaperEvent::TagsUpdated {
            paper_id,
            old_tags,
            new_tags,
            occurred_at: self.occurred_at.unwrap_or_else(Utc::now),
            event_id: self.event_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
        }
    }
    
    /// 构建评分更新事件
    pub fn rating_updated(
        self,
        paper_id: PaperId,
        old_rating: Option<u8>,
        new_rating: Option<u8>,
    ) -> PaperEvent {
        PaperEvent::RatingUpdated {
            paper_id,
            old_rating,
            new_rating,
            occurred_at: self.occurred_at.unwrap_or_else(Utc::now),
            event_id: self.event_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
        }
    }
    
    /// 构建笔记更新事件
    pub fn notes_updated(
        self,
        paper_id: PaperId,
        notes_added: bool,
    ) -> PaperEvent {
        PaperEvent::NotesUpdated {
            paper_id,
            notes_added,
            occurred_at: self.occurred_at.unwrap_or_else(Utc::now),
            event_id: self.event_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
        }
    }
    
    /// 构建本地文件更新事件
    pub fn local_file_updated(
        self,
        paper_id: PaperId,
        old_state: LocalPaperState,
        new_state: LocalPaperState,
        file_path: Option<String>,
    ) -> PaperEvent {
        PaperEvent::LocalFileUpdated {
            paper_id,
            old_state,
            new_state,
            file_path,
            occurred_at: self.occurred_at.unwrap_or_else(Utc::now),
            event_id: self.event_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
        }
    }
    
    /// 构建论文删除事件
    pub fn paper_deleted(
        self,
        paper_id: PaperId,
        final_metadata: PaperMetadata,
    ) -> PaperEvent {
        PaperEvent::PaperDeleted {
            paper_id,
            final_metadata,
            occurred_at: self.occurred_at.unwrap_or_else(Utc::now),
            event_id: self.event_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
        }
    }
    
    /// 构建收藏状态改变事件
    pub fn favorite_status_changed(
        self,
        paper_id: PaperId,
        is_favorite: bool,
    ) -> PaperEvent {
        PaperEvent::FavoriteStatusChanged {
            paper_id,
            is_favorite,
            occurred_at: self.occurred_at.unwrap_or_else(Utc::now),
            event_id: self.event_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
        }
    }
    
    /// 构建下载开始事件
    pub fn download_started(
        self,
        paper_id: PaperId,
        download_url: String,
    ) -> PaperEvent {
        PaperEvent::DownloadStarted {
            paper_id,
            download_url,
            occurred_at: self.occurred_at.unwrap_or_else(Utc::now),
            event_id: self.event_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
        }
    }
    
    /// 构建下载完成事件
    pub fn download_completed(
        self,
        paper_id: PaperId,
        file_path: String,
        file_size: u64,
    ) -> PaperEvent {
        PaperEvent::DownloadCompleted {
            paper_id,
            file_path,
            file_size,
            occurred_at: self.occurred_at.unwrap_or_else(Utc::now),
            event_id: self.event_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
        }
    }
    
    /// 构建下载失败事件
    pub fn download_failed(
        self,
        paper_id: PaperId,
        error_message: String,
    ) -> PaperEvent {
        PaperEvent::DownloadFailed {
            paper_id,
            error_message,
            occurred_at: self.occurred_at.unwrap_or_else(Utc::now),
            event_id: self.event_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
        }
    }
}

/// 论文事件处理器特征
pub trait PaperEventHandler: Send + Sync + std::fmt::Debug {
    /// 处理论文事件
    fn handle_event(&mut self, event: &PaperEvent) -> Result<(), Box<dyn std::error::Error>>;
}

/// 论文事件存储接口
pub trait PaperEventStore: Send + Sync + std::fmt::Debug {
    /// 保存事件
    fn save_event(&mut self, event: &PaperEvent) -> Result<(), Box<dyn std::error::Error>>;
    
    /// 获取论文的所有事件
    fn get_events_for_paper(&self, paper_id: &PaperId) -> Result<Vec<PaperEvent>, Box<dyn std::error::Error>>;
    
    /// 获取指定时间范围内的事件
    fn get_events_in_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<PaperEvent>, Box<dyn std::error::Error>>;
    
    /// 获取指定类型的事件
    fn get_events_by_type(&self, event_type: &str) -> Result<Vec<PaperEvent>, Box<dyn std::error::Error>>;
}

/// 论文事件发布器
#[derive(Debug)]
pub struct PaperEventPublisher {
    handlers: Vec<Box<dyn PaperEventHandler>>,
    event_store: Option<Box<dyn PaperEventStore>>,
}

impl PaperEventPublisher {
    /// 创建新的事件发布器
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            event_store: None,
        }
    }
    
    /// 添加事件处理器
    pub fn add_handler(&mut self, handler: Box<dyn PaperEventHandler>) {
        self.handlers.push(handler);
    }
    
    /// 设置事件存储
    pub fn set_event_store(&mut self, store: Box<dyn PaperEventStore>) {
        self.event_store = Some(store);
    }
    
    /// 发布事件
    pub fn publish(&mut self, event: PaperEvent) -> Result<(), Box<dyn std::error::Error>> {
        // 保存事件到存储
        if let Some(store) = &mut self.event_store {
            store.save_event(&event)?;
        }
        
        // 通知所有处理器
        for handler in &mut self.handlers {
            if let Err(e) = handler.handle_event(&event) {
                eprintln!("Error handling event {}: {}", event.event_id(), e);
            }
        }
        
        Ok(())
    }
}

impl Default for PaperEventPublisher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domains::paper::models::Authors;

    #[test]
    fn test_paper_event_creation() {
        let paper_id = PaperId::new("2301.12345").unwrap();
        let metadata = PaperMetadata {
            title: "Test Paper".to_string(),
            authors: Authors::from_vec(vec!["Author 1".to_string()]),
            abstract_text: "Test abstract".to_string(),
            keywords: vec!["test".to_string(), "paper".to_string()],
            language: "en".to_string(),
        };
        
        let event = PaperEventBuilder::new()
            .paper_created(paper_id.clone(), metadata.clone());
        
        assert_eq!(event.paper_id(), &paper_id);
        assert_eq!(event.event_type(), "PaperCreated");
        
        match event {
            PaperEvent::PaperCreated { paper_id: event_paper_id, metadata: event_metadata, .. } => {
                assert_eq!(event_paper_id, paper_id);
                assert_eq!(event_metadata.title, metadata.title);
            }
            _ => panic!("Expected PaperCreated event"),
        }
    }
    
    #[test]
    fn test_reading_status_changed_event() {
        let paper_id = PaperId::new("2301.12345").unwrap();
        let old_status = ReadingStatus::Unread;
        let new_status = ReadingStatus::Reading { progress: 0.5 };
        
        let event = PaperEventBuilder::new()
            .reading_status_changed(paper_id.clone(), old_status.clone(), new_status.clone());
        
        assert_eq!(event.event_type(), "ReadingStatusChanged");
        
        match event {
            PaperEvent::ReadingStatusChanged { old_status: event_old, new_status: event_new, .. } => {
                assert_eq!(event_old, ReadingStatus::Unread);
                assert_eq!(event_new, ReadingStatus::Reading { progress: 0.5 });
            }
            _ => panic!("Expected ReadingStatusChanged event"),
        }
    }
    
    #[test]
    fn test_event_display() {
        let paper_id = PaperId::new("2301.12345").unwrap();
        let event = PaperEventBuilder::new()
            .download_started(paper_id, "http://example.com/test.pdf".to_string());
        
        let display_str = format!("{}", event);
        assert!(display_str.contains("2301.12345"));
        assert!(display_str.contains("download started"));
    }
    
    #[test]
    fn test_event_publisher() {
        let mut publisher = PaperEventPublisher::new();
        let paper_id = PaperId::new("2301.12345").unwrap();
        
        let event = PaperEventBuilder::new()
            .favorite_status_changed(paper_id, true);
        
        // 测试发布事件（没有处理器时应该成功）
        let result = publisher.publish(event);
        assert!(result.is_ok());
    }
}

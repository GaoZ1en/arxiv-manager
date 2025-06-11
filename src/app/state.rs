//! 应用程序状态管理 - 使用新的模块化架构

use crate::config::Config;
use crate::core::{ArxivPaper, SearchQuery};
use crate::core::app_state::ArxivManager as CoreAppState;
use crate::core::events::EventBus;
use crate::core::arxiv_api::ArxivClient;
use crate::database::{Database, PaperRecord};
use crate::downloader::{DownloadManager, DownloadTask, Priority};
use crate::utils::Result;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum AppMessage {
    // Search related
    SearchQueryChanged(String),
    SearchSubmitted,
    SearchResultsReceived(Vec<ArxivPaper>),
    SearchFailed(String),
    
    // Download related
    DownloadPaper(ArxivPaper),
    DownloadCompleted(String, String),
    DownloadFailed(String, String),
    DownloadProgress(String, u64, Option<u64>),
    
    // UI related
    TabSelected(TabId),
    ThemeToggled,
    WindowResized,
    
    // Database related
    LoadRecentPapers,
    RecentPapersLoaded(Vec<PaperRecord>),
    
    // Settings
    SettingsChanged(Config),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabId {
    Search,
    Library,
    Downloads,
    Settings,
}

/// 集成的应用程序状态
pub struct AppState {
    // 模块化状态组件
    pub core_state: CoreAppState,
    pub event_bus: EventBus,
    
    // 配置
    pub config: Config,
    
    // 服务依赖
    pub arxiv_client: ArxivClient,
    pub database: Arc<Mutex<Database>>,
    pub download_manager: Arc<DownloadManager>,
    
    // 兼容性 - 保留原有的接口用于逐步迁移
    pub download_event_rx: mpsc::UnboundedReceiver<crate::downloader::DownloadEvent>,
    
    // 临时字段用于向后兼容
    pub recent_papers: Vec<PaperRecord>,
    pub selected_paper: Option<PaperRecord>,
    pub window_size: (u32, u32),
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let config = Config::load()?;
        config.ensure_directories()?;
        
        // 初始化数据库
        let database = Arc::new(Mutex::new(Database::new(&config.database.db_path)?));
        
        // 设置下载事件通道（兼容性）
        let (download_event_tx, download_event_rx) = mpsc::unbounded_channel();
        
        // 初始化下载管理器
        let download_manager = Arc::new(DownloadManager::new(
            config.download.max_concurrent_downloads,
            database.clone(),
            download_event_tx,
            config.download.retry_attempts,
            config.download.timeout_seconds,
        ));
        
        // 初始化 arXiv 客户端
        let arxiv_client = ArxivClient::new();
        
        // 创建模块化状态
        let (core_state, _initial_command) = CoreAppState::new();
        
        // 创建事件总线
        let event_bus = EventBus::new();
        
        Ok(Self {
            core_state,
            event_bus,
            config,
            arxiv_client,
            database,
            download_manager,
            download_event_rx,
            recent_papers: Vec::new(),
            selected_paper: None,
            window_size: (1200, 800),
        })
    }
    
    /// 处理应用程序消息，使用新的模块化架构
    pub async fn update(&mut self, message: AppMessage) -> Result<()> {
        match message {
            AppMessage::SearchQueryChanged(query) => {
                // 更新搜索状态
                self.core_state.search_query = query;
            }
            
            AppMessage::SearchSubmitted => {
                let query_text = self.core_state.search_query.clone();
                if !query_text.trim().is_empty() && !self.core_state.is_searching {
                    // 设置搜索状态
                    self.core_state.is_searching = true;
                    self.core_state.search_results.clear();
                    
                    let query = SearchQuery {
                        query: query_text.clone(),
                        max_results: 20,
                        ..Default::default()
                    };
                    
                    match self.arxiv_client.search(&query).await {
                        Ok(results) => {
                            // 更新搜索状态
                            self.core_state.search_results = results.clone();
                            
                            // 存储论文到数据库
                            for paper in &results {
                                if let Ok(db) = self.database.try_lock() {
                                    let _ = db.insert_paper(paper);
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Search failed: {}", e);
                        }
                    }
                    
                    self.core_state.is_searching = false;
                }
            }
            
            AppMessage::DownloadPaper(paper) => {
                let output_path = self.download_manager.generate_file_path(
                    &paper,
                    &self.config.download.download_dir,
                    &self.config.download.naming_pattern,
                );
                
                let task = DownloadTask {
                    paper: paper.clone(),
                    output_path: output_path.clone(),
                    priority: Priority::Normal,
                };
                
                // 在后台启动下载
                let manager = self.download_manager.clone();
                tokio::spawn(async move {
                    let _ = manager.download_paper(task).await;
                });
            }
            
            AppMessage::LoadRecentPapers => {
                if let Ok(db) = self.database.try_lock() {
                    match db.get_recent_papers(50) {
                        Ok(papers) => {
                            self.recent_papers = papers;
                        }
                        Err(e) => {
                            log::error!("Failed to load recent papers: {}", e);
                        }
                    }
                }
            }
            
            AppMessage::TabSelected(tab) => {
                log::info!("Tab selected: {:?}", tab);
                
                // 如果是库标签页，加载最近的论文
                if tab == TabId::Library {
                    if let Ok(db) = self.database.try_lock() {
                        match db.get_recent_papers(50) {
                            Ok(papers) => {
                                self.recent_papers = papers;
                            }
                            Err(e) => {
                                log::error!("Failed to load recent papers: {}", e);
                            }
                        }
                    }
                }
            }
            
            AppMessage::ThemeToggled => {
                log::info!("Theme toggled");
            }
            
            _ => {
                // 处理其他消息
            }
        }
        
        Ok(())
    }
    
    /// 处理下载事件（兼容性方法）
    pub fn process_download_events(&mut self) {
        while let Ok(event) = self.download_event_rx.try_recv() {
            match event {
                crate::downloader::DownloadEvent::Progress { arxiv_id, bytes_downloaded, total_bytes } => {
                    log::info!("Download progress: {} - {}/{:?}", arxiv_id, bytes_downloaded, total_bytes);
                }
                crate::downloader::DownloadEvent::Completed { arxiv_id, file_path } => {
                    log::info!("Download completed: {} -> {}", arxiv_id, file_path.display());
                }
                crate::downloader::DownloadEvent::Failed { arxiv_id, error } => {
                    log::error!("Download failed: {} - {}", arxiv_id, error);
                }
                _ => {}
            }
        }
    }
    
    // 向后兼容的访问器方法
    
    /// 获取当前搜索查询
    pub fn search_query(&self) -> &str {
        &self.core_state.search_query
    }
    
    /// 获取搜索结果
    pub fn search_results(&self) -> &[ArxivPaper] {
        &self.core_state.search_results
    }
    
    /// 检查是否正在搜索
    pub fn is_searching(&self) -> bool {
        self.core_state.is_searching
    }
    
    /// 获取活动标签页（临时实现）
    pub fn active_tab(&self) -> TabId {
        TabId::Search
    }
    
    /// 检查是否为暗色主题（临时实现）
    pub fn theme_dark(&self) -> bool {
        true
    }
    
    /// 获取下载进度（临时实现）
    pub fn download_progress(&self) -> HashMap<String, (u64, Option<u64>)> {
        HashMap::new()
    }
    
    /// 获取下载错误（临时实现）
    pub fn download_errors(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}

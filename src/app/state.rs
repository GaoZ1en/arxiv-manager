use crate::config::Config;
use crate::core::{ArxivClient, ArxivPaper, SearchQuery};
use crate::database::{Database, PaperRecord};
use crate::downloader::{DownloadManager, DownloadTask, DownloadEvent, Priority, DownloadQueue};
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
    DownloadCompleted(String, String), // arxiv_id, file_path
    DownloadFailed(String, String),    // arxiv_id, error
    DownloadProgress(String, u64, Option<u64>), // arxiv_id, downloaded, total
    
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

#[derive(Debug)]
pub struct AppState {
    // Configuration
    pub config: Config,
    
    // Search state
    pub search_query: String,
    pub search_results: Vec<ArxivPaper>,
    pub is_searching: bool,
    
    // Library state
    pub recent_papers: Vec<PaperRecord>,
    pub selected_paper: Option<PaperRecord>,
    
    // Download state
    pub download_queue: DownloadQueue,
    pub download_progress: HashMap<String, (u64, Option<u64>)>, // arxiv_id -> (downloaded, total)
    pub download_errors: HashMap<String, String>, // arxiv_id -> error_message
    
    // UI state
    pub active_tab: TabId,
    pub theme_dark: bool,
    pub window_size: (u32, u32),
    
    // Services
    pub arxiv_client: ArxivClient,
    pub database: Arc<Mutex<Database>>,
    pub download_manager: Arc<DownloadManager>,
    
    // Event channels
    pub download_event_rx: mpsc::UnboundedReceiver<DownloadEvent>,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let config = Config::load()?;
        config.ensure_directories()?;
        
        // Initialize database
        let database = Arc::new(Mutex::new(Database::new(&config.database.db_path)?));
        
        // Setup download event channel
        let (download_event_tx, download_event_rx) = mpsc::unbounded_channel();
        
        // Initialize download manager
        let download_manager = Arc::new(DownloadManager::new(
            config.download.max_concurrent_downloads,
            database.clone(),
            download_event_tx,
            config.download.retry_attempts,
            config.download.timeout_seconds,
        ));
        
        // Initialize arXiv client
        let arxiv_client = ArxivClient::new();
        
        Ok(Self {
            config,
            search_query: String::new(),
            search_results: Vec::new(),
            is_searching: false,
            recent_papers: Vec::new(),
            selected_paper: None,
            download_queue: DownloadQueue::new(),
            download_progress: HashMap::new(),
            download_errors: HashMap::new(),
            active_tab: TabId::Search,
            theme_dark: true,
            window_size: (1200, 800),
            arxiv_client,
            database,
            download_manager,
            download_event_rx,
        })
    }
    
    pub async fn update(&mut self, message: AppMessage) -> Result<()> {
        match message {
            AppMessage::SearchQueryChanged(query) => {
                self.search_query = query;
            }
            
            AppMessage::SearchSubmitted => {
                if !self.search_query.trim().is_empty() && !self.is_searching {
                    self.is_searching = true;
                    self.search_results.clear();
                    
                    let query = SearchQuery {
                        query: self.search_query.clone(),
                        max_results: 20,
                        ..Default::default()
                    };
                    
                    match self.arxiv_client.search(&query).await {
                        Ok(results) => {
                            self.search_results = results;
                            // Store papers in database
                            for paper in &self.search_results {
                                if let Ok(db) = self.database.try_lock() {
                                    let _ = db.insert_paper(paper);
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Search failed: {}", e);
                        }
                    }
                    
                    self.is_searching = false;
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
                    output_path,
                    priority: Priority::Normal,
                };
                
                self.download_queue.add_task(task.clone());
                
                // Start download in background
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
                self.active_tab = tab;
                
                // Load data when switching to library tab
                if tab == TabId::Library {
                    // Load recent papers directly instead of recursive call
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
                self.theme_dark = !self.theme_dark;
            }
            
            _ => {
                // Handle other messages
            }
        }
        
        Ok(())
    }
    
    pub fn process_download_events(&mut self) {
        while let Ok(event) = self.download_event_rx.try_recv() {
            match event {
                DownloadEvent::Progress { arxiv_id, bytes_downloaded, total_bytes } => {
                    self.download_progress.insert(arxiv_id, (bytes_downloaded, total_bytes));
                }
                DownloadEvent::Completed { arxiv_id, .. } => {
                    self.download_progress.remove(&arxiv_id);
                    self.download_errors.remove(&arxiv_id);
                }
                DownloadEvent::Failed { arxiv_id, error } => {
                    self.download_progress.remove(&arxiv_id);
                    self.download_errors.insert(arxiv_id, error);
                }
                _ => {}
            }
        }
    }
}

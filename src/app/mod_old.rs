use iced::{
    Application, Command, Element, Settings, Subscription,
    Theme, executor, window, Length, Color,
};
use std::collections::HashMap;
use tokio::sync::mpsc;

use crate::config::AppConfig;
use crate::core::{ArxivClient, models::ArxivPaper};
use crate::database::{Database, models::Paper};
use crate::downloader::{DownloadManager, DownloadEvent, DownloadTask};
use crate::ui::{style, Message, View};

/// arXiv 管理器主应用
pub struct ArxivManager {
    pub config: AppConfig,
    arxiv_client: ArxivClient,
    download_manager: DownloadManager,
    database: Option<Database>,
    
    // UI 状态
    current_view: View,
    search_query: String,
    papers: Vec<Paper>,
    search_results: Vec<ArxivPaper>,
    pub download_tasks: Vec<DownloadTask>,
    selected_paper: Option<usize>,
    download_events: Option<mpsc::UnboundedReceiver<DownloadEvent>>,
    
    // 应用状态
    is_loading: bool,
    pub is_online: bool,
    error_message: Option<String>,
    status_message: Option<String>,
    pub total_papers: Option<usize>,
    pub downloaded_papers: Option<usize>,
}

impl Application for ArxivManager {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = AppConfig;

    fn new(config: AppConfig) -> (Self, Command<Self::Message>) {
        let arxiv_client = ArxivClient::new();
        let download_manager = DownloadManager::new(config.clone());
        
        let app = Self {
            config: config.clone(),
            arxiv_client,
            download_manager,
            database: None,
            current_view: View::Search,
            search_query: String::new(),
            papers: Vec::new(),
            search_results: Vec::new(),
            download_tasks: Vec::new(),
            selected_paper: None,
            download_events: None,
            is_loading: false,
            is_online: true,
            error_message: None,
            status_message: None,
            total_papers: None,
            downloaded_papers: None,
        };

        let commands = vec![
            // 初始化数据库
            Command::perform(
                Database::new(&config.database.database_path),
                |result| match result {
                    Ok(_db) => Message::LoadData,
                    Err(e) => Message::ShowError(format!("数据库初始化失败: {}", e)),
                }
            ),
        ];

        (app, Command::batch(commands))
    }

    fn title(&self) -> String {
        match &self.current_view {
            View::Search => "arXiv 管理器 - 搜索".to_string(),
            View::Library => "arXiv 管理器 - 文献库".to_string(),
            View::Downloads => "arXiv 管理器 - 下载".to_string(),
            View::Settings => "arXiv 管理器 - 设置".to_string(),
            View::Reader => "arXiv 管理器 - 阅读器".to_string(),
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ChangeView(view) => {
                self.current_view = view;
                Command::none()
            }
            
            Message::SearchInputChanged(query) => {
                self.search_query = query;
                Command::none()
            }
            
            Message::SearchSubmitted => {
                if self.search_query.trim().is_empty() {
                    return Command::none();
                }

                self.is_loading = true;
                self.error_message = None;
                
                let client = self.arxiv_client.clone();
                let query = self.search_query.clone();
                
                Command::perform(
                    async move {
                        match client.search_by_keywords(&[query], 50).await {
                            Ok(result) => Message::ShowStatus(format!("找到 {} 篇论文", result.entries.len())),
                            Err(e) => Message::ShowError(format!("搜索失败: {}", e)),
                        }
                    },
                    |msg| msg,
                )
            }
            
            Message::DownloadPaper(index) => {
                if let Some(paper) = self.search_results.get(index) {
                    // 创建下载任务
                    let download_task = DownloadTask::new(
                        paper.id.clone(),
                        paper.title.clone(),
                        paper.pdf_url.clone(),
                        self.config.download.download_directory.join(format!("{}.pdf", paper.id)),
                    );
                    self.download_tasks.push(download_task);
                }
                Command::none()
            }
            
            Message::StartDownload(index) => {
                if let Some(task) = self.download_tasks.get_mut(index) {
                    task.start();
                }
                Command::none()
            }
            
            Message::PauseDownload(index) => {
                if let Some(task) = self.download_tasks.get_mut(index) {
                    task.pause();
                }
                Command::none()
            }
            
            Message::ResumeDownload(index) => {
                if let Some(task) = self.download_tasks.get_mut(index) {
                    task.resume();
                }
                Command::none()
            }
            
            Message::RemoveDownload(index) => {
                if index < self.download_tasks.len() {
                    self.download_tasks.remove(index);
                }
                Command::none()
            }
            
            Message::ClearCompletedDownloads => {
                self.download_tasks.retain(|task| !task.is_completed());
                Command::none()
            }
            
            Message::UpdateDownloadDirectory(path) => {
                self.config.download.download_directory = std::path::PathBuf::from(path);
                Command::none()
            }
            
            Message::SaveSettings => {
                let config = self.config.clone();
                Command::perform(
                    async move {
                        match config.save() {
                            Ok(_) => Message::ShowStatus("设置已保存".to_string()),
                            Err(e) => Message::ShowError(format!("保存设置失败: {}", e)),
                        }
                    },
                    |msg| msg,
                )
            }
            
            Message::ShowError(error) => {
                self.error_message = Some(error);
                Command::none()
            }
            
            Message::HideError => {
                self.error_message = None;
                Command::none()
            }
            
            Message::ShowStatus(status) => {
                self.status_message = Some(status);
                Command::none()
            }
            
            Message::HideStatus => {
                self.status_message = None;
                Command::none()
            }
            
            Message::LoadData => {
                // 初始化统计数据
                self.total_papers = Some(0);
                self.downloaded_papers = Some(0);
                Command::none()
            }
            
            _ => {
                // 暂时忽略其他消息，等实现完整功能后再处理
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        crate::ui::view::main_view(self)
    }

    fn theme(&self) -> Self::Theme {
        match self.config.ui.theme.as_str() {
            "light" => Theme::Light,
            _ => Theme::Dark,
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch(vec![
            // 窗口事件
            window::events().map(|_event| {
                Message::HideStatus // 简化事件处理
            }),
        ])
    }
}

impl ArxivManager {
    /// 获取当前视图
    pub fn current_view(&self) -> View {
        self.current_view
    }

    /// 获取搜索查询
    pub fn search_query(&self) -> &str {
        &self.search_query
    }

    /// 获取搜索结果
    pub fn search_results(&self) -> &[ArxivPaper] {
        &self.search_results
    }

    /// 获取本地论文
    pub fn papers(&self) -> &[Paper] {
        &self.papers
    }

    /// 获取是否正在加载
    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    /// 获取错误信息
    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    /// 获取状态信息
    pub fn status_message(&self) -> Option<&str> {
        self.status_message.as_deref()
    }
}

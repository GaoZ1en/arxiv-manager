// arXiv 论文管理器 - 带有真实API集成的完整应用程序
use iced::widget::{button, column, container, row, text, text_input, scrollable, progress_bar};
use iced::{Element, Task, Theme, Length};
use std::path::PathBuf;

fn main() -> iced::Result {
    env_logger::init();
    
    iced::application("arXiv Paper Manager", update, view)
        .theme(|state: &State| {
            if state.dark_theme {
                Theme::Dark
            } else {
                Theme::Light
            }
        })
        .window_size((1200.0, 800.0))
        .run_with(|| {
            let initial_state = State::new();
            let init_task = Task::perform(async {
                Message::AppInitialized
            }, |result| result);
            (initial_state, init_task)
        })
}

#[derive(Debug, Clone)]
struct ArxivPaper {
    id: String,
    title: String,
    authors: Vec<String>,
    abstract_text: String,
    published: String,
    updated: String,
    categories: Vec<String>,
    pdf_url: String,
    entry_url: String,
}

#[derive(Debug, Clone)]
struct State {
    active_tab: Tab,
    search_query: String,
    search_results: Vec<ArxivPaper>,
    saved_papers: Vec<ArxivPaper>,
    downloads: Vec<DownloadItem>,
    dark_theme: bool,
    is_searching: bool,
    search_error: Option<String>,
}

#[derive(Debug, Clone)]
struct DownloadItem {
    paper_id: String,
    title: String,
    progress: f32,
    status: DownloadStatus,
    file_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
enum DownloadStatus {
    Pending,
    Downloading,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tab {
    Search,
    Library,
    Downloads,
    Settings,
}

#[derive(Debug, Clone)]
enum Message {
    AppInitialized,
    TabSelected(Tab),
    SearchQueryChanged(String),
    SearchSubmitted,
    SearchCompleted(Result<Vec<ArxivPaper>, String>),
    DownloadPaper(ArxivPaper),
    DownloadProgress { paper_id: String, progress: f32 },
    DownloadCompleted { paper_id: String, file_path: PathBuf },
    DownloadFailed { paper_id: String, error: String },
    SavePaper(ArxivPaper),
    RemovePaper(String),
    ThemeToggled,
}

impl State {
    fn new() -> Self {
        Self {
            active_tab: Tab::Search,
            search_query: String::new(),
            search_results: Vec::new(),
            saved_papers: Vec::new(),
            downloads: Vec::new(),
            dark_theme: true,
            is_searching: false,
            search_error: None,
        }
    }
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::AppInitialized => {
            // 应用程序初始化完成
            Task::none()
        }
        Message::TabSelected(tab) => {
            state.active_tab = tab;
            Task::none()
        }
        Message::SearchQueryChanged(query) => {
            state.search_query = query;
            Task::none()
        }
        Message::SearchSubmitted => {
            if !state.search_query.trim().is_empty() {
                state.is_searching = true;
                state.search_error = None;
                let query = state.search_query.clone();
                
                Task::perform(
                    search_arxiv_papers(query),
                    Message::SearchCompleted
                )
            } else {
                Task::none()
            }
        }
        Message::SearchCompleted(result) => {
            state.is_searching = false;
            match result {
                Ok(papers) => {
                    state.search_results = papers;
                    state.search_error = None;
                }
                Err(error) => {
                    state.search_error = Some(error);
                    state.search_results.clear();
                }
            }
            Task::none()
        }
        Message::DownloadPaper(paper) => {
            // 添加到下载队列
            let download_item = DownloadItem {
                paper_id: paper.id.clone(),
                title: paper.title.clone(),
                progress: 0.0,
                status: DownloadStatus::Pending,
                file_path: None,
            };
            state.downloads.push(download_item);
            
            // 开始下载
            Task::perform(
                download_pdf(paper),
                |result| match result {
                    Ok((paper_id, file_path)) => Message::DownloadCompleted { paper_id, file_path },
                    Err((paper_id, error)) => Message::DownloadFailed { paper_id, error },
                }
            )
        }
        Message::DownloadProgress { paper_id, progress } => {
            // 更新下载进度
            if let Some(download) = state.downloads.iter_mut().find(|d| d.paper_id == paper_id) {
                download.progress = progress;
                download.status = DownloadStatus::Downloading;
            }
            Task::none()
        }
        Message::DownloadCompleted { paper_id, file_path } => {
            // 下载完成
            if let Some(download) = state.downloads.iter_mut().find(|d| d.paper_id == paper_id) {
                download.progress = 100.0;
                download.status = DownloadStatus::Completed;
                download.file_path = Some(file_path);
            }
            Task::none()
        }
        Message::DownloadFailed { paper_id, error } => {
            // 下载失败
            if let Some(download) = state.downloads.iter_mut().find(|d| d.paper_id == paper_id) {
                download.status = DownloadStatus::Failed(error);
            }
            Task::none()
        }
        Message::SavePaper(paper) => {
            // 保存论文到库
            if !state.saved_papers.iter().any(|p| p.id == paper.id) {
                state.saved_papers.push(paper);
            }
            Task::none()
        }
        Message::RemovePaper(paper_id) => {
            // 从库中移除论文
            state.saved_papers.retain(|p| p.id != paper_id);
            Task::none()
        }
        Message::ThemeToggled => {
            state.dark_theme = !state.dark_theme;
            Task::none()
        }
    }
}

fn view(state: &State) -> Element<Message> {
    let tab_bar = row![
        tab_button("Search", Tab::Search, state.active_tab),
        tab_button("Library", Tab::Library, state.active_tab),
        tab_button("Downloads", Tab::Downloads, state.active_tab),
        tab_button("Settings", Tab::Settings, state.active_tab),
    ]
    .spacing(10)
    .padding(20);

    let content = match state.active_tab {
        Tab::Search => search_view(state),
        Tab::Library => library_view(state),
        Tab::Downloads => downloads_view(state),
        Tab::Settings => settings_view(state),
    };

    column![
        container(tab_bar),
        container(content).height(iced::Length::Fill)
    ]
    .into()
}

fn tab_button(label: &str, tab: Tab, active_tab: Tab) -> Element<Message> {
    let btn = button(text(label).size(16)).padding([8, 16]);
    
    if tab == active_tab {
        // Could add different styling for active tab
    }
    
    btn.on_press(Message::TabSelected(tab)).into()
}

fn search_view(state: &State) -> Element<Message> {
    let search_input = text_input("Enter keywords or arXiv ID...", &state.search_query)
        .on_input(Message::SearchQueryChanged)
        .on_submit(Message::SearchSubmitted)
        .padding(10)
        .size(16);

    let search_button = button(
        text(if state.is_searching { "Searching..." } else { "Search arXiv" }).size(16)
    )
    .on_press(Message::SearchSubmitted)
    .padding([10, 20]);

    let search_row = row![search_input, search_button]
        .spacing(10);

    let content = if state.is_searching {
        column![
            text("🔍 Searching arXiv database...").size(18),
            text("Please wait, fetching latest data from arXiv.org").size(14),
        ]
        .spacing(10)
    } else if let Some(error) = &state.search_error {
        column![
            text("❌ Search Error").size(18),
            text(format!("Error: {}", error)).size(14),
            text("Please check network connection and try again").size(12),
        ]
        .spacing(10)
    } else if state.search_results.is_empty() && !state.search_query.is_empty() {
        column![
            text("🔍 No papers found").size(18),
            text("Try different keywords or check spelling").size(14),
        ]
        .spacing(10)
    } else if state.search_results.is_empty() {
        column![
            text("🔍 arXiv Paper Search").size(24),
            text("Enter keywords to search academic papers").size(16),
            text(""),
            text("Supported search types:").size(14),
            text("• Keyword search (e.g. machine learning)").size(12),
            text("• Author name (e.g. Geoffrey Hinton)").size(12),
            text("• arXiv ID (e.g. 1706.03762)").size(12),
            text("• Category search (e.g. cs.AI, cs.LG)").size(12),
        ]
        .spacing(5)
    } else {
        let mut results_column = column![
            text(format!("Search Results ({} papers):", state.search_results.len())).size(18),
        ].spacing(15);

        for paper in &state.search_results {
            let authors_text = if paper.authors.len() > 3 {
                format!("{} et al.", paper.authors[..3].join(", "))
            } else {
                paper.authors.join(", ")
            };
            
            let categories_text = paper.categories.join(", ");
            
            let paper_card = container(
                column![
                    text(&paper.title).size(16),
                    text(format!("Authors: {}", authors_text)).size(12),
                    text(format!("Categories: {}", categories_text)).size(11),
                    text(format!("Published: {}", paper.published)).size(11),
                    text(format!("arXiv ID: {}", paper.id)).size(11),
                    text(format!("Abstract: {}", 
                        if paper.abstract_text.len() > 200 {
                            format!("{}...", &paper.abstract_text[..200])
                        } else {
                            paper.abstract_text.clone()
                        }
                    )).size(12),
                    row![
                        button(text("Download PDF").size(12))
                            .on_press(Message::DownloadPaper(paper.clone()))
                            .padding([5, 10]),
                        button(text("Save to Library").size(12))
                            .on_press(Message::SavePaper(paper.clone()))
                            .padding([5, 10]),
                        button(text("View Details").size(12))
                            .padding([5, 10]),
                    ].spacing(10)
                ].spacing(8)
            )
            .padding(15);
            
            results_column = results_column.push(paper_card);
        }
        
        column![
            scrollable(results_column)
                .height(Length::Fill)
        ]
    };

    column![
        search_row,
        content
    ]
    .spacing(20)
    .padding(30)
    .into()
}

fn library_view(state: &State) -> Element<Message> {
    if state.saved_papers.is_empty() {
        column![
            text("📚 Paper Library").size(24),
            text("Your paper collection and download history").size(16),
            text(""),
            text("No papers saved yet").size(14),
            text("Save interesting papers from the search page").size(12),
            text(""),
            text("Feature preview:").size(14),
            text("• Saved papers list").size(12),
            text("• Favorites and tag management").size(12),
            text("• Reading progress tracking").size(12),
            text("• Paper notes and annotations").size(12),
            text("• Citation export (BibTeX)").size(12),
        ]
        .spacing(10)
        .padding(30)
        .into()
    } else {
        let mut library_column = column![
            text(format!("📚 Paper Library ({} papers)", state.saved_papers.len())).size(24),
        ].spacing(15);

        for paper in &state.saved_papers {
            let authors_text = if paper.authors.len() > 3 {
                format!("{} et al.", paper.authors[..3].join(", "))
            } else {
                paper.authors.join(", ")
            };
            
            let paper_card = container(
                column![
                    text(&paper.title).size(16),
                    text(format!("Authors: {}", authors_text)).size(12),
                    text(format!("Categories: {}", paper.categories.join(", "))).size(11),
                    text(format!("arXiv ID: {}", paper.id)).size(11),
                    row![
                        button(text("Download PDF").size(12))
                            .on_press(Message::DownloadPaper(paper.clone()))
                            .padding([5, 10]),
                        button(text("Remove from Library").size(12))
                            .on_press(Message::RemovePaper(paper.id.clone()))
                            .padding([5, 10]),
                        button(text("View Details").size(12))
                            .padding([5, 10]),
                    ].spacing(10)
                ].spacing(8)
            )
            .padding(15);
            
            library_column = library_column.push(paper_card);
        }
        
        container(
            scrollable(library_column)
                .height(Length::Fill)
        )
        .padding(30)
        .into()
    }
}

fn downloads_view(state: &State) -> Element<Message> {
    if state.downloads.is_empty() {
        column![
            text("⬇️ Download Manager").size(24),
            text("PDF download queue and progress").size(16),
            text(""),
            text("No active downloads").size(14),
            text("Start downloading papers from search or library page").size(12),
            text(""),
            text("Features:").size(14),
            text("• Multi-threaded concurrent downloads").size(12),
            text("• Download progress monitoring").size(12),
            text("• Automatic retry mechanism").size(12),
            text("• Download history tracking").size(12),
        ]
        .spacing(10)
        .padding(30)
        .into()
    } else {
        let mut downloads_column = column![
            text(format!("⬇️ Download Manager ({} tasks)", state.downloads.len())).size(24),
        ].spacing(15);

        for download in &state.downloads {
            let status_text = match &download.status {
                DownloadStatus::Pending => "Pending".to_string(),
                DownloadStatus::Downloading => "Downloading".to_string(),
                DownloadStatus::Completed => "Completed".to_string(),
                DownloadStatus::Failed(error) => format!("Failed: {}", error),
            };
            
            let progress_section = match &download.status {
                DownloadStatus::Downloading => {
                    column![
                        text(format!("Progress: {:.1}%", download.progress)).size(12),
                        progress_bar(0.0..=100.0, download.progress)
                    ].spacing(5)
                }
                DownloadStatus::Completed => {
                    column![
                        text("100% Completed").size(12),
                        progress_bar(0.0..=100.0, 100.0)
                    ].spacing(5)
                }
                _ => {
                    column![text(status_text.clone()).size(12)]
                }
            };
            
            let download_card = container(
                column![
                    text(&download.title).size(14),
                    text(format!("arXiv ID: {}", download.paper_id)).size(11),
                    progress_section,
                    if let Some(file_path) = &download.file_path {
                        text(format!("File location: {}", file_path.display())).size(10)
                    } else {
                        text("").size(10)
                    }
                ].spacing(5)
            )
            .padding(15);
            
            downloads_column = downloads_column.push(download_card);
        }
        
        container(
            scrollable(downloads_column)
                .height(Length::Fill)
        )
        .padding(30)
        .into()
    }
}

fn settings_view(state: &State) -> Element<Message> {
    let theme_button = button(
        text(if state.dark_theme { "Switch to Light Theme" } else { "Switch to Dark Theme" }).size(14)
    )
    .on_press(Message::ThemeToggled)
    .padding([8, 16]);

    column![
        text("⚙️ Settings").size(24),
        text("Application Configuration").size(16),
        text(""),
        row![
            text("Theme:").size(14),
            theme_button
        ].spacing(10),
        text(""),
        text("Other settings (under development):").size(14),
        text("• Download directory configuration").size(12),
        text("• Concurrent download count").size(12),
        text("• File naming rules").size(12),
        text("• Shortcut customization").size(12),
        text("• Language settings").size(12),
    ]
    .spacing(10)
    .padding(30)
    .into()
}

// 异步函数：搜索arXiv论文
async fn search_arxiv_papers(query: String) -> Result<Vec<ArxivPaper>, String> {
    use reqwest::Client;
    
    let client = Client::new();
    let search_url = format!(
        "http://export.arxiv.org/api/query?search_query=all:{}&start=0&max_results=20&sortBy=relevance&sortOrder=descending",
        urlencoding::encode(&query)
    );
    
    match client.get(&search_url).send().await {
        Ok(response) => {
            match response.text().await {
                Ok(xml_content) => {
                    parse_arxiv_response(&xml_content)
                }
                Err(e) => Err(format!("Failed to read response: {}", e))
            }
        }
        Err(e) => Err(format!("Network error: {}", e))
    }
}

// 解析arXiv API的XML响应
fn parse_arxiv_response(xml_content: &str) -> Result<Vec<ArxivPaper>, String> {
    use roxmltree::Document;
    
    let doc = Document::parse(xml_content)
        .map_err(|e| format!("XML parsing error: {}", e))?;
    
    let mut papers = Vec::new();
    
    // 查找所有entry元素
    for entry in doc.descendants().filter(|n| n.has_tag_name("entry")) {
        let mut paper = ArxivPaper {
            id: String::new(),
            title: String::new(),
            authors: Vec::new(),
            abstract_text: String::new(),
            published: String::new(),
            updated: String::new(),
            categories: Vec::new(),
            pdf_url: String::new(),
            entry_url: String::new(),
        };
        
        for child in entry.children() {
            match child.tag_name().name() {
                "id" => {
                    if let Some(text) = child.text() {
                        paper.id = text.trim().to_string();
                        if let Some(id_part) = text.split('/').last() {
                            paper.id = id_part.to_string();
                        }
                    }
                }
                "title" => {
                    if let Some(text) = child.text() {
                        paper.title = text.trim().replace('\n', " ").replace("  ", " ");
                    }
                }
                "author" => {
                    for name_elem in child.children().filter(|n| n.has_tag_name("name")) {
                        if let Some(name) = name_elem.text() {
                            paper.authors.push(name.trim().to_string());
                        }
                    }
                }
                "summary" => {
                    if let Some(text) = child.text() {
                        paper.abstract_text = text.trim().replace('\n', " ").replace("  ", " ");
                    }
                }
                "published" => {
                    if let Some(text) = child.text() {
                        paper.published = text.trim().to_string();
                    }
                }
                "updated" => {
                    if let Some(text) = child.text() {
                        paper.updated = text.trim().to_string();
                    }
                }
                "category" => {
                    if let Some(term) = child.attribute("term") {
                        paper.categories.push(term.to_string());
                    }
                }
                "link" => {
                    if let Some(href) = child.attribute("href") {
                        if let Some(rel) = child.attribute("rel") {
                            match rel {
                                "alternate" => paper.entry_url = href.to_string(),
                                "related" => {
                                    if let Some(title) = child.attribute("title") {
                                        if title == "pdf" {
                                            paper.pdf_url = href.to_string();
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        
        // 只添加有效的论文记录
        if !paper.id.is_empty() && !paper.title.is_empty() {
            papers.push(paper);
        }
    }
    
    Ok(papers)
}

// 异步函数：下载PDF文件
async fn download_pdf(paper: ArxivPaper) -> Result<(String, PathBuf), (String, String)> {
    use reqwest::Client;
    use std::fs;
    
    let client = Client::new();
    let downloads_dir = dirs::download_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap())
        .join("arxiv_papers");
    
    // 创建下载目录
    if let Err(e) = fs::create_dir_all(&downloads_dir) {
        return Err((paper.id, format!("Failed to create download directory: {}", e)));
    }
    
    // 清理文件名
    let safe_title = paper.title
        .chars()
        .map(|c| if c.is_alphanumeric() || c == ' ' || c == '-' || c == '_' { c } else { '_' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("_");
    
    let filename = format!("{}_{}.pdf", paper.id.replace("/", "_"), safe_title);
    let file_path = downloads_dir.join(filename);
    
    // 如果文件已存在，直接返回
    if file_path.exists() {
        return Ok((paper.id, file_path));
    }
    
    // 下载PDF
    match client.get(&paper.pdf_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.bytes().await {
                    Ok(bytes) => {
                        match tokio::fs::write(&file_path, bytes).await {
                            Ok(_) => Ok((paper.id, file_path)),
                            Err(e) => Err((paper.id, format!("Failed to write file: {}", e)))
                        }
                    }
                    Err(e) => Err((paper.id, format!("Failed to read response: {}", e)))
                }
            } else {
                Err((paper.id, format!("HTTP error: {}", response.status())))
            }
        }
        Err(e) => Err((paper.id, format!("Network error: {}", e)))
    }
}

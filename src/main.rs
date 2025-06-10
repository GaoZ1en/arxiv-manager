// arXiv 论文管理器 - 带有 halloy 风格界面

use iced::widget::{
    button, column, container, row, text, text_input, scrollable, progress_bar, 
    pane_grid, horizontal_rule, vertical_space
};
use iced::{Element, Task, Length, Color, Background, Border, Shadow};
use std::path::PathBuf;
use std::time::Instant;

fn main() -> iced::Result {
    env_logger::init();
    
    iced::application("arXiv Paper Manager", ArxivManager::update, ArxivManager::view)
        .theme(ArxivManager::theme)
        .window_size((1400.0, 900.0))
        .run_with(|| {
            let (app, task) = ArxivManager::new();
            (app, task)
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
enum PaneType {
    Search,
    Library,
    Downloads,
    Settings,
    PaperView(usize),
}

#[derive(Clone, Debug)]
struct Pane {
    pane_type: PaneType,
    title: String,
}

struct ArxivManager {
    panes: pane_grid::State<Pane>,
    focus: Option<pane_grid::Pane>,
    sidebar_visible: bool,
    search_query: String,
    search_results: Vec<ArxivPaper>,
    saved_papers: Vec<ArxivPaper>,
    downloads: Vec<DownloadItem>,
    is_searching: bool,
    search_error: Option<String>,
    last_interaction: Option<Instant>,
}

#[derive(Debug, Clone)]
enum Message {
    PaneClicked(pane_grid::Pane),
    PaneResized(pane_grid::ResizeEvent),
    PaneDragged(pane_grid::DragEvent),
    SidebarToggled,
    SearchQueryChanged(String),
    SearchSubmitted,
    SearchCompleted(Result<Vec<ArxivPaper>, String>),
    DownloadPaper(ArxivPaper),
    DownloadProgress { paper_id: String, progress: f32 },
    DownloadCompleted { paper_id: String, file_path: PathBuf },
    DownloadFailed { paper_id: String, error: String },
    SavePaper(ArxivPaper),
    RemovePaper(String),
    OpenPaperPane(ArxivPaper),
    CloseFocusedPane,
    SplitHorizontal,
    SplitVertical,
}

// Gruvbox 暗色主题颜色
const GRUVBOX_BG: Color = Color::from_rgb(0.16, 0.16, 0.16);     // #282828
const GRUVBOX_SURFACE: Color = Color::from_rgb(0.20, 0.19, 0.17); // #32302f
const GRUVBOX_BORDER: Color = Color::from_rgb(0.35, 0.33, 0.29);  // #595959
const GRUVBOX_TEXT: Color = Color::from_rgb(0.92, 0.86, 0.70);    // #ebdbb2
const GRUVBOX_TEXT_MUTED: Color = Color::from_rgb(0.66, 0.61, 0.52); // #a89984
const GRUVBOX_GREEN: Color = Color::from_rgb(0.72, 0.73, 0.15);   // #b8bb26
const GRUVBOX_RED: Color = Color::from_rgb(0.98, 0.38, 0.37);     // #fb4934
const GRUVBOX_ORANGE: Color = Color::from_rgb(0.80, 0.41, 0.13);  // #cc6d19

impl ArxivManager {
    fn new() -> (Self, Task<Message>) {
        let (mut panes, _first_pane) = pane_grid::State::new(Pane {
            pane_type: PaneType::Search,
            title: "Search".to_string(),
        });

        let manager = Self {
            panes,
            focus: None,
            sidebar_visible: true,
            search_query: String::new(),
            search_results: Vec::new(),
            saved_papers: Vec::new(),
            downloads: Vec::new(),
            is_searching: false,
            search_error: None,
            last_interaction: None,
        };

        (manager, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PaneClicked(pane) => {
                self.focus = Some(pane);
                self.last_interaction = Some(Instant::now());
                Task::none()
            }
            Message::PaneResized(resize_event) => {
                self.panes.resize(resize_event.split, resize_event.ratio);
                Task::none()
            }
            Message::PaneDragged(_drag_event) => {
                // In iced 0.13, drag handling is managed automatically by the pane grid
                Task::none()
            }
            Message::SidebarToggled => {
                self.sidebar_visible = !self.sidebar_visible;
                Task::none()
            }
            Message::SearchQueryChanged(query) => {
                self.search_query = query;
                Task::none()
            }
            Message::SearchSubmitted => {
                if !self.search_query.trim().is_empty() {
                    self.is_searching = true;
                    self.search_error = None;
                    let query = self.search_query.clone();
                    
                    Task::perform(
                        search_arxiv_papers(query),
                        Message::SearchCompleted
                    )
                } else {
                    Task::none()
                }
            }
            Message::SearchCompleted(result) => {
                self.is_searching = false;
                match result {
                    Ok(papers) => {
                        self.search_results = papers;
                        self.search_error = None;
                    }
                    Err(error) => {
                        self.search_error = Some(error);
                        self.search_results.clear();
                    }
                }
                Task::none()
            }
            Message::DownloadPaper(paper) => {
                let download_item = DownloadItem {
                    paper_id: paper.id.clone(),
                    title: paper.title.clone(),
                    progress: 0.0,
                    status: DownloadStatus::Pending,
                    file_path: None,
                };
                self.downloads.push(download_item);
                
                Task::perform(
                    download_pdf(paper),
                    |result| match result {
                        Ok((paper_id, file_path)) => Message::DownloadCompleted { paper_id, file_path },
                        Err((paper_id, error)) => Message::DownloadFailed { paper_id, error },
                    }
                )
            }
            Message::DownloadProgress { paper_id, progress } => {
                if let Some(download) = self.downloads.iter_mut().find(|d| d.paper_id == paper_id) {
                    download.progress = progress;
                    download.status = DownloadStatus::Downloading;
                }
                Task::none()
            }
            Message::DownloadCompleted { paper_id, file_path } => {
                if let Some(download) = self.downloads.iter_mut().find(|d| d.paper_id == paper_id) {
                    download.progress = 100.0;
                    download.status = DownloadStatus::Completed;
                    download.file_path = Some(file_path);
                }
                Task::none()
            }
            Message::DownloadFailed { paper_id, error } => {
                if let Some(download) = self.downloads.iter_mut().find(|d| d.paper_id == paper_id) {
                    download.status = DownloadStatus::Failed(error);
                }
                Task::none()
            }
            Message::SavePaper(paper) => {
                if !self.saved_papers.iter().any(|p| p.id == paper.id) {
                    self.saved_papers.push(paper);
                }
                Task::none()
            }
            Message::RemovePaper(paper_id) => {
                self.saved_papers.retain(|p| p.id != paper_id);
                Task::none()
            }
            Message::OpenPaperPane(paper) => {
                let index = self.saved_papers.len();
                self.saved_papers.push(paper.clone());
                
                let pane_type = PaneType::PaperView(index);
                let new_pane = Pane {
                    pane_type,
                    title: paper.title.clone(),
                };
                
                if let Some(focus) = self.focus {
                    let _ = self.panes.split(
                        pane_grid::Axis::Vertical,
                        focus,
                        new_pane,
                    );
                }
                // If no focus, we can't split - just save the paper instead
                Task::none()
            }
            Message::CloseFocusedPane => {
                if let Some(focus) = self.focus {
                    if let Some(_) = self.panes.close(focus) {
                        self.focus = None;
                    }
                }
                Task::none()
            }
            Message::SplitHorizontal => {
                if let Some(focus) = self.focus {
                    let new_pane = Pane {
                        pane_type: PaneType::Search,
                        title: "Search".to_string(),
                    };
                    let _ = self.panes.split(pane_grid::Axis::Horizontal, focus, new_pane);
                }
                Task::none()
            }
            Message::SplitVertical => {
                if let Some(focus) = self.focus {
                    let new_pane = Pane {
                        pane_type: PaneType::Search,
                        title: "Search".to_string(),
                    };
                    let _ = self.panes.split(pane_grid::Axis::Vertical, focus, new_pane);
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let sidebar = if self.sidebar_visible {
            Some(self.sidebar())
        } else {
            None
        };

        let pane_grid = pane_grid::PaneGrid::new(&self.panes, |_pane, pane_data, _is_maximized| {
            let title_bar = pane_grid::TitleBar::new(text(&pane_data.title).color(GRUVBOX_TEXT))
                .controls(pane_grid::Controls::new(text("×")));
            
            let content = match pane_data.pane_type {
                PaneType::Search => self.search_view(),
                PaneType::Library => self.library_view(),
                PaneType::Downloads => self.downloads_view(),
                PaneType::Settings => self.settings_view(),
                PaneType::PaperView(index) => {
                    if let Some(paper) = self.saved_papers.get(index) {
                        self.paper_view(paper)
                    } else {
                        container(text("Paper not found").color(GRUVBOX_TEXT)).into()
                    }
                }
            };

            pane_grid::Content::new(content)
                .title_bar(title_bar)
        })
        .on_click(Message::PaneClicked)
        .on_resize(10, Message::PaneResized)
        .on_drag(Message::PaneDragged)
        .spacing(4);

        let main_content = container(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| iced::widget::container::Style {
                background: Some(Background::Color(GRUVBOX_BG)),
                border: Border::default(),
                text_color: Some(GRUVBOX_TEXT),
                shadow: Shadow::default(),
            });

        if let Some(sidebar) = sidebar {
            row![sidebar, main_content]
                .spacing(0)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        } else {
            main_content.into()
        }
    }

    fn sidebar(&self) -> Element<Message> {
        let toggle_button = button(text("☰").color(GRUVBOX_TEXT))
            .on_press(Message::SidebarToggled)
            .style(button_secondary_style);

        let pane_controls = column![
            button(text("Split Horizontal").color(GRUVBOX_TEXT))
                .on_press(Message::SplitHorizontal)
                .style(button_secondary_style),
            button(text("Split Vertical").color(GRUVBOX_TEXT))
                .on_press(Message::SplitVertical)
                .style(button_secondary_style),
            button(text("Close Pane").color(Color::WHITE))
                .on_press(Message::CloseFocusedPane)
                .style(button_danger_style),
        ]
        .spacing(8);

        let saved_papers_list = scrollable(
            column(
                self.saved_papers.iter().map(|paper| {
                    button(text(&paper.title).color(GRUVBOX_TEXT))
                        .on_press(Message::OpenPaperPane(paper.clone()))
                        .width(Length::Fill)
                        .style(button_secondary_style)
                        .into()
                }).collect::<Vec<Element<Message>>>()
            ).spacing(4)
        );

        container(
            column![
                toggle_button,
                horizontal_rule(2),
                text("Pane Controls").color(GRUVBOX_TEXT).size(16),
                pane_controls,
                horizontal_rule(2),
                text("Saved Papers").color(GRUVBOX_TEXT).size(16),
                saved_papers_list,
            ]
            .spacing(16)
            .padding(16)
        )
        .width(280)
        .height(Length::Fill)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_SURFACE)),
            border: Border {
                color: GRUVBOX_BORDER,
                width: 1.0,
                radius: 0.0.into(),
            },
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    fn search_view(&self) -> Element<Message> {
        let search_input = text_input("Search arXiv papers...", &self.search_query)
            .on_input(Message::SearchQueryChanged)
            .on_submit(Message::SearchSubmitted)
            .style(|_theme, status| iced::widget::text_input::Style {
                background: Background::Color(GRUVBOX_SURFACE),
                border: Border {
                    color: match status {
                        iced::widget::text_input::Status::Focused => GRUVBOX_GREEN,
                        _ => GRUVBOX_BORDER,
                    },
                    width: 1.0,
                    radius: 4.0.into(),
                },
                icon: Color::TRANSPARENT,
                placeholder: GRUVBOX_TEXT_MUTED,
                value: GRUVBOX_TEXT,
                selection: GRUVBOX_GREEN,
            });

        let search_button = button(text("Search").color(Color::BLACK))
            .on_press(Message::SearchSubmitted)
            .style(button_primary_style);

        let search_bar = row![search_input, search_button]
            .spacing(10)
            .padding(10);

        let content = if self.is_searching {
            column![text("Searching...").color(GRUVBOX_TEXT)]
        } else if let Some(error) = &self.search_error {
            column![
                text("Error:").color(GRUVBOX_RED),
                text(error).color(GRUVBOX_TEXT)
            ]
        } else if self.search_results.is_empty() {
            column![text("No results").color(GRUVBOX_TEXT_MUTED)]
        } else {
            column(
                self.search_results.iter().map(|paper| {
                    self.paper_card(paper, false)
                }).collect::<Vec<Element<Message>>>()
            ).spacing(10)
        };

        container(
            column![
                search_bar,
                horizontal_rule(1),
                scrollable(content).height(Length::Fill)
            ]
        )
        .padding(20)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_BG)),
            border: Border::default(),
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    fn library_view(&self) -> Element<Message> {
        let content = if self.saved_papers.is_empty() {
            column![text("No saved papers").color(GRUVBOX_TEXT_MUTED)]
        } else {
            column(
                self.saved_papers.iter().map(|paper| {
                    self.paper_card(paper, true)
                }).collect::<Vec<Element<Message>>>()
            ).spacing(10)
        };

        container(
            scrollable(content).height(Length::Fill)
        )
        .padding(20)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_BG)),
            border: Border::default(),
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    fn downloads_view(&self) -> Element<Message> {
        let content = if self.downloads.is_empty() {
            column![text("No downloads").color(GRUVBOX_TEXT_MUTED)]
        } else {
            column(
                self.downloads.iter().map(|download| {
                    self.download_card(download)
                }).collect::<Vec<Element<Message>>>()
            ).spacing(10)
        };

        container(
            scrollable(content).height(Length::Fill)
        )
        .padding(20)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_BG)),
            border: Border::default(),
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    fn settings_view(&self) -> Element<Message> {
        container(
            column![
                text("Settings").color(GRUVBOX_TEXT).size(24),
                vertical_space().height(20),
                text("Theme: Gruvbox Dark").color(GRUVBOX_TEXT),
            ]
        )
        .padding(20)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_BG)),
            border: Border::default(),
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    fn paper_view<'a>(&'a self, paper: &'a ArxivPaper) -> Element<'a, Message> {
        let title = text(&paper.title)
            .color(GRUVBOX_TEXT)
            .size(24);

        let authors = text(paper.authors.join(", "))
            .color(GRUVBOX_TEXT_MUTED)
            .size(14);

        let published = text(format!("Published: {}", paper.published))
            .color(GRUVBOX_TEXT_MUTED)
            .size(12);

        let categories = text(format!("Categories: {}", paper.categories.join(", ")))
            .color(GRUVBOX_TEXT_MUTED)
            .size(12);

        let abstract_text = text(&paper.abstract_text)
            .color(GRUVBOX_TEXT)
            .size(14);

        container(
            scrollable(
                column![
                    title,
                    authors,
                    published,
                    categories,
                    horizontal_rule(1),
                    text("Abstract").color(GRUVBOX_TEXT).size(18),
                    abstract_text,
                ]
                .spacing(10)
            )
        )
        .padding(20)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_BG)),
            border: Border::default(),
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    fn paper_card<'a>(&'a self, paper: &'a ArxivPaper, is_saved: bool) -> Element<'a, Message> {
        let title = text(&paper.title)
            .color(GRUVBOX_TEXT)
            .size(16);

        let authors = text(paper.authors.join(", "))
            .color(GRUVBOX_TEXT_MUTED)
            .size(12);

        let buttons = if is_saved {
            row![
                button(text("Remove").color(Color::WHITE))
                    .on_press(Message::RemovePaper(paper.id.clone()))
                    .style(button_danger_style),
                button(text("Download").color(Color::BLACK))
                    .on_press(Message::DownloadPaper(paper.clone()))
                    .style(button_primary_style),
                button(text("View").color(GRUVBOX_TEXT))
                    .on_press(Message::OpenPaperPane(paper.clone()))
                    .style(button_secondary_style),
            ]
        } else {
            row![
                button(text("Save").color(Color::BLACK))
                    .on_press(Message::SavePaper(paper.clone()))
                    .style(button_primary_style),
                button(text("Download").color(GRUVBOX_TEXT))
                    .on_press(Message::DownloadPaper(paper.clone()))
                    .style(button_secondary_style),
            ]
        }
        .spacing(8);

        container(
            column![
                title,
                authors,
                vertical_space().height(8),
                buttons,
            ]
            .spacing(4)
        )
        .padding(12)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_SURFACE)),
            border: Border {
                color: GRUVBOX_BORDER,
                width: 1.0,
                radius: 8.0.into(),
            },
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    fn download_card<'a>(&'a self, download: &'a DownloadItem) -> Element<'a, Message> {
        let title = text(&download.title)
            .color(GRUVBOX_TEXT)
            .size(14);

        let status_text = match &download.status {
            DownloadStatus::Pending => "Pending",
            DownloadStatus::Downloading => "Downloading",
            DownloadStatus::Completed => "Completed",
            DownloadStatus::Failed(_) => "Failed",
        };

        let status = text(status_text)
            .color(match download.status {
                DownloadStatus::Failed(_) => GRUVBOX_RED,
                DownloadStatus::Completed => GRUVBOX_GREEN,
                _ => GRUVBOX_TEXT_MUTED,
            })
            .size(12);

        let progress = if matches!(download.status, DownloadStatus::Downloading) {
            Some(progress_bar(0.0..=100.0, download.progress))
        } else {
            None
        };

        let mut content = column![title, status].spacing(4);
        
        if let Some(progress_bar) = progress {
            content = content.push(progress_bar);
        }

        container(content)
            .padding(12)
            .style(|_theme| iced::widget::container::Style {
                background: Some(Background::Color(GRUVBOX_SURFACE)),
                border: Border {
                    color: GRUVBOX_BORDER,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                text_color: Some(GRUVBOX_TEXT),
                shadow: Shadow::default(),
            })
            .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}

// 按钮样式函数
fn button_primary_style(_theme: &iced::Theme, status: iced::widget::button::Status) -> iced::widget::button::Style {
    let (background, text_color) = match status {
        iced::widget::button::Status::Hovered => (GRUVBOX_GREEN, Color::BLACK),
        _ => (GRUVBOX_GREEN, Color::BLACK),
    };
    
    iced::widget::button::Style {
        background: Some(Background::Color(background)),
        text_color,
        border: Border {
            color: GRUVBOX_GREEN,
            width: 1.0,
            radius: 4.0.into(),
        },
        shadow: Shadow::default(),
    }
}

fn button_secondary_style(_theme: &iced::Theme, status: iced::widget::button::Status) -> iced::widget::button::Style {
    let background = match status {
        iced::widget::button::Status::Hovered => Color::from_rgb(0.45, 0.41, 0.35),
        _ => GRUVBOX_SURFACE,
    };
    
    iced::widget::button::Style {
        background: Some(Background::Color(background)),
        text_color: GRUVBOX_TEXT,
        border: Border {
            color: GRUVBOX_BORDER,
            width: 1.0,
            radius: 4.0.into(),
        },
        shadow: Shadow::default(),
    }
}

fn button_danger_style(_theme: &iced::Theme, status: iced::widget::button::Status) -> iced::widget::button::Style {
    let background = match status {
        iced::widget::button::Status::Hovered => Color::from_rgb(0.85, 0.3, 0.3),
        _ => GRUVBOX_RED,
    };
    
    iced::widget::button::Style {
        background: Some(Background::Color(background)),
        text_color: Color::WHITE,
        border: Border {
            color: GRUVBOX_RED,
            width: 1.0,
            radius: 4.0.into(),
        },
        shadow: Shadow::default(),
    }
}

// 异步函数实现
async fn search_arxiv_papers(query: String) -> Result<Vec<ArxivPaper>, String> {
    // 模拟 API 调用
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    
    // 返回模拟结果
    Ok(vec![
        ArxivPaper {
            id: "2301.00001".to_string(),
            title: format!("Sample Paper about {}", query),
            authors: vec!["John Doe".to_string(), "Jane Smith".to_string()],
            abstract_text: "This is a sample abstract for demonstration purposes. It contains detailed information about the research methodology and findings.".to_string(),
            published: "2023-01-01".to_string(),
            updated: "2023-01-01".to_string(),
            categories: vec!["cs.LG".to_string(), "stat.ML".to_string()],
            pdf_url: "https://arxiv.org/pdf/2301.00001.pdf".to_string(),
            entry_url: "https://arxiv.org/abs/2301.00001".to_string(),
        },
        ArxivPaper {
            id: "2301.00002".to_string(),
            title: format!("Another Research on {}", query),
            authors: vec!["Alice Johnson".to_string(), "Bob Wilson".to_string()],
            abstract_text: "Another comprehensive study exploring different aspects of the research topic with novel approaches and insights.".to_string(),
            published: "2023-01-02".to_string(),
            updated: "2023-01-02".to_string(),
            categories: vec!["cs.AI".to_string(), "cs.CV".to_string()],
            pdf_url: "https://arxiv.org/pdf/2301.00002.pdf".to_string(),
            entry_url: "https://arxiv.org/abs/2301.00002".to_string(),
        }
    ])
}

async fn download_pdf(paper: ArxivPaper) -> Result<(String, PathBuf), (String, String)> {
    // 模拟下载延迟
    tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
    
    let file_path = PathBuf::from(format!("downloads/{}.pdf", paper.id));
    Ok((paper.id, file_path))
}

use iced::widget::{container, column, row, button, text};
use iced::{Task, Element, Length, Theme};

use crate::app::TabId;

pub struct ArxivManagerApp {
    state: AppState,
}

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(TabId),
    SearchQueryChanged(String),
    SearchSubmitted,
    ThemeToggled,
}

impl ArxivManagerApp {
    pub fn new() -> (Self, Task<Message>) {
        // Create a simple state for now
        let state = AppState {
            active_tab: TabId::Search,
            search_query: String::new(),
            search_results: Vec::new(),
            is_searching: false,
            theme_dark: true,
        };
        
        (Self { state }, Task::none())
    }
    
    pub fn title(&self) -> String {
        "arXiv 论文管理器".to_string()
    }
    
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TabSelected(tab) => {
                self.state.active_tab = tab;
            }
            Message::SearchQueryChanged(query) => {
                self.state.search_query = query;
            }
            Message::SearchSubmitted => {
                if !self.state.search_query.trim().is_empty() {
                    self.state.is_searching = true;
                    // Simulate search completion
                    self.state.search_results = vec![
                        "论文示例 1: Attention Is All You Need".to_string(),
                        "论文示例 2: BERT: Pre-training Transformers".to_string(),
                        format!("搜索结果: '{}'", self.state.search_query),
                    ];
                    self.state.is_searching = false;
                }
            }
            Message::ThemeToggled => {
                self.state.theme_dark = !self.state.theme_dark;
            }
        }
        
        Task::none()
    }
    
    pub fn view(&self) -> Element<Message> {
        let tab_bar = row![
            tab_button("搜索", TabId::Search, self.state.active_tab),
            tab_button("论文库", TabId::Library, self.state.active_tab),
            tab_button("下载", TabId::Downloads, self.state.active_tab),
            tab_button("设置", TabId::Settings, self.state.active_tab),
        ]
        .spacing(5)
        .padding(10);
        
        let main_content = match self.state.active_tab {
            TabId::Search => search_page(&self.state),
            TabId::Library => library_page(),
            TabId::Downloads => downloads_page(),
            TabId::Settings => settings_page(&self.state),
        };
        
        column![
            container(tab_bar),
            container(main_content)
                .height(Length::Fill)
                .width(Length::Fill)
        ]
        .into()
    }
    
    pub fn theme(&self) -> Theme {
        if self.state.theme_dark {
            Theme::Dark
        } else {
            Theme::Light
        }
    }
}

// Simplified AppState for demo purposes
#[derive(Debug, Clone)]
pub struct AppState {
    pub active_tab: TabId,
    pub search_query: String,
    pub search_results: Vec<String>, // Simplified for now
    pub is_searching: bool,
    pub theme_dark: bool,
}

fn tab_button(label: &str, tab_id: TabId, active_tab: TabId) -> Element<Message> {
    let mut btn = button(text(label).size(16))
        .padding([10, 20]);
    
    if tab_id == active_tab {
        // Active tab styling would go here in a real implementation
    }
    
    btn = btn.on_press(Message::TabSelected(tab_id));
    btn.into()
}

fn search_page(state: &AppState) -> Element<Message> {
    let search_input = iced::widget::text_input("输入搜索关键词...", &state.search_query)
        .on_input(Message::SearchQueryChanged)
        .on_submit(Message::SearchSubmitted)
        .padding(10)
        .size(16);
    
    let search_button = button(text("搜索").size(16))
        .on_press(Message::SearchSubmitted)
        .padding([10, 20]);
    
    let search_row = row![search_input, search_button]
        .spacing(10);
    
    let results_area = if state.is_searching {
        column![text("搜索中...").size(16)]
    } else if state.search_results.is_empty() {
        column![
            text("欢迎使用 arXiv 论文管理器！").size(20),
            text("在上方输入关键词开始搜索论文").size(14),
        ]
    } else {
        let mut results = column![];
        for result in &state.search_results {
            results = results.push(text(result).size(14));
        }
        results
    };
    
    column![
        text("论文搜索").size(24),
        search_row,
        container(results_area)
            .padding(20)
            .height(Length::Fill)
    ]
    .spacing(20)
    .padding(20)
    .into()
}

fn library_page() -> Element<'static, Message> {
    column![
        text("论文库").size(24),
        text("您的论文收藏将显示在这里").size(16),
        text("• 已下载的论文").size(14),
        text("• 收藏的论文").size(14),
        text("• 阅读进度").size(14),
        text("• 标签和分类").size(14),
    ]
    .spacing(15)
    .padding(20)
    .into()
}

fn downloads_page() -> Element<'static, Message> {
    column![
        text("下载管理").size(24),
        text("下载队列和进度将显示在这里").size(16),
        text("• 正在下载的文件").size(14),
        text("• 下载队列").size(14),
        text("• 下载历史").size(14),
        text("• 失败重试").size(14),
    ]
    .spacing(15)
    .padding(20)
    .into()
}

fn settings_page(state: &AppState) -> Element<Message> {
    let theme_toggle = button(
        text(if state.theme_dark { "切换到浅色主题" } else { "切换到深色主题" }).size(14)
    )
    .on_press(Message::ThemeToggled)
    .padding([8, 16]);
    
    column![
        text("设置").size(24),
        text("应用程序设置").size(16),
        row![text("主题:").size(14), theme_toggle].spacing(10),
        text("• 下载设置").size(14),
        text("• 界面设置").size(14),
        text("• 数据库设置").size(14),
        text("• 快捷键设置").size(14),
    ]
    .spacing(15)
    .padding(20)
    .into()
}

pub fn run() -> iced::Result {
    env_logger::init();
    
    fn title(_app: &ArxivManagerApp) -> String {
        "arXiv 论文管理器".to_string()
    }
    
    fn update(app: &mut ArxivManagerApp, message: Message) -> Task<Message> {
        app.update(message)
    }
    
    fn view(app: &ArxivManagerApp) -> Element<Message> {
        app.view()
    }
    
    fn theme(app: &ArxivManagerApp) -> Theme {
        app.theme()
    }
    
    iced::application(title, update, view)
        .theme(theme)
        .window_size((1200.0, 800.0))
        .centered()
        .run_with(ArxivManagerApp::new)
}

// UI相关的数据模型

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PaneType {
    Search,
    Library,
    Downloads,
    Settings,
    PaperView(usize),
}

#[derive(Clone, Debug)]
pub struct Pane {
    pub pane_type: PaneType,
    pub title: String,
}

// 标签页相关结构
#[derive(Debug, Clone, PartialEq)]
pub struct Tab {
    pub id: usize,
    pub title: String,
    pub content: TabContent,
    pub closable: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TabContent {
    Search,
    Library,
    Downloads,
    Settings,
    PaperView(usize), // Index into saved_papers
}

impl Tab {
    pub fn new(id: usize, title: String, content: TabContent) -> Self {
        let closable = !matches!(content, TabContent::Search | TabContent::Library | TabContent::Downloads | TabContent::Settings);
        Self {
            id,
            title,
            content,
            closable,
        }
    }
}

// 主题相关
#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    GruvboxDark,
    GruvboxLight,
    Dark,
    Light,
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl Theme {
    pub fn display_name(&self) -> &'static str {
        match self {
            Theme::GruvboxDark => "Gruvbox Dark",
            Theme::GruvboxLight => "Gruvbox Light",
            Theme::Dark => "Dark",
            Theme::Light => "Light",
        }
    }

    pub fn all_variants() -> Vec<Self> {
        vec![
            Theme::GruvboxDark,
            Theme::GruvboxLight,
            Theme::Dark,
            Theme::Light,
        ]
    }
}

// 语言设置
#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    English,
    Chinese,
    Japanese,
    German,
    French,
    Spanish,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl Language {
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Chinese => "中文",
            Language::Japanese => "日本語",
            Language::German => "Deutsch",
            Language::French => "Français",
            Language::Spanish => "Español",
        }
    }

    pub fn all_variants() -> Vec<Self> {
        vec![
            Language::English,
            Language::Chinese,
            Language::Japanese,
            Language::German,
            Language::French,
            Language::Spanish,
        ]
    }
}

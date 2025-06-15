// UI相关的数据模型

use serde::{Deserialize, Serialize};

// UI相关的数据模型
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PaneType {
    Search,
    Library,
    Downloads,
    Settings,
    PaperView(usize),
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Pane {
    pub pane_type: PaneType,
    pub title: String,
}

// 标签页分组
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TabGroup {
    Default,
    Research,    // 研究相关
    Library,     // 图书馆相关
    Downloads,   // 下载相关
    Custom(String), // 自定义分组
}

impl TabGroup {
    pub fn display_name(&self) -> &str {
        match self {
            TabGroup::Default => "默认",
            TabGroup::Research => "研究",
            TabGroup::Library => "图书馆",
            TabGroup::Downloads => "下载",
            TabGroup::Custom(name) => name,
        }
    }
}

// 标签页相关结构
#[derive(Debug, Clone, PartialEq)]
pub struct Tab {
    pub id: usize,
    pub title: String,
    pub content: TabContent,
    pub closable: bool,
    pub pinned: bool,        // 是否固定
    pub group: TabGroup,     // 所属分组
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TabContent {
    Search,
    Library,
    Downloads,
    Settings,
    PaperView(usize), // Index into saved_papers
}

impl Tab {
    pub fn new(id: usize, title: String, content: TabContent) -> Self {
        let closable = true; // 所有标签页默认都可以关闭，用户可以通过固定来保护重要标签页
        let pinned = false; // 默认不固定任何标签页
        let group = match &content {
            TabContent::Search => TabGroup::Default,
            TabContent::Library => TabGroup::Library,
            TabContent::Downloads => TabGroup::Downloads,
            TabContent::Settings => TabGroup::Default,
            TabContent::PaperView(_) => TabGroup::Research,
        };
        
        Self {
            id,
            title,
            content,
            closable,
            pinned,
            group,
        }
    }
    
    pub fn new_with_group(id: usize, title: String, content: TabContent, group: TabGroup) -> Self {
        let mut tab = Self::new(id, title, content);
        tab.group = group;
        tab
    }
    
    pub fn pin(&mut self) {
        self.pinned = true;
        self.closable = false;
    }
    
    pub fn unpin(&mut self) {
        self.pinned = false;
        // 取消固定后，除了被固定的标签页，其他都可以关闭
        self.closable = true;
    }
}

// 主题相关
#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    // 现代暗色主题
    ModernDark,
    ModernLight,
    
    // Gruvbox 主题系列
    GruvboxDark,
    GruvboxLight,
    GruvboxMaterial,
    
    // Catppuccin 主题系列  
    CatppuccinMocha,
    CatppuccinMacchiato,
    CatppuccinFrappe,
    CatppuccinLatte,
    
    // Solarized 主题系列
    SolarizedDark,
    SolarizedLight,
    
    // Dracula 主题
    Dracula,
    
    // Nord 主题
    Nord,
    NordLight,
    
    // One 主题系列
    OneDark,
    OneLight,
    
    // GitHub 主题系列
    GitHubDark,
    GitHubLight,
    
    // 经典主题
    Dark,
    Light,
    
    // Tokyo Night 主题系列
    TokyoNight,
    TokyoNightLight,
    
    // Ayu 主题系列
    AyuDark,
    AyuMirage,
    AyuLight,
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl Theme {
    pub fn display_name(&self) -> &'static str {
        match self {
            // 现代主题
            Theme::ModernDark => "Modern Dark",
            Theme::ModernLight => "Modern Light",
            
            // Gruvbox 主题系列
            Theme::GruvboxDark => "Gruvbox Dark",
            Theme::GruvboxLight => "Gruvbox Light", 
            Theme::GruvboxMaterial => "Gruvbox Material",
            
            // Catppuccin 主题系列
            Theme::CatppuccinMocha => "Catppuccin Mocha",
            Theme::CatppuccinMacchiato => "Catppuccin Macchiato",
            Theme::CatppuccinFrappe => "Catppuccin Frappé",
            Theme::CatppuccinLatte => "Catppuccin Latte",
            
            // Solarized 主题系列
            Theme::SolarizedDark => "Solarized Dark",
            Theme::SolarizedLight => "Solarized Light",
            
            // Dracula 主题
            Theme::Dracula => "Dracula",
            
            // Nord 主题
            Theme::Nord => "Nord",
            Theme::NordLight => "Nord Light",
            
            // One 主题系列
            Theme::OneDark => "One Dark",
            Theme::OneLight => "One Light",
            
            // GitHub 主题系列
            Theme::GitHubDark => "GitHub Dark",
            Theme::GitHubLight => "GitHub Light",
            
            // 经典主题
            Theme::Dark => "Dark",
            Theme::Light => "Light",
            
            // Tokyo Night 主题系列
            Theme::TokyoNight => "Tokyo Night",
            Theme::TokyoNightLight => "Tokyo Night Light",
            
            // Ayu 主题系列
            Theme::AyuDark => "Ayu Dark",
            Theme::AyuMirage => "Ayu Mirage",
            Theme::AyuLight => "Ayu Light",
        }
    }

    pub fn all_variants() -> Vec<Self> {
        vec![
            // 现代主题 (默认)
            Theme::ModernDark,
            Theme::ModernLight,
            
            // Gruvbox 主题系列
            Theme::GruvboxDark,
            Theme::GruvboxLight,
            Theme::GruvboxMaterial,
            
            // Catppuccin 主题系列  
            Theme::CatppuccinMocha,
            Theme::CatppuccinMacchiato,
            Theme::CatppuccinFrappe,
            Theme::CatppuccinLatte,
            
            // Solarized 主题系列
            Theme::SolarizedDark,
            Theme::SolarizedLight,
            
            // Dracula 主题
            Theme::Dracula,
            
            // Nord 主题
            Theme::Nord,
            Theme::NordLight,
            
            // One 主题系列
            Theme::OneDark,
            Theme::OneLight,
            
            // GitHub 主题系列
            Theme::GitHubDark,
            Theme::GitHubLight,
            
            // 经典主题
            Theme::Dark,
            Theme::Light,
            
            // Tokyo Night 主题系列
            Theme::TokyoNight,
            Theme::TokyoNightLight,
            
            // Ayu 主题系列
            Theme::AyuDark,
            Theme::AyuMirage,
            Theme::AyuLight,
        ]
    }
    
    /// 获取主题类别，用于分组显示
    pub fn category(&self) -> &'static str {
        match self {
            Theme::ModernDark | Theme::ModernLight => "Modern",
            Theme::GruvboxDark | Theme::GruvboxLight | Theme::GruvboxMaterial => "Gruvbox",
            Theme::CatppuccinMocha | Theme::CatppuccinMacchiato | Theme::CatppuccinFrappe | Theme::CatppuccinLatte => "Catppuccin",
            Theme::SolarizedDark | Theme::SolarizedLight => "Solarized",
            Theme::Dracula => "Dracula",
            Theme::Nord | Theme::NordLight => "Nord",
            Theme::OneDark | Theme::OneLight => "One",
            Theme::GitHubDark | Theme::GitHubLight => "GitHub",
            Theme::Dark | Theme::Light => "Classic",
            Theme::TokyoNight | Theme::TokyoNightLight => "Tokyo Night",
            Theme::AyuDark | Theme::AyuMirage | Theme::AyuLight => "Ayu",
        }
    }
    
    /// 检查是否为暗色主题
    pub fn is_dark(&self) -> bool {
        match self {
            Theme::ModernDark | Theme::GruvboxDark | Theme::GruvboxMaterial |
            Theme::CatppuccinMocha | Theme::CatppuccinMacchiato | Theme::CatppuccinFrappe |
            Theme::SolarizedDark | Theme::Dracula | Theme::Nord | Theme::OneDark |
            Theme::GitHubDark | Theme::Dark | Theme::TokyoNight | Theme::AyuDark | Theme::AyuMirage => true,
            _ => false,
        }
    }
}

// 语言设置
#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    English,
    Chinese,
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
        }
    }

    pub fn all_variants() -> Vec<Self> {
        vec![
            Language::English,
            Language::Chinese,
        ]
    }
}

// Library视图相关的排序和显示选项
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LibrarySortBy {
    Title,           // 按标题排序
    Author,          // 按作者排序
    PublishDate,     // 按发布时间排序
    AddedDate,       // 按添加时间排序
    Category,        // 按分类排序
    Relevance,       // 按相关性排序
}

impl LibrarySortBy {
    pub fn display_name(&self) -> &str {
        match self {
            LibrarySortBy::Title => "Title",
            LibrarySortBy::Author => "Author", 
            LibrarySortBy::PublishDate => "Publish Date",
            LibrarySortBy::AddedDate => "Added Date",
            LibrarySortBy::Category => "Category",
            LibrarySortBy::Relevance => "Relevance",
        }
    }

    pub fn all_variants() -> Vec<Self> {
        vec![
            LibrarySortBy::Title,
            LibrarySortBy::Author,
            LibrarySortBy::PublishDate,
            LibrarySortBy::AddedDate,
            LibrarySortBy::Category,
            LibrarySortBy::Relevance,
        ]
    }
}

impl std::fmt::Display for LibrarySortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LibraryGroupBy {
    None,            // 不分组
    Author,          // 按作者分组
    Category,        // 按分类分组
    PublishYear,     // 按发布年份分组
    AddedDate,       // 按添加日期分组
    Tag,             // 按标签分组
}

impl LibraryGroupBy {
    pub fn display_name(&self) -> &str {
        match self {
            LibraryGroupBy::None => "No Grouping",
            LibraryGroupBy::Author => "By Author",
            LibraryGroupBy::Category => "By Category", 
            LibraryGroupBy::PublishYear => "By Year",
            LibraryGroupBy::AddedDate => "By Added Date",
            LibraryGroupBy::Tag => "By Tag",
        }
    }

    pub fn all_variants() -> Vec<Self> {
        vec![
            LibraryGroupBy::None,
            LibraryGroupBy::Author,
            LibraryGroupBy::Category,
            LibraryGroupBy::PublishYear,
            LibraryGroupBy::AddedDate,
            LibraryGroupBy::Tag,
        ]
    }
}

impl std::fmt::Display for LibraryGroupBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LibraryViewMode {
    Waterfall,       // 瀑布流视图（唯一选项）
}

impl LibraryViewMode {
    pub fn display_name(&self) -> &str {
        match self {
            LibraryViewMode::Waterfall => "Waterfall",
        }
    }

    pub fn all_variants() -> Vec<Self> {
        vec![
            LibraryViewMode::Waterfall,
        ]
    }
}

impl Default for LibraryViewMode {
    fn default() -> Self {
        LibraryViewMode::Waterfall
    }
}

impl std::fmt::Display for LibraryViewMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

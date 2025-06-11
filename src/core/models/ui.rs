// UI相关的数据模型

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

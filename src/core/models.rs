// arXiv 管理器数据模型

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ArxivPaper {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub abstract_text: String,
    pub published: String,
    pub updated: String,
    pub categories: Vec<String>,
    pub pdf_url: String,
    pub entry_url: String,
    pub doi: Option<String>,
    pub journal_ref: Option<String>,
    pub comments: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DownloadItem {
    pub paper_id: String,
    pub title: String,
    pub progress: f32,
    pub status: DownloadStatus,
    pub file_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Completed,
    Failed(String),
}

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

// 高级搜索配置
#[derive(Debug, Clone)]
pub struct SearchConfig {
    pub query: String,
    pub search_in: SearchField,
    pub categories: Vec<String>,
    pub date_range: DateRange,
    pub sort_by: SortBy,
    pub sort_order: SortOrder,
    pub max_results: u32,
    pub authors: Vec<String>,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self {
            query: String::new(),
            search_in: SearchField::All,
            categories: Vec::new(),
            date_range: DateRange::Any,
            sort_by: SortBy::Relevance,
            sort_order: SortOrder::Descending,
            max_results: 20,
            authors: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchField {
    All,
    Title,
    Abstract,
    Authors,
    Comments,
}

impl std::fmt::Display for SearchField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl SearchField {
    pub fn as_str(&self) -> &'static str {
        match self {
            SearchField::All => "all",
            SearchField::Title => "ti",
            SearchField::Abstract => "abs",
            SearchField::Authors => "au",
            SearchField::Comments => "co",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            SearchField::All => "All Fields",
            SearchField::Title => "Title",
            SearchField::Abstract => "Abstract",
            SearchField::Authors => "Authors",
            SearchField::Comments => "Comments",
        }
    }

    pub fn all_variants() -> Vec<Self> {
        vec![
            SearchField::All,
            SearchField::Title,
            SearchField::Abstract,
            SearchField::Authors,
            SearchField::Comments,
        ]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DateRange {
    Any,
    LastWeek,
    LastMonth,
    LastYear,
    Custom { from: String, to: String },
}

impl std::fmt::Display for DateRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl DateRange {
    pub fn display_name(&self) -> String {
        match self {
            DateRange::Any => "Any Date".to_string(),
            DateRange::LastWeek => "Last Week".to_string(),
            DateRange::LastMonth => "Last Month".to_string(),
            DateRange::LastYear => "Last Year".to_string(),
            DateRange::Custom { from, to } => format!("{} to {}", from, to),
        }
    }

    pub fn all_variants() -> Vec<Self> {
        vec![
            DateRange::Any,
            DateRange::LastWeek,
            DateRange::LastMonth,
            DateRange::LastYear,
        ]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortBy {
    Relevance,
    SubmittedDate,
    LastUpdatedDate,
}

impl std::fmt::Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl SortBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortBy::Relevance => "relevance",
            SortBy::SubmittedDate => "submittedDate",
            SortBy::LastUpdatedDate => "lastUpdatedDate",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            SortBy::Relevance => "Relevance",
            SortBy::SubmittedDate => "Submitted Date",
            SortBy::LastUpdatedDate => "Last Updated",
        }
    }

    pub fn all_variants() -> Vec<Self> {
        vec![
            SortBy::Relevance,
            SortBy::SubmittedDate,
            SortBy::LastUpdatedDate,
        ]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl SortOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortOrder::Ascending => "ascending",
            SortOrder::Descending => "descending",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            SortOrder::Ascending => "Ascending",
            SortOrder::Descending => "Descending",
        }
    }

    pub fn all_variants() -> Vec<Self> {
        vec![SortOrder::Ascending, SortOrder::Descending]
    }
}

// 常用的arXiv分类 - 为理论物理工作者优化
pub const ARXIV_CATEGORIES: &[(&str, &str)] = &[
    // 物理学 - 理论物理核心领域
    ("hep-th", "High Energy Physics - Theory"),
    ("hep-ph", "High Energy Physics - Phenomenology"),
    ("hep-lat", "High Energy Physics - Lattice"),
    ("hep-ex", "High Energy Physics - Experiment"),
    ("gr-qc", "General Relativity and Quantum Cosmology"),
    ("quant-ph", "Quantum Physics"),
    ("nucl-th", "Nuclear Theory"),
    ("nucl-ex", "Nuclear Experiment"),
    ("math-ph", "Mathematical Physics"),
    ("nlin.SI", "Exactly Solvable and Integrable Systems"),
    ("nlin.CD", "Chaotic Dynamics"),
    ("nlin.PS", "Pattern Formation and Solitons"),
    ("physics.class-ph", "Classical Physics"),
    ("physics.gen-ph", "General Physics"),
    
    // 天体物理学
    ("astro-ph.CO", "Cosmology and Nongalactic Astrophysics"),
    ("astro-ph.GA", "Astrophysics of Galaxies"),
    ("astro-ph.HE", "High Energy Astrophysical Phenomena"),
    ("astro-ph.IM", "Instrumentation and Methods for Astrophysics"),
    ("astro-ph.SR", "Solar and Stellar Astrophysics"),
    ("astro-ph.EP", "Earth and Planetary Astrophysics"),
    
    // 凝聚态物理
    ("cond-mat.str-el", "Strongly Correlated Electrons"),
    ("cond-mat.mes-hall", "Mesoscale and Nanoscale Physics"),
    ("cond-mat.stat-mech", "Statistical Mechanics"),
    ("cond-mat.supr-con", "Superconductivity"),
    ("cond-mat.quant-gas", "Quantum Gases"),
    ("cond-mat.dis-nn", "Disordered Systems and Neural Networks"),
    ("cond-mat.mtrl-sci", "Materials Science"),
    ("cond-mat.other", "Other Condensed Matter"),
    ("cond-mat.soft", "Soft Condensed Matter"),
    
    // 数学 - 理论物理相关
    ("math.DG", "Differential Geometry"),
    ("math.AG", "Algebraic Geometry"),
    ("math.AT", "Algebraic Topology"),
    ("math.GT", "Geometric Topology"),
    ("math.SG", "Symplectic Geometry"),
    ("math.RT", "Representation Theory"),
    ("math.QA", "Quantum Algebra"),
    ("math.KT", "K-Theory and Homology"),
    ("math.CT", "Category Theory"),
    ("math.LO", "Logic"),
    ("math.NT", "Number Theory"),
    ("math.AP", "Analysis of PDEs"),
    ("math.DS", "Dynamical Systems"),
    ("math.MP", "Mathematical Physics"),
    ("math.PR", "Probability"),
    ("math.ST", "Statistics Theory"),
    ("math.FA", "Functional Analysis"),
    ("math.SP", "Spectral Theory"),
    ("math.OA", "Operator Algebras"),
    ("math.GR", "Group Theory"),
    ("math.RA", "Rings and Algebras"),
    ("math.AC", "Commutative Algebra"),
    ("math.CV", "Complex Variables"),
    ("math.CA", "Classical Analysis and ODEs"),
    ("math.NA", "Numerical Analysis"),
    ("math.OC", "Optimization and Control"),
    ("math.CO", "Combinatorics"),
    ("math.MG", "Metric Geometry"),
    ("math.IT", "Information Theory"),
    
    // 其他物理分支
    ("physics.atom-ph", "Atomic Physics"),
    ("physics.chem-ph", "Chemical Physics"),
    ("physics.flu-dyn", "Fluid Dynamics"),
    ("physics.optics", "Optics"),
    ("physics.plasm-ph", "Plasma Physics"),
    ("physics.space-ph", "Space Physics"),
    ("physics.acc-ph", "Accelerator Physics"),
    ("physics.ao-ph", "Atmospheric and Oceanic Physics"),
    ("physics.bio-ph", "Biological Physics"),
    ("physics.comp-ph", "Computational Physics"),
    ("physics.data-an", "Data Analysis, Statistics and Probability"),
    ("physics.ed-ph", "Physics Education"),
    ("physics.geo-ph", "Geophysics"),
    ("physics.hist-ph", "History and Philosophy of Physics"),
    ("physics.ins-det", "Instrumentation and Detectors"),
    ("physics.med-ph", "Medical Physics"),
    ("physics.pop-ph", "Popular Physics"),
    ("physics.soc-ph", "Physics and Society"),
    
    // 计算机科学 - 理论物理相关
    ("cs.AI", "Artificial Intelligence"),
    ("cs.LG", "Machine Learning"),
    ("cs.NA", "Numerical Analysis"),
    ("cs.CC", "Computational Complexity"),
    ("cs.IT", "Information Theory"),
    ("cs.DM", "Discrete Mathematics"),
    ("cs.SY", "Systems and Control"),
    
    // 统计学
    ("stat.AP", "Applications"),
    ("stat.CO", "Computation"),
    ("stat.ME", "Methodology"),
    ("stat.ML", "Machine Learning"),
    ("stat.TH", "Theory"),
    
    // 量化生物学
    ("q-bio.BM", "Biomolecules"),
    ("q-bio.CB", "Cell Behavior"),
    ("q-bio.GN", "Genomics"),
    ("q-bio.MN", "Molecular Networks"),
    ("q-bio.NC", "Neurons and Cognition"),
    ("q-bio.OT", "Other Quantitative Biology"),
    ("q-bio.PE", "Populations and Evolution"),
    ("q-bio.QM", "Quantitative Methods"),
    ("q-bio.SC", "Subcellular Processes"),
    ("q-bio.TO", "Tissues and Organs"),
    
    // 量化金融
    ("q-fin.CP", "Computational Finance"),
    ("q-fin.EC", "Economics"),
    ("q-fin.GN", "General Finance"),
    ("q-fin.MF", "Mathematical Finance"),
    ("q-fin.PM", "Portfolio Management"),
    ("q-fin.PR", "Pricing of Securities"),
    ("q-fin.RM", "Risk Management"),
    ("q-fin.ST", "Statistical Finance"),
    ("q-fin.TR", "Trading and Market Microstructure"),
    
    // 经济学
    ("econ.EM", "Econometrics"),
    ("econ.GN", "General Economics"),
    ("econ.TH", "Theoretical Economics"),
];

// 应用设置
#[derive(Debug, Clone)]
pub struct AppSettings {
    pub theme: Theme,
    pub download_directory: String,
    pub auto_download: bool,
    pub max_concurrent_downloads: u32,
    pub show_abstracts_in_search: bool,
    pub default_search_field: SearchField,
    pub default_sort_by: SortBy,
    pub default_sort_order: SortOrder,
    pub default_max_results: u32,
    pub auto_save_searches: bool,
    pub notification_enabled: bool,
    pub check_updates: bool,
    pub language: Language,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: Theme::GruvboxDark,
            download_directory: "downloads".to_string(),
            auto_download: false,
            max_concurrent_downloads: 3,
            show_abstracts_in_search: true,
            default_search_field: SearchField::All,
            default_sort_by: SortBy::Relevance,
            default_sort_order: SortOrder::Descending,
            default_max_results: 20,
            auto_save_searches: false,
            notification_enabled: true,
            check_updates: true,
            language: Language::English,
        }
    }
}

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

// 论文相关的数据模型
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArxivPaper {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub abstract_text: String,
    pub published: String,
    #[allow(dead_code)]
    pub updated: String,
    pub categories: Vec<String>,
    #[allow(dead_code)]
    pub pdf_url: String,
    #[allow(dead_code)]
    pub entry_url: String,
    #[allow(dead_code)]
    pub doi: Option<String>,
    #[allow(dead_code)]
    pub journal_ref: Option<String>,
    #[allow(dead_code)]
    pub comments: Option<String>,
    // Library功能相关字段
    pub is_favorite: bool,                          // 是否为收藏
    pub added_at: Option<DateTime<Utc>>,           // 添加到库的时间
    pub collection_ids: Vec<i64>,                  // 所属集合ID列表
    pub tags: Vec<String>,                         // 用户标签
    pub notes: Option<String>,                     // 用户笔记
    pub read_status: ReadingStatus,                // 阅读状态
    pub rating: Option<u8>,                        // 用户评分 (1-5)
    pub local_file_path: Option<String>,           // 本地PDF文件路径
}

/// 阅读状态
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub enum ReadingStatus {
    #[default]
    Unread,        // 未读
    Reading,       // 正在阅读
    Read,         // 已读
    Skipped,      // 跳过
}

impl ReadingStatus {
    pub fn display_name(&self) -> &'static str {
        match self {
            ReadingStatus::Unread => "Unread",
            ReadingStatus::Reading => "Reading",
            ReadingStatus::Read => "Read",
            ReadingStatus::Skipped => "Skipped",
        }
    }
    
    pub fn all_variants() -> Vec<Self> {
        vec![
            ReadingStatus::Unread,
            ReadingStatus::Reading,
            ReadingStatus::Read,
            ReadingStatus::Skipped,
        ]
    }
}

impl ArxivPaper {
    /// 创建新的论文实例，自动设置添加时间
    pub fn new_with_library_data(
        id: String,
        title: String,
        authors: Vec<String>,
        abstract_text: String,
        published: String,
        updated: String,
        categories: Vec<String>,
        pdf_url: String,
        entry_url: String,
        doi: Option<String>,
        journal_ref: Option<String>,
        comments: Option<String>,
    ) -> Self {
        Self {
            id,
            title,
            authors,
            abstract_text,
            published,
            updated,
            categories,
            pdf_url,
            entry_url,
            doi,
            journal_ref,
            comments,
            is_favorite: false,
            added_at: Some(Utc::now()),
            collection_ids: Vec::new(),
            tags: Vec::new(),
            notes: None,
            read_status: ReadingStatus::Unread,
            rating: None,
            local_file_path: None,
        }
    }

    /// 切换收藏状态
    pub fn toggle_favorite(&mut self) {
        self.is_favorite = !self.is_favorite;
    }

    /// 添加到集合
    pub fn add_to_collection(&mut self, collection_id: i64) {
        if !self.collection_ids.contains(&collection_id) {
            self.collection_ids.push(collection_id);
        }
    }

    /// 从集合中移除
    pub fn remove_from_collection(&mut self, collection_id: i64) {
        self.collection_ids.retain(|&id| id != collection_id);
    }

    /// 检查是否属于特定集合
    pub fn belongs_to_collection(&self, collection_id: i64) -> bool {
        self.collection_ids.contains(&collection_id)
    }

    /// 检查是否为未分类（不属于任何用户集合）
    pub fn is_uncategorized(&self) -> bool {
        self.collection_ids.iter().all(|&id| id < 0) // 负数ID为系统集合
    }

    /// 添加标签
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// 移除标签
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    /// 设置评分
    pub fn set_rating(&mut self, rating: Option<u8>) {
        if let Some(r) = rating {
            if r <= 5 {
                self.rating = Some(r);
            }
        } else {
            self.rating = None;
        }
    }

    /// 设置笔记
    pub fn set_notes(&mut self, notes: Option<String>) {
        self.notes = notes;
    }

    /// 设置阅读状态
    pub fn set_read_status(&mut self, status: ReadingStatus) {
        self.read_status = status;
    }

    /// 设置本地文件路径
    pub fn set_local_file_path(&mut self, path: Option<String>) {
        self.local_file_path = path;
    }

    /// 获取本地文件路径
    pub fn get_local_file_path(&self) -> Option<&String> {
        self.local_file_path.as_ref()
    }
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
    Failed(#[allow(dead_code)] String),
}

// 常用的arXiv分类 - 为理论物理工作者优化
#[allow(dead_code)]
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

// 论文相关的数据模型
use std::path::PathBuf;

#[derive(Debug, Clone)]
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

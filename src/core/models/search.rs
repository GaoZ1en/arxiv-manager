// 搜索相关的数据模型 - 增强版
// 支持arXiv原生的高级搜索功能

#[derive(Debug, Clone)]
pub struct SearchConfig {
    pub query: String,
    pub search_in: SearchField,
    pub categories: Vec<ArxivCategory>,
    pub date_range: DateRange,
    pub sort_by: SortBy,
    pub sort_order: SortOrder,
    pub max_results: u32,
    pub authors: Vec<String>,
    // 新增arXiv原生高级选项
    pub exact_phrase: Option<String>,
    pub exclude_words: Vec<String>,
    pub journal_ref: Option<String>,
    pub subject_class: Option<String>,
    pub report_number: Option<String>,
    pub id_list: Vec<String>,
    // 分页支持
    pub start_index: u32,
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
            max_results: 50, // 增加默认搜索结果数量
            authors: Vec::new(),
            exact_phrase: None,
            exclude_words: Vec::new(),
            journal_ref: None,
            subject_class: None,
            report_number: None,
            id_list: Vec::new(),
            start_index: 0,
        }
    }
}

// arXiv主要学科分类
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArxivCategory {
    // 物理学
    AstrophysicsOfGalaxies,
    AstrophysicsEarth,
    AstrophysicsInstrumentation,
    AstrophysicsSolarStellar,
    AstrophysicsCosmology,
    CondensedMatterDisordered,
    CondensedMatterMaterials,
    CondensedMatterMesoscale,
    CondensedMatterOther,
    CondensedMatterQuantumGases,
    CondensedMatterSoftMatter,
    CondensedMatterStatMech,
    CondensedMatterStronglyCorr,
    CondensedMatterSupercond,
    GeneralRelativityQuantumCosmology,
    HighEnergyPhysicsExperiment,
    HighEnergyPhysicsLattice,
    HighEnergyPhysicsPhenomenology,
    HighEnergyPhysicsTheory,
    MathematicalPhysics,
    NonlinearScienceAdaptation,
    NonlinearScienceCellularAutomata,
    NonlinearScienceChaotic,
    NonlinearScienceExactly,
    NonlinearSciencePattern,
    NuclearExperiment,
    NuclearTheory,
    PhysicsAcceleratorPhysics,
    PhysicsAppliedPhysics,
    PhysicsAtmosphericOceanic,
    PhysicsAtomicMolecularClusters,
    PhysicsAtomicPhysics,
    PhysicsBiological,
    PhysicsChemicalPhysics,
    PhysicsClassicalPhysics,
    PhysicsComputationalPhysics,
    PhysicsDataAnalysis,
    PhysicsFluidDynamics,
    PhysicsGeneralPhysics,
    PhysicsGeophysics,
    PhysicsHistory,
    PhysicsInstrumentation,
    PhysicsMedicalPhysics,
    PhysicsOptics,
    PhysicsPhysicsEducation,
    PhysicsPhysicsSociety,
    PhysicsPlasmaPhysics,
    PhysicsPopularPhysics,
    PhysicsSpacePhysics,
    QuantumPhysics,
    
    // 数学
    Mathematics,
    MathematicsAlgebraicGeometry,
    MathematicsAlgebraicTopology,
    MathematicsAnalysisOfPDEs,
    MathematicsCategory,
    MathematicsClassicalAnalysis,
    MathematicsCombinatorics,
    MathematicsCommutativeAlgebra,
    MathematicsComplexVariables,
    MathematicsDifferentialGeometry,
    MathematicsDynamicalSystems,
    MathematicsFunctionalAnalysis,
    MathematicsGeneralMathematics,
    MathematicsGeneralTopology,
    MathematicsGeometricTopology,
    MathematicsGroupTheory,
    MathematicsHistory,
    MathematicsInformationTheory,
    MathematicsKTheoryHomology,
    MathematicsLogic,
    MathematicsMathematicalPhysics,
    MathematicsMetricGeometry,
    MathematicsNumberTheory,
    MathematicsNumericanalysis,
    MathematicsOperatorAlgebras,
    MathematicsOptimizationControl,
    MathematicsProbability,
    MathematicsQuantumAlgebra,
    MathematicsRepresentationTheory,
    MathematicsRingsAlgebras,
    MathematicsSpectralTheory,
    MathematicsStatisticsMathematics,
    MathematicsSymplecticGeometry,
    
    // 计算机科学
    ComputerScienceAI,
    ComputerScienceAlgorithms,
    ComputerScienceArchitecture,
    ComputerScienceComputationalComplexity,
    ComputerScienceComputationalEngineering,
    ComputerScienceComputationalGeometry,
    ComputerScienceComputerVision,
    ComputerScienceCryptography,
    ComputerScienceDataStructures,
    ComputerScienceDatabases,
    ComputerScienceDigitalLibraries,
    ComputerScienceDiscreteMathematics,
    ComputerScienceDistributed,
    ComputerScienceEmergingTechnologies,
    ComputerScienceFormalLanguages,
    ComputerScienceGeneral,
    ComputerScienceGraphics,
    ComputerScienceHumanComputerInteraction,
    ComputerScienceInformationRetrieval,
    ComputerScienceInformationTheory,
    ComputerScienceInstrumentation,
    ComputerScienceLearning,
    ComputerScienceLogicInCS,
    ComputerScienceMachineLearning,
    ComputerScienceMultimedis,
    ComputerScienceNetworking,
    ComputerScienceNeuralEvolutionary,
    ComputerScienceNumericalAnalysis,
    ComputerScienceOperatingSystems,
    ComputerScienceOther,
    ComputerSciencePerformance,
    ComputerScienceProgrammingLanguages,
    ComputerScienceRobotics,
    ComputerScienceSoftwareEngineering,
    ComputerScienceSound,
    ComputerScienceSymbolicComputation,
    ComputerScienceSystems,
    
    // 其他领域
    QuantitativeBiology,
    QuantitativeFinance,
    Statistics,
    Economics,
}

impl ArxivCategory {
    pub fn code(&self) -> &'static str {
        match self {
            // 理论物理学核心
            ArxivCategory::HighEnergyPhysicsTheory => "hep-th",
            ArxivCategory::GeneralRelativityQuantumCosmology => "gr-qc",
            ArxivCategory::MathematicalPhysics => "math-ph",
            ArxivCategory::QuantumPhysics => "quant-ph",
            ArxivCategory::HighEnergyPhysicsPhenomenology => "hep-ph",
            ArxivCategory::HighEnergyPhysicsExperiment => "hep-ex",
            ArxivCategory::HighEnergyPhysicsLattice => "hep-lat",
            
            // 凝聚态物理
            ArxivCategory::CondensedMatterDisordered => "cond-mat.dis-nn",
            ArxivCategory::CondensedMatterStronglyCorr => "cond-mat.str-el",
            ArxivCategory::CondensedMatterMaterials => "cond-mat.mtrl-sci",
            ArxivCategory::CondensedMatterMesoscale => "cond-mat.mes-hall",
            ArxivCategory::CondensedMatterQuantumGases => "cond-mat.quant-gas",
            ArxivCategory::CondensedMatterSoftMatter => "cond-mat.soft",
            ArxivCategory::CondensedMatterStatMech => "cond-mat.stat-mech",
            ArxivCategory::CondensedMatterSupercond => "cond-mat.supr-con",
            
            // 天体物理
            ArxivCategory::AstrophysicsOfGalaxies => "astro-ph.GA",
            ArxivCategory::AstrophysicsEarth => "astro-ph.EP",
            ArxivCategory::AstrophysicsCosmology => "astro-ph.CO",
            ArxivCategory::AstrophysicsInstrumentation => "astro-ph.IM",
            ArxivCategory::AstrophysicsSolarStellar => "astro-ph.SR",
            
            // 数学物理相关数学
            ArxivCategory::Mathematics => "math",
            ArxivCategory::MathematicsAlgebraicGeometry => "math.AG",
            ArxivCategory::MathematicsNumberTheory => "math.NT",
            ArxivCategory::MathematicsDifferentialGeometry => "math.DG",
            ArxivCategory::MathematicsRepresentationTheory => "math.RT",
            ArxivCategory::MathematicsQuantumAlgebra => "math.QA",
            ArxivCategory::MathematicsFunctionalAnalysis => "math.FA",
            ArxivCategory::MathematicsAlgebraicTopology => "math.AT",
            ArxivCategory::MathematicsMathematicalPhysics => "math.MP",
            
            // 核物理
            ArxivCategory::NuclearExperiment => "nucl-ex",
            ArxivCategory::NuclearTheory => "nucl-th",
            
            // 计算机科学
            ArxivCategory::ComputerScienceAI => "cs.AI",
            ArxivCategory::ComputerScienceMachineLearning => "cs.LG",
            ArxivCategory::ComputerScienceComputerVision => "cs.CV",
            ArxivCategory::ComputerScienceNeuralEvolutionary => "cs.NE",
            ArxivCategory::ComputerScienceAlgorithms => "cs.DS",
            ArxivCategory::ComputerScienceCryptography => "cs.CR",
            
            // 其他
            ArxivCategory::QuantitativeBiology => "q-bio",
            ArxivCategory::QuantitativeFinance => "q-fin",
            ArxivCategory::Statistics => "stat",
            ArxivCategory::Economics => "econ",
            
            // 为简化，这里只列举部分，实际应用中应该包含所有类别
            _ => "physics",
        }
    }
    
    pub fn display_name(&self) -> &'static str {
        match self {
            // 理论物理学核心
            ArxivCategory::HighEnergyPhysicsTheory => "High Energy Physics - Theory (hep-th)",
            ArxivCategory::GeneralRelativityQuantumCosmology => "General Relativity and Quantum Cosmology (gr-qc)",
            ArxivCategory::MathematicalPhysics => "Mathematical Physics (math-ph)",
            ArxivCategory::QuantumPhysics => "Quantum Physics (quant-ph)",
            ArxivCategory::HighEnergyPhysicsPhenomenology => "High Energy Physics - Phenomenology (hep-ph)",
            ArxivCategory::HighEnergyPhysicsExperiment => "High Energy Physics - Experiment (hep-ex)",
            ArxivCategory::HighEnergyPhysicsLattice => "High Energy Physics - Lattice (hep-lat)",
            
            // 凝聚态物理
            ArxivCategory::CondensedMatterDisordered => "Condensed Matter - Disordered Systems",
            ArxivCategory::CondensedMatterStronglyCorr => "Condensed Matter - Strongly Correlated Electrons",
            ArxivCategory::CondensedMatterMaterials => "Condensed Matter - Materials Science",
            ArxivCategory::CondensedMatterQuantumGases => "Condensed Matter - Quantum Gases",
            
            // 天体物理与宇宙学
            ArxivCategory::AstrophysicsOfGalaxies => "Astrophysics - Galaxies",
            ArxivCategory::AstrophysicsCosmology => "Astrophysics - Cosmology",
            ArxivCategory::AstrophysicsEarth => "Astrophysics - Earth and Planetary",
            
            // 数学物理相关数学
            ArxivCategory::Mathematics => "Mathematics (General)",
            ArxivCategory::MathematicsAlgebraicGeometry => "Mathematics - Algebraic Geometry",
            ArxivCategory::MathematicsDifferentialGeometry => "Mathematics - Differential Geometry",
            ArxivCategory::MathematicsRepresentationTheory => "Mathematics - Representation Theory",
            ArxivCategory::MathematicsQuantumAlgebra => "Mathematics - Quantum Algebra",
            ArxivCategory::MathematicsFunctionalAnalysis => "Mathematics - Functional Analysis",
            ArxivCategory::MathematicsNumberTheory => "Mathematics - Number Theory",
            ArxivCategory::MathematicsAlgebraicTopology => "Mathematics - Algebraic Topology",
            ArxivCategory::MathematicsMathematicalPhysics => "Mathematics - Mathematical Physics",
            
            // 核物理
            ArxivCategory::NuclearTheory => "Nuclear Theory",
            ArxivCategory::NuclearExperiment => "Nuclear Experiment",
            
            // 计算机科学
            ArxivCategory::ComputerScienceAI => "Computer Science - Artificial Intelligence",
            ArxivCategory::ComputerScienceMachineLearning => "Computer Science - Machine Learning",
            ArxivCategory::ComputerScienceComputerVision => "Computer Science - Computer Vision",
            ArxivCategory::ComputerScienceNeuralEvolutionary => "Computer Science - Neural Networks",
            ArxivCategory::ComputerScienceAlgorithms => "Computer Science - Data Structures and Algorithms",
            ArxivCategory::ComputerScienceCryptography => "Computer Science - Cryptography",
            
            ArxivCategory::QuantitativeBiology => "Quantitative Biology",
            ArxivCategory::QuantitativeFinance => "Quantitative Finance",
            ArxivCategory::Statistics => "Statistics",
            ArxivCategory::Economics => "Economics",
            
            _ => "Other",
        }
    }
    
    pub fn popular_categories() -> Vec<Self> {
        vec![
            // 理论物理学核心领域
            ArxivCategory::HighEnergyPhysicsTheory,          // hep-th
            ArxivCategory::GeneralRelativityQuantumCosmology, // gr-qc
            ArxivCategory::MathematicalPhysics,              // math-ph
            ArxivCategory::QuantumPhysics,                   // quant-ph
            
            // 数学物理相关数学分支
            ArxivCategory::MathematicsAlgebraicGeometry,
            ArxivCategory::MathematicsDifferentialGeometry,
            ArxivCategory::MathematicsRepresentationTheory,
            ArxivCategory::MathematicsQuantumAlgebra,
            
            // 相关物理分支
            ArxivCategory::HighEnergyPhysicsPhenomenology,
            ArxivCategory::CondensedMatterStronglyCorr,
            ArxivCategory::AstrophysicsCosmology,
            
            // 其他数学分支
            ArxivCategory::Mathematics,
            ArxivCategory::MathematicsNumberTheory,
            ArxivCategory::MathematicsFunctionalAnalysis,
        ]
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

// 日期范围
#[allow(dead_code)]
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
}

// 排序方式
#[derive(Debug, Clone, PartialEq)]
pub enum SortBy {
    Relevance,
    SubmissionDate,
    LastUpdated,
}

impl std::fmt::Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortBy::Relevance => write!(f, "Relevance"),
            SortBy::SubmissionDate => write!(f, "Submission Date"),
            SortBy::LastUpdated => write!(f, "Last Updated"),
        }
    }
}

impl SortBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortBy::Relevance => "relevance",
            SortBy::SubmissionDate => "submittedDate",
            SortBy::LastUpdated => "lastUpdatedDate",
        }
    }

    pub fn all_variants() -> Vec<Self> {
        vec![
            SortBy::Relevance,
            SortBy::SubmissionDate,
            SortBy::LastUpdated,
        ]
    }
}

// 排序顺序
#[derive(Debug, Clone, PartialEq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOrder::Ascending => write!(f, "Ascending"),
            SortOrder::Descending => write!(f, "Descending"),
        }
    }
}

impl SortOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortOrder::Ascending => "ascending",
            SortOrder::Descending => "descending",
        }
    }

    pub fn all_variants() -> Vec<Self> {
        vec![
            SortOrder::Ascending,
            SortOrder::Descending,
        ]
    }
}

impl std::fmt::Display for ArxivCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}

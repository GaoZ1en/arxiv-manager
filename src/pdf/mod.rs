// PDF 处理和浏览模块

pub mod viewer;
pub mod renderer;
pub mod page_cache;
pub mod search;

pub use viewer::{PdfViewer, PdfViewerMessage};
pub use renderer::{PdfRenderer, RenderError, HighlightRect};
pub use page_cache::{PageCache, CacheConfig, CacheStats};
pub use search::{PdfSearchEngine, SearchResult, SearchStats};

/// PDF 页面信息
#[derive(Debug, Clone)]
pub struct PdfPage {
    pub page_number: u32,
    pub svg_data: String, // SVG矢量数据
    pub width: u32,
    pub height: u32,
    pub text_content: Option<String>,
    pub search_highlights: Vec<SearchHighlight>,
}

/// 搜索高亮信息
#[derive(Debug, Clone)]
pub struct SearchHighlight {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub text: String,
}

/// PDF 查看器状态
#[derive(Debug, Clone)]
pub struct PdfViewerState {
    pub current_page: u32,
    pub total_pages: u32,
    pub zoom_level: f32,
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub is_loading: bool,
    pub search_term: Option<String>,
    pub search_results: Vec<SearchResult>,
    pub current_search_index: Option<usize>,
}

impl Default for PdfViewerState {
    fn default() -> Self {
        Self {
            current_page: 1,
            total_pages: 0,
            zoom_level: 1.0,
            scroll_x: 0.0,
            scroll_y: 0.0,
            is_loading: false,
            search_term: None,
            search_results: Vec::new(),
            current_search_index: None,
        }
    }
}

/// PDF 浏览器配置
#[derive(Debug, Clone)]
pub struct PdfConfig {
    pub max_cache_size: usize,
    pub prerender_pages: u32,
    pub default_zoom: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub zoom_step: f32,
    pub background_color: [u8; 3],
    pub text_color: [u8; 3],
    pub highlight_color: [u8; 3],
    pub invert_colors: bool,
}

impl Default for PdfConfig {
    fn default() -> Self {
        Self {
            max_cache_size: 50,
            prerender_pages: 3,
            default_zoom: 1.0,
            min_zoom: 0.1,
            max_zoom: 5.0,
            zoom_step: 0.1,
            background_color: [255, 255, 255],
            text_color: [0, 0, 0],
            highlight_color: [255, 255, 0],
            invert_colors: false,
        }
    }
}

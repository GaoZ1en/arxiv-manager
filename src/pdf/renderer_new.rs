// PDF渲染引擎 - 基于TDF库架构设计

use std::path::Path;
use mupdf::{Document, Matrix, Pixmap, Colorspace, TextPageOptions};
use flume::{Receiver, Sender};

use super::SearchHighlight;

#[derive(Debug, Clone)]
pub enum RenderError {
    Document(String),
    Page(String),
    Render(String),
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderError::Document(msg) => write!(f, "Document error: {}", msg),
            RenderError::Page(msg) => write!(f, "Page error: {}", msg),
            RenderError::Render(msg) => write!(f, "Render error: {}", msg),
        }
    }
}

impl std::error::Error for RenderError {}

/// 页面渲染信息
#[derive(Debug, Clone)]
pub struct PageInfo {
    pub page_number: u32,
    pub image_data: ImageData,
    pub search_highlights: Vec<SearchHighlight>,
}

#[derive(Debug, Clone)]
pub struct ImageData {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

/// PDF渲染器 - 负责将PDF页面转换为图像数据
pub struct PdfRenderer {
    document: Option<Document>,
    page_count: usize,
}

impl PdfRenderer {
    /// 创建新的PDF渲染器
    pub async fn new(path: &Path) -> Result<Self, RenderError> {
        let doc = Document::open(path)
            .map_err(|e| RenderError::Document(format!("Failed to open PDF: {}", e)))?;
        
        let page_count = doc.page_count()
            .map_err(|e| RenderError::Document(format!("Failed to get page count: {}", e)))? as usize;
        
        Ok(Self {
            document: Some(doc),
            page_count,
        })
    }
    
    /// 获取页面总数
    pub async fn get_page_count(&self) -> u32 {
        self.page_count as u32
    }
    
    /// 渲染指定页面
    pub async fn render_page(&self, page_number: u32, zoom: f32) -> Result<PageInfo, RenderError> {
        let doc = self.document.as_ref()
            .ok_or_else(|| RenderError::Document("Document not loaded".to_string()))?;
        
        if page_number == 0 || page_number > self.page_count as u32 {
            return Err(RenderError::Page(format!("Invalid page number: {}", page_number)));
        }
        
        let page = doc.load_page(page_number as i32 - 1)
            .map_err(|e| RenderError::Page(format!("Failed to load page {}: {}", page_number, e)))?;
        
        let bounds = page.bounds()
            .map_err(|e| RenderError::Page(format!("Failed to get page bounds: {}", e)))?;
        
        // 计算渲染尺寸
        let scale = zoom * 2.0; // 高DPI渲染
        let width = (bounds.x1 - bounds.x0) * scale;
        let height = (bounds.y1 - bounds.y0) * scale;
        
        // 创建变换矩阵
        let matrix = Matrix::new_scale(scale, scale);
        
        // 创建像素图
        let colorspace = Colorspace::device_rgb();
        let mut pixmap = Pixmap::new(&colorspace, width as i32, height as i32, true)
            .map_err(|e| RenderError::Render(format!("Failed to create pixmap: {}", e)))?;
        
        // 渲染页面
        page.run(&mut pixmap, &matrix, None)
            .map_err(|e| RenderError::Render(format!("Failed to render page: {}", e)))?;
        
        // 提取像素数据
        let pixels = pixmap.samples().to_vec();
        
        let image_data = ImageData {
            pixels,
            width: width as u32,
            height: height as u32,
        };
        
        Ok(PageInfo {
            page_number,
            image_data,
            search_highlights: Vec::new(), // 稍后实现搜索功能
        })
    }
    
    /// 获取页面文本内容（用于搜索）
    pub async fn get_page_text(&self, page_number: u32) -> Result<String, RenderError> {
        let doc = self.document.as_ref()
            .ok_or_else(|| RenderError::Document("Document not loaded".to_string()))?;
        
        if page_number == 0 || page_number > self.page_count as u32 {
            return Err(RenderError::Page(format!("Invalid page number: {}", page_number)));
        }
        
        let page = doc.load_page(page_number as i32 - 1)
            .map_err(|e| RenderError::Page(format!("Failed to load page {}: {}", page_number, e)))?;
        
        let text_page = page.to_text_page()
            .map_err(|e| RenderError::Page(format!("Failed to extract text: {}", e)))?;
        
        let text = text_page.to_text()
            .map_err(|e| RenderError::Page(format!("Failed to get text: {}", e)))?;
        
        Ok(text)
    }
    
    /// 搜索文本并返回高亮区域
    pub async fn search_text(&self, page_number: u32, query: &str) -> Result<Vec<SearchHighlight>, RenderError> {
        let doc = self.document.as_ref()
            .ok_or_else(|| RenderError::Document("Document not loaded".to_string()))?;
        
        if page_number == 0 || page_number > self.page_count as u32 {
            return Err(RenderError::Page(format!("Invalid page number: {}", page_number)));
        }
        
        let page = doc.load_page(page_number as i32 - 1)
            .map_err(|e| RenderError::Page(format!("Failed to load page {}: {}", page_number, e)))?;
        
        let text_page = page.to_text_page()
            .map_err(|e| RenderError::Page(format!("Failed to extract text: {}", e)))?;
        
        let mut highlights = Vec::new();
        
        // 搜索文本
        if let Ok(search_results) = text_page.search(query, mupdf::TextPageOptions::empty()) {
            for quad in search_results {
                highlights.push(SearchHighlight {
                    x: quad.ll.x,
                    y: quad.ll.y,
                    width: quad.ur.x - quad.ll.x,
                    height: quad.ur.y - quad.ll.y,
                    text: query.to_string(),
                });
            }
        }
        
        Ok(highlights)
    }
}

impl Clone for PdfRenderer {
    fn clone(&self) -> Self {
        // 注意：这里我们创建一个没有文档的新实例
        // 在实际使用中，可能需要重新加载文档
        Self {
            document: None,
            page_count: self.page_count,
        }
    }
}

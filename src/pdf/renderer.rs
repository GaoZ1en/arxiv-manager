use anyhow::Result;
use std::path::Path;
use poppler::Document;
use cairo::{Context, SvgSurface};

use super::{PdfPage, PdfConfig};

/// PDF 渲染错误
#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("PDF parsing error: {0}")]
    PdfParsing(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid page number: {0}")]
    InvalidPage(u32),
    #[error("Document not loaded")]
    DocumentNotLoaded,
    #[error("Document error: {0}")]
    Document(String),
}

/// 搜索高亮矩形
#[derive(Debug, Clone)]
pub struct HighlightRect {
    pub ul_x: u32,
    pub ul_y: u32,
    pub lr_x: u32,
    pub lr_y: u32,
}

/// PDF 渲染器（使用 Poppler 库实现真正的PDF渲染）
pub struct PdfRenderer {
    document: Option<Document>,
    file_path: Option<String>,
    config: PdfConfig,
}

impl Default for PdfRenderer {
    fn default() -> Self {
        Self::new(PdfConfig::default())
    }
}

impl PdfRenderer {
    /// 创建新的 PDF 渲染器
    pub fn new(config: PdfConfig) -> Self {
        Self {
            document: None,
            file_path: None,
            config,
        }
    }

    /// 加载 PDF 文档
    pub fn load_document(&mut self, path: &str) -> Result<u32, RenderError> {
        // 检查文件是否存在
        if !Path::new(path).exists() {
            return Err(RenderError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "PDF file not found"
            )));
        }

        // 构建正确的file:// URI - 确保路径是绝对路径
        let file_uri = if path.starts_with("file://") {
            path.to_string()
        } else {
            // 确保路径是绝对路径
            let absolute_path = if path.starts_with('/') {
                path.to_string()
            } else {
                std::env::current_dir()
                    .map_err(|e| RenderError::Io(e))?
                    .join(path)
                    .to_string_lossy()
                    .to_string()
            };
            format!("file://{}", absolute_path)
        };
        
        println!("Loading PDF document from URI: {}", file_uri);

        // 使用Poppler加载PDF文档
        let document = Document::from_file(&file_uri, None)
            .map_err(|e| RenderError::Document(format!("Failed to load PDF: {}", e)))?;
        
        let page_count = document.n_pages() as u32;
        
        self.document = Some(document);
        self.file_path = Some(path.to_string());
        
        Ok(page_count)
    }

    /// 获取页面数量
    pub fn page_count(&self) -> Result<u32, RenderError> {
        if let Some(doc) = &self.document {
            Ok(doc.n_pages() as u32)
        } else {
            Err(RenderError::DocumentNotLoaded)
        }
    }

    /// 渲染指定页面（只生成SVG）
    pub fn render_page(
        &self,
        page_num: u32,
        zoom_level: f32,
        _viewport_size: (u32, u32),
        search_term: Option<&str>,
    ) -> Result<PdfPage, RenderError> {
        let document = self.document.as_ref()
            .ok_or(RenderError::DocumentNotLoaded)?;

        let page_index = (page_num - 1) as i32;
        if page_index < 0 || page_index >= document.n_pages() {
            return Err(RenderError::InvalidPage(page_num));
        }

        let page = document.page(page_index)
            .ok_or_else(|| RenderError::InvalidPage(page_num))?;

        // 获取页面原始尺寸
        let (page_width, page_height) = page.size();
        
        // 应用缩放
        let render_width = page_width * zoom_level as f64;
        let render_height = page_height * zoom_level as f64;

        // 创建SVG表面和上下文 - 使用内存流避免文件I/O
        use std::io::Cursor;
        use std::sync::{Arc, Mutex};
        
        let svg_buffer = Arc::new(Mutex::new(Vec::new()));
        let buffer_clone = svg_buffer.clone();
        
        // 创建一个写入器包装
        struct VecWriter {
            buffer: Arc<Mutex<Vec<u8>>>,
        }
        
        impl std::io::Write for VecWriter {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                self.buffer.lock().unwrap().extend_from_slice(buf);
                Ok(buf.len())
            }
            
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        
        let writer = VecWriter { buffer: buffer_clone };
        let surface = SvgSurface::for_stream(render_width, render_height, writer)
            .map_err(|e| RenderError::Document(format!("Failed to create SVG surface: {}", e)))?;
        
        let ctx = Context::new(&surface)
            .map_err(|e| RenderError::Document(format!("Failed to create context: {}", e)))?;

        // 设置白色背景
        ctx.set_source_rgb(1.0, 1.0, 1.0);
        ctx.paint()
            .map_err(|e| RenderError::Document(format!("Failed to paint background: {}", e)))?;

        // 缩放上下文
        ctx.scale(zoom_level as f64, zoom_level as f64);

        // 渲染PDF页面为矢量图形
        page.render(&ctx);

        // 如果有搜索词，添加高亮
        let mut search_highlights = Vec::new();
        if let Some(term) = search_term {
            if !term.is_empty() {
                search_highlights = self.find_and_highlight_text(&ctx, &page, term, zoom_level as f64)?;
            }
        }

        // 确保SVG完成
        surface.finish();
        drop(ctx);
        drop(surface);

        // 获取SVG数据
        let svg_data = svg_buffer.lock().unwrap().clone();
        let svg_string = String::from_utf8(svg_data)
            .map_err(|_| RenderError::Document("Invalid UTF-8 in SVG data".to_string()))?;

        // 提取页面文本用于搜索
        let page_text = page.text().map(|s| s.to_string()).unwrap_or_default();

        Ok(PdfPage {
            page_number: page_num,
            svg_data: svg_string,
            width: render_width as u32,
            height: render_height as u32,
            text_content: Some(page_text),
            search_highlights,
        })
    }

    /// 查找并高亮文本（简化版本，因为poppler-rs API限制）
    fn find_and_highlight_text(
        &self,
        ctx: &Context,
        page: &poppler::Page,
        search_term: &str,
        scale: f64,
    ) -> Result<Vec<super::SearchHighlight>, RenderError> {
        let mut highlights = Vec::new();
        
        // 获取页面文本
        let full_text = page.text().map(|s| s.to_string()).unwrap_or_default();
        let search_lower = search_term.to_lowercase();
        let text_lower = full_text.to_lowercase();
        
        // 获取页面尺寸
        let (page_width, page_height) = page.size();
        
        let mut start = 0;
        while let Some(pos) = text_lower[start..].find(&search_lower) {
            let actual_pos = start + pos;
            
            // 估算文本位置（基于字符数和页面尺寸）
            let chars_per_line = 80; // 估算每行字符数
            let line_height = 16.0; // 估算行高
            let char_width = 8.0;   // 估算字符宽度
            
            let line = actual_pos / chars_per_line;
            let col = actual_pos % chars_per_line;
            
            let x = 50.0 + (col as f64 * char_width);
            let y = 50.0 + (line as f64 * line_height);
            let width = search_term.len() as f64 * char_width;
            let height = line_height;
            
            // 确保坐标在页面范围内
            if x < page_width && y < page_height {
                // 设置高亮颜色（半透明黄色）
                ctx.set_source_rgba(1.0, 1.0, 0.0, 0.3);
                ctx.rectangle(x, y, width, height);
                let _ = ctx.fill();
                
                highlights.push(super::SearchHighlight {
                    x: (x * scale) as f32,
                    y: (y * scale) as f32,
                    width: (width * scale) as f32,
                    height: (height * scale) as f32,
                    text: search_term.to_string(),
                });
            }
            
            start = actual_pos + search_term.len();
        }
        
        Ok(highlights)
    }

    /// 提取页面文本
    pub fn extract_text(&self, page_num: u32) -> Result<String, RenderError> {
        let document = self.document.as_ref()
            .ok_or(RenderError::DocumentNotLoaded)?;

        let page_index = (page_num - 1) as i32;
        if page_index < 0 || page_index >= document.n_pages() {
            return Err(RenderError::InvalidPage(page_num));
        }

        let page = document.page(page_index)
            .ok_or_else(|| RenderError::InvalidPage(page_num))?;

        Ok(page.text().map(|s| s.to_string()).unwrap_or_default())
    }

    /// 搜索文本
    pub fn search_text(&self, query: &str) -> Result<Vec<(u32, Vec<String>)>, RenderError> {
        let document = self.document.as_ref()
            .ok_or(RenderError::DocumentNotLoaded)?;

        let mut results = Vec::new();
        let query_lower = query.to_lowercase();

        for page_num in 0..document.n_pages() {
            if let Some(page) = document.page(page_num) {
                let page_text = page.text().map(|s| s.to_string()).unwrap_or_default();
                let text_lower = page_text.to_lowercase();
                let mut matches = Vec::new();
                let mut start = 0;

                while let Some(pos) = text_lower[start..].find(&query_lower) {
                    let actual_pos = start + pos;
                    let context_start = actual_pos.saturating_sub(50);
                    let context_end = (actual_pos + query.len() + 50).min(page_text.len());
                    let context = page_text[context_start..context_end].to_string();
                    matches.push(context);
                    start = actual_pos + query.len();
                }

                if !matches.is_empty() {
                    results.push((page_num as u32 + 1, matches));
                }
            }
        }

        Ok(results)
    }

    /// 搜索结果计数
    pub fn count_search_results(&self, page_num: u32, search_term: &str) -> usize {
        if let Ok(text) = self.extract_text(page_num) {
            let text_lower = text.to_lowercase();
            let term_lower = search_term.to_lowercase();
            let mut count = 0;
            let mut start = 0;
            
            while let Some(pos) = text_lower[start..].find(&term_lower) {
                count += 1;
                start = start + pos + search_term.len();
            }
            
            count
        } else {
            0
        }
    }
}
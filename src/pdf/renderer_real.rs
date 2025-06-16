use anyhow::Result;
use std::path::Path;

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

/// PDF 渲染器（使用 pdf crate）
pub struct PdfRenderer {
    document: Option<pdf::file::File<Vec<u8>>>,
    file_path: Option<String>,
    config: PdfConfig,
    page_count: u32,
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
            page_count: 0,
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

        // 读取文件内容
        let file_content = std::fs::read(path)?;
        
        // 尝试打开PDF文档
        match pdf::file::File::from_data(file_content) {
            Ok(document) => {
                self.page_count = document.num_pages();
                self.document = Some(document);
                self.file_path = Some(path.to_string());
                Ok(self.page_count)
            }
            Err(e) => {
                Err(RenderError::Document(format!("Failed to open PDF: {}", e)))
            }
        }
    }

    /// 获取页面数量
    pub fn page_count(&self) -> Result<u32, RenderError> {
        if self.document.is_none() {
            return Err(RenderError::DocumentNotLoaded);
        }
        Ok(self.page_count)
    }

    /// 渲染指定页面（简化版本）
    pub fn render_page(
        &self,
        page_num: u32,
        zoom_level: f32,
        viewport_size: (u32, u32),
        search_term: Option<&str>,
    ) -> Result<PdfPage, RenderError> {
        let document = self.document.as_ref()
            .ok_or(RenderError::DocumentNotLoaded)?;

        if page_num == 0 || page_num > self.page_count {
            return Err(RenderError::InvalidPage(page_num));
        }

        // 获取页面
        let page = document.get_page(page_num - 1)
            .map_err(|e| RenderError::Document(format!("Failed to get page {}: {}", page_num, e)))?;

        let (viewport_width, viewport_height) = viewport_size;
        
        // 计算渲染尺寸
        let render_width = (viewport_width as f32 * zoom_level) as u32;
        let render_height = (viewport_height as f32 * zoom_level) as u32;
        
        // 创建简单的占位符图像数据（白色背景）
        let pixel_count = (render_width * render_height * 4) as usize; // RGBA
        let mut image_data = vec![255u8; pixel_count]; // 白色背景
        
        // 添加一些简单的内容指示
        self.draw_placeholder_content(&mut image_data, render_width, render_height, page_num);

        // 提取文本内容
        let text_content = self.extract_page_text(&page);

        // 处理搜索高亮
        let search_highlights = if let Some(term) = search_term {
            self.find_search_highlights(&text_content, term)
        } else {
            Vec::new()
        };

        Ok(PdfPage {
            page_number: page_num,
            image_data,
            width: render_width,
            height: render_height,
            text_content: Some(text_content),
            search_highlights,
        })
    }

    /// 绘制占位符内容
    fn draw_placeholder_content(&self, image_data: &mut [u8], width: u32, height: u32, page_num: u32) {
        // 绘制边框
        for y in 0..height {
            for x in 0..width {
                let index = ((y * width + x) * 4) as usize;
                if index + 3 < image_data.len() {
                    // 绘制边框
                    if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                        image_data[index] = 0;     // R
                        image_data[index + 1] = 0; // G
                        image_data[index + 2] = 0; // B
                        image_data[index + 3] = 255; // A
                    }
                    // 绘制页码指示
                    else if x > 50 && x < 200 && y > 50 && y < 80 {
                        image_data[index] = 100;   // R
                        image_data[index + 1] = 100; // G
                        image_data[index + 2] = 100; // B
                        image_data[index + 3] = 255; // A
                    }
                }
            }
        }
    }

    /// 从页面提取文本
    fn extract_page_text(&self, page: &pdf::object::Page) -> String {
        // 尝试提取页面的文本内容
        // 这是一个简化实现，实际的文本提取需要更复杂的逻辑
        match page.extract_text() {
            Ok(text) => text,
            Err(_) => {
                format!("第 {} 页（文本提取失败）\n\n这是一个PDF页面的占位符显示。\n实际应用中会显示真实的PDF内容。", page.page_nr + 1)
            }
        }
    }

    /// 查找搜索高亮
    fn find_search_highlights(&self, text: &str, search_term: &str) -> Vec<super::SearchHighlight> {
        let mut highlights = Vec::new();
        
        // 简单的文本搜索
        for (index, _) in text.match_indices(search_term) {
            highlights.push(super::SearchHighlight {
                x: 50.0 + (index % 20) as f32 * 10.0,
                y: 100.0 + (index / 20) as f32 * 25.0,
                width: search_term.len() as f32 * 8.0,
                height: 20.0,
                text: search_term.to_string(),
            });
        }
        
        highlights
    }

    /// 提取页面文本内容
    pub fn extract_text(&self, page_num: u32) -> Result<String, RenderError> {
        let document = self.document.as_ref()
            .ok_or(RenderError::DocumentNotLoaded)?;

        if page_num == 0 || page_num > self.page_count {
            return Err(RenderError::InvalidPage(page_num));
        }

        // 获取页面并提取文本
        let page = document.get_page(page_num - 1)
            .map_err(|e| RenderError::Document(format!("Failed to get page {}: {}", page_num, e)))?;

        Ok(self.extract_page_text(&page))
    }

    /// 计算搜索结果数量
    pub fn count_search_results(&self, page_num: u32, search_term: &str) -> Result<usize, RenderError> {
        let text = self.extract_text(page_num)?;
        Ok(text.matches(search_term).count())
    }

    /// 检查文档是否已加载
    pub fn is_loaded(&self) -> bool {
        self.document.is_some()
    }

    /// 关闭文档
    pub fn close(&mut self) {
        self.document = None;
        self.file_path = None;
        self.page_count = 0;
    }

    /// 获取文档信息
    pub fn get_document_info(&self) -> Result<DocumentInfo, RenderError> {
        let document = self.document.as_ref()
            .ok_or(RenderError::DocumentNotLoaded)?;

        // 尝试获取文档元数据
        let title = document.trailer.info_dict
            .as_ref()
            .and_then(|info| info.title.as_ref())
            .map(|t| t.to_string())
            .or_else(|| Some("PDF 文档".to_string()));

        let author = document.trailer.info_dict
            .as_ref()
            .and_then(|info| info.author.as_ref())
            .map(|a| a.to_string());

        Ok(DocumentInfo {
            title,
            author,
            subject: None,
            creator: Some("PDF 查看器".to_string()),
            page_count: self.page_count,
            file_path: self.file_path.clone().unwrap_or_default(),
        })
    }
}

/// 文档信息
#[derive(Debug, Clone)]
pub struct DocumentInfo {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub creator: Option<String>,
    pub page_count: u32,
    pub file_path: String,
}

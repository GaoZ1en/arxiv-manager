// PDF 渲染器模块
// 基于 pdfium-render 库实现 PDF 页面渲染

use anyhow::{Result, anyhow};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use pdfium_render::prelude::*;

use super::{PdfPage, SearchHighlight, PdfConfig};

/// PDF 渲染错误
#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("PDF library error: {0}")]
    PdfiumError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid page number: {0}")]
    InvalidPage(u32),
    #[error("Rendering failed: {0}")]
    RenderFailed(String),
}

/// 页面渲染信息
#[derive(Debug, Clone)]
pub struct PageInfo {
    pub page_number: u32,
    pub width: u32,
    pub height: u32,
    pub dpi: f32,
}

/// PDF 渲染器
pub struct PdfRenderer {
    pdfium: Arc<Mutex<Pdfium>>,
    document: Option<Arc<Mutex<PdfDocument<'static>>>>,
    config: PdfConfig,
    render_sender: Option<mpsc::UnboundedSender<RenderRequest>>,
}

/// 渲染请求
#[derive(Debug)]
pub struct RenderRequest {
    pub page_number: u32,
    pub width: u32,
    pub height: u32,
    pub zoom: f32,
    pub response_sender: mpsc::UnboundedSender<Result<PdfPage, RenderError>>,
}

impl PdfRenderer {
    /// 创建新的 PDF 渲染器
    pub fn new(config: PdfConfig) -> Result<Self> {
        // 初始化 Pdfium
        let pdfium = match Pdfium::new(
            Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
                .or_else(|_| Pdfium::bind_to_system_library())
                .map_err(|e| anyhow!("Failed to initialize Pdfium: {:?}", e))?
        ) {
            Ok(p) => Arc::new(Mutex::new(p)),
            Err(e) => return Err(anyhow!("Failed to create Pdfium instance: {:?}", e)),
        };

        Ok(Self {
            pdfium,
            document: None,
            config,
            render_sender: None,
        })
    }

    /// 加载 PDF 文档
    pub async fn load_document<P: AsRef<Path>>(&mut self, path: P) -> Result<u32> {
        let path = path.as_ref();
        log::info!("Loading PDF document: {:?}", path);

        let file_data = tokio::fs::read(path).await?;
        
        let pdfium = self.pdfium.lock().unwrap();
        let document = pdfium
            .load_pdf_from_byte_vec(file_data, None)
            .map_err(|e| anyhow!("Failed to load PDF: {:?}", e))?;
        
        let page_count = document.pages().len() as u32;
        self.document = Some(Arc::new(Mutex::new(document)));
        
        // 启动渲染工作线程
        self.start_render_worker().await?;
        
        log::info!("PDF loaded successfully, {} pages", page_count);
        Ok(page_count)
    }

    /// 启动渲染工作线程
    async fn start_render_worker(&mut self) -> Result<()> {
        let (sender, mut receiver) = mpsc::unbounded_channel::<RenderRequest>();
        self.render_sender = Some(sender);

        let document = self.document.as_ref()
            .ok_or_else(|| anyhow!("No document loaded"))?
            .clone();
        let config = self.config.clone();

        tokio::spawn(async move {
            while let Some(request) = receiver.recv().await {
                let result = Self::render_page_internal(
                    &document,
                    &config,
                    request.page_number,
                    request.width,
                    request.height,
                    request.zoom,
                ).await;
                
                let _ = request.response_sender.send(result);
            }
        });

        Ok(())
    }

    /// 内部页面渲染方法
    async fn render_page_internal(
        document: &Arc<Mutex<PdfDocument<'static>>>,
        config: &PdfConfig,
        page_number: u32,
        width: u32,
        height: u32,
        zoom: f32,
    ) -> Result<PdfPage, RenderError> {
        let document = document.lock().unwrap();
        
        // 获取页面
        let page = document.pages().get((page_number - 1) as u16)
            .map_err(|_| RenderError::InvalidPage(page_number))?;

        // 计算渲染尺寸
        let page_width = page.width().value;
        let page_height = page.height().value;
        let scale = zoom * (width as f32 / page_width).min(height as f32 / page_height);
        
        let render_width = (page_width * scale) as u32;
        let render_height = (page_height * scale) as u32;

        // 创建位图并渲染
        let bitmap = page.render(
            PdfRenderConfig::new()
                .set_target_width(render_width as i32)
                .set_target_height(render_height as i32)
                .rotate_if_landscape(PdfPageRenderRotation::None, false)
        ).map_err(|e| RenderError::RenderFailed(format!("Render failed: {:?}", e)))?;

        // 转换为 RGB 数据
        let image_data = bitmap.as_bytes().to_vec();
        
        // 提取文本内容（用于搜索）
        let text_content = page.text()
            .map(|text| text.all())
            .unwrap_or_default();

        Ok(PdfPage {
            page_number,
            image_data,
            width: render_width,
            height: render_height,
            text_content: Some(text_content),
            search_highlights: Vec::new(),
        })
    }

    /// 渲染指定页面
    pub async fn render_page(
        &self,
        page_number: u32,
        width: u32,
        height: u32,
        zoom: f32,
    ) -> Result<PdfPage, RenderError> {
        let sender = self.render_sender.as_ref()
            .ok_or_else(|| RenderError::RenderFailed("Renderer not initialized".to_string()))?;

        let (response_sender, mut response_receiver) = mpsc::unbounded_channel();
        
        let request = RenderRequest {
            page_number,
            width,
            height,
            zoom,
            response_sender,
        };

        sender.send(request)
            .map_err(|_| RenderError::RenderFailed("Failed to send render request".to_string()))?;

        response_receiver.recv().await
            .ok_or_else(|| RenderError::RenderFailed("Failed to receive render response".to_string()))?
    }

    /// 获取页面信息
    pub fn get_page_info(&self, page_number: u32) -> Result<PageInfo, RenderError> {
        let document = self.document.as_ref()
            .ok_or_else(|| RenderError::RenderFailed("No document loaded".to_string()))?;
        
        let document = document.lock().unwrap();
        let page = document.pages().get((page_number - 1) as u16)
            .map_err(|_| RenderError::InvalidPage(page_number))?;

        Ok(PageInfo {
            page_number,
            width: page.width().value as u32,
            height: page.height().value as u32,
            dpi: 72.0, // 默认 DPI
        })
    }

    /// 获取文档页数
    pub fn get_page_count(&self) -> u32 {
        self.document.as_ref()
            .map(|doc| doc.lock().unwrap().pages().len() as u32)
            .unwrap_or(0)
    }

    /// 获取文档元数据
    pub fn get_metadata(&self) -> Result<PdfMetadata> {
        let document = self.document.as_ref()
            .ok_or_else(|| anyhow!("No document loaded"))?;
        
        let document = document.lock().unwrap();
        
        Ok(PdfMetadata {
            title: document.metadata().title().unwrap_or_default(),
            author: document.metadata().author().unwrap_or_default(),
            subject: document.metadata().subject().unwrap_or_default(),
            creator: document.metadata().creator().unwrap_or_default(),
            producer: document.metadata().producer().unwrap_or_default(),
            creation_date: document.metadata().creation_date()
                .map(|date| date.to_string())
                .unwrap_or_default(),
            modification_date: document.metadata().modification_date()
                .map(|date| date.to_string())
                .unwrap_or_default(),
        })
    }
}

/// PDF 元数据
#[derive(Debug, Clone, Default)]
pub struct PdfMetadata {
    pub title: String,
    pub author: String,
    pub subject: String,
    pub creator: String,
    pub producer: String,
    pub creation_date: String,
    pub modification_date: String,
}

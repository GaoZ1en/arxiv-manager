// PDF 搜索引擎模块
// 实现文本搜索和高亮功能

use std::collections::HashMap;
use regex::Regex;
use anyhow::{Result, anyhow};

use super::{SearchHighlight, PdfPage};
use super::renderer::PdfRenderer;

/// 搜索结果
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub page_number: u32,
    pub text: String,
    pub position: TextPosition,
    pub context: String,
    pub highlights: Vec<SearchHighlight>,
}

/// 文本位置
#[derive(Debug, Clone)]
pub struct TextPosition {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// 搜索选项
#[derive(Debug, Clone)]
pub struct SearchOptions {
    pub case_sensitive: bool,
    pub whole_words: bool,
    pub regex: bool,
    pub max_results: usize,
    pub context_chars: usize,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            case_sensitive: false,
            whole_words: false,
            regex: false,
            max_results: 100,
            context_chars: 50,
        }
    }
}

/// PDF 搜索引擎
pub struct PdfSearchEngine {
    text_cache: HashMap<u32, String>,
    last_search_term: Option<String>,
    last_search_results: Vec<SearchResult>,
}

impl PdfSearchEngine {
    /// 创建新的搜索引擎
    pub fn new() -> Self {
        Self {
            text_cache: HashMap::new(),
            last_search_term: None,
            last_search_results: Vec::new(),
        }
    }

    /// 搜索文本
    pub async fn search(
        &mut self,
        renderer: &PdfRenderer,
        query: &str,
        options: &SearchOptions,
    ) -> Result<Vec<SearchResult>> {
        if query.is_empty() {
            return Ok(Vec::new());
        }

        log::info!("Searching for: '{}' with options: {:?}", query, options);

        let mut results = Vec::new();
        let total_pages = renderer.get_page_count();

        // 构建搜索模式
        let pattern = self.build_search_pattern(query, options)?;

        // 搜索所有页面
        for page_num in 1..=total_pages {
            let text = self.get_page_text(renderer, page_num).await?;
            let page_results = self.search_in_text(&text, &pattern, page_num, options)?;
            results.extend(page_results);

            // 限制结果数量
            if results.len() >= options.max_results {
                results.truncate(options.max_results);
                break;
            }
        }

        log::info!("Found {} search results", results.len());
        
        // 缓存搜索结果
        self.last_search_term = Some(query.to_string());
        self.last_search_results = results.clone();

        Ok(results)
    }

    /// 在特定页面搜索
    pub async fn search_in_page(
        &mut self,
        renderer: &PdfRenderer,
        page_number: u32,
        query: &str,
        options: &SearchOptions,
    ) -> Result<Vec<SearchResult>> {
        if query.is_empty() {
            return Ok(Vec::new());
        }

        let text = self.get_page_text(renderer, page_number).await?;
        let pattern = self.build_search_pattern(query, options)?;
        self.search_in_text(&text, &pattern, page_number, options)
    }

    /// 构建搜索模式
    fn build_search_pattern(&self, query: &str, options: &SearchOptions) -> Result<Regex> {
        let pattern = if options.regex {
            query.to_string()
        } else {
            // 转义特殊字符
            let escaped = regex::escape(query);
            if options.whole_words {
                format!(r"\b{}\b", escaped)
            } else {
                escaped
            }
        };

        let flags = if options.case_sensitive {
            ""
        } else {
            "(?i)"
        };

        let full_pattern = format!("{}{}", flags, pattern);
        Regex::new(&full_pattern).map_err(|e| anyhow!("Invalid regex pattern: {}", e))
    }

    /// 在文本中搜索
    fn search_in_text(
        &self,
        text: &str,
        pattern: &Regex,
        page_number: u32,
        options: &SearchOptions,
    ) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();

        for mat in pattern.find_iter(text) {
            let start = mat.start();
            let end = mat.end();
            let matched_text = mat.as_str().to_string();

            // 获取上下文
            let context_start = start.saturating_sub(options.context_chars);
            let context_end = (end + options.context_chars).min(text.len());
            let context = text[context_start..context_end].to_string();

            // 估算文本位置（简化实现）
            let line_number = text[..start].matches('\n').count();
            let char_in_line = start - text[..start].rfind('\n').unwrap_or(0);
            
            // 简化的位置估算 - 实际实现需要更复杂的文本布局分析
            let position = TextPosition {
                x: (char_in_line * 8) as f32, // 假设平均字符宽度为8像素
                y: (line_number * 16) as f32,  // 假设行高为16像素
                width: (matched_text.len() * 8) as f32,
                height: 16.0,
            };

            // 创建高亮
            let highlight = SearchHighlight {
                x: position.x,
                y: position.y,
                width: position.width,
                height: position.height,
                text: matched_text.clone(),
            };

            let result = SearchResult {
                page_number,
                text: matched_text,
                position,
                context,
                highlights: vec![highlight],
            };

            results.push(result);
        }

        Ok(results)
    }

    /// 获取页面文本（带缓存）
    async fn get_page_text(&mut self, renderer: &PdfRenderer, page_number: u32) -> Result<String> {
        // 检查缓存
        if let Some(cached_text) = self.text_cache.get(&page_number) {
            return Ok(cached_text.clone());
        }

        // 从渲染器获取文本
        let text = self.extract_text_from_renderer(renderer, page_number).await?;
        
        // 缓存文本
        self.text_cache.insert(page_number, text.clone());
        
        Ok(text)
    }

    /// 从渲染器提取文本
    async fn extract_text_from_renderer(
        &self,
        renderer: &PdfRenderer,
        page_number: u32,
    ) -> Result<String> {
        // 这里需要根据具体的渲染器实现来提取文本
        // 为了演示，我们使用一个简化的方法
        
        // 渲染页面以获取文本内容
        let page = renderer.render_page(page_number, 800, 600, 1.0).await
            .map_err(|e| anyhow!("Failed to render page for text extraction: {:?}", e))?;
        
        Ok(page.text_content.unwrap_or_default())
    }

    /// 清除文本缓存
    pub fn clear_text_cache(&mut self) {
        self.text_cache.clear();
        log::debug!("Text cache cleared");
    }

    /// 清除特定页面的文本缓存
    pub fn clear_page_text_cache(&mut self, page_number: u32) {
        self.text_cache.remove(&page_number);
        log::debug!("Text cache cleared for page {}", page_number);
    }

    /// 获取上次搜索结果
    pub fn get_last_search_results(&self) -> &[SearchResult] {
        &self.last_search_results
    }

    /// 获取上次搜索词
    pub fn get_last_search_term(&self) -> Option<&str> {
        self.last_search_term.as_deref()
    }

    /// 查找下一个搜索结果
    pub fn find_next_result(&self, current_page: u32, current_position: Option<&TextPosition>) -> Option<&SearchResult> {
        if self.last_search_results.is_empty() {
            return None;
        }

        // 如果没有当前位置，返回当前页面的第一个结果
        if current_position.is_none() {
            return self.last_search_results.iter()
                .find(|result| result.page_number >= current_page);
        }

        let current_pos = current_position.unwrap();
        
        // 查找当前页面中当前位置之后的结果
        for result in &self.last_search_results {
            if result.page_number == current_page && result.position.y > current_pos.y {
                return Some(result);
            }
            if result.page_number > current_page {
                return Some(result);
            }
        }

        // 如果没找到，返回第一个结果（循环搜索）
        self.last_search_results.first()
    }

    /// 查找上一个搜索结果
    pub fn find_previous_result(&self, current_page: u32, current_position: Option<&TextPosition>) -> Option<&SearchResult> {
        if self.last_search_results.is_empty() {
            return None;
        }

        // 如果没有当前位置，返回当前页面的最后一个结果
        if current_position.is_none() {
            return self.last_search_results.iter()
                .rev()
                .find(|result| result.page_number <= current_page);
        }

        let current_pos = current_position.unwrap();
        
        // 查找当前页面中当前位置之前的结果
        for result in self.last_search_results.iter().rev() {
            if result.page_number == current_page && result.position.y < current_pos.y {
                return Some(result);
            }
            if result.page_number < current_page {
                return Some(result);
            }
        }

        // 如果没找到，返回最后一个结果（循环搜索）
        self.last_search_results.last()
    }

    /// 获取搜索统计
    pub fn get_search_stats(&self) -> SearchStats {
        let mut page_counts: HashMap<u32, usize> = HashMap::new();
        
        for result in &self.last_search_results {
            *page_counts.entry(result.page_number).or_insert(0) += 1;
        }

        SearchStats {
            total_results: self.last_search_results.len(),
            pages_with_results: page_counts.len(),
            results_per_page: page_counts,
        }
    }
}

impl Default for PdfSearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// 搜索统计信息
#[derive(Debug, Clone)]
pub struct SearchStats {
    pub total_results: usize,
    pub pages_with_results: usize,
    pub results_per_page: HashMap<u32, usize>,
}

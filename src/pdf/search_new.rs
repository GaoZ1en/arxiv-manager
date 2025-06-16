// PDF 文本搜索引擎 - 简化版本，不使用regex

use std::collections::HashMap;
use anyhow::{Result, anyhow};

use super::{PdfRenderer, SearchHighlight};

/// 搜索结果
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub page_number: u32,
    pub position: u32,
    pub context: String,
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
    pub max_results: usize,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            case_sensitive: false,
            whole_words: false,
            max_results: 100,
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
    pub async fn new(path: &std::path::Path) -> Result<Self> {
        Ok(Self {
            text_cache: HashMap::new(),
            last_search_term: None,
            last_search_results: Vec::new(),
        })
    }

    /// 搜索文本
    pub async fn search(&mut self, query: String) -> Result<Vec<SearchResult>> {
        if query.is_empty() {
            return Ok(Vec::new());
        }

        let options = SearchOptions::default();
        let mut results = Vec::new();

        // 简化的搜索实现 - 暂时返回模拟结果
        for page_num in 1..=5 {
            if let Ok(text) = self.get_page_text_cached(page_num).await {
                let page_results = self.search_in_text(&text, &query, page_num, &options);
                results.extend(page_results);

                if results.len() >= options.max_results {
                    results.truncate(options.max_results);
                    break;
                }
            }
        }

        // 缓存搜索结果
        self.last_search_term = Some(query);
        self.last_search_results = results.clone();

        Ok(results)
    }

    /// 获取缓存的页面文本
    async fn get_page_text_cached(&mut self, page_number: u32) -> Result<String> {
        if let Some(text) = self.text_cache.get(&page_number) {
            return Ok(text.clone());
        }

        // 模拟文本提取
        let text = format!("Sample text content for page {}", page_number);
        self.text_cache.insert(page_number, text.clone());
        Ok(text)
    }

    /// 在文本中搜索
    fn search_in_text(
        &self,
        text: &str,
        query: &str,
        page_number: u32,
        options: &SearchOptions,
    ) -> Vec<SearchResult> {
        let mut results = Vec::new();
        let search_text = if options.case_sensitive { text } else { &text.to_lowercase() };
        let search_query = if options.case_sensitive { query } else { &query.to_lowercase() };

        let mut start = 0;
        while let Some(pos) = search_text[start..].find(search_query) {
            let actual_pos = start + pos;
            
            // 如果需要完整单词匹配，检查边界
            if options.whole_words {
                let before_ok = actual_pos == 0 || 
                    !text.chars().nth(actual_pos - 1).unwrap_or(' ').is_alphanumeric();
                let after_ok = actual_pos + query.len() >= text.len() || 
                    !text.chars().nth(actual_pos + query.len()).unwrap_or(' ').is_alphanumeric();
                
                if !before_ok || !after_ok {
                    start = actual_pos + 1;
                    continue;
                }
            }

            // 创建上下文
            let context_start = actual_pos.saturating_sub(20);
            let context_end = (actual_pos + query.len() + 20).min(text.len());
            let context = text[context_start..context_end].to_string();

            results.push(SearchResult {
                page_number,
                position: actual_pos as u32,
                context,
                x: 100.0, // 模拟位置
                y: 100.0 + (results.len() as f32 * 20.0),
                width: query.len() as f32 * 8.0,
                height: 16.0,
            });

            start = actual_pos + query.len();
        }

        results
    }

    /// 清除缓存
    pub fn clear_cache(&mut self) {
        self.text_cache.clear();
        self.last_search_term = None;
        self.last_search_results.clear();
    }

    /// 获取最后一次搜索结果
    pub fn get_last_results(&self) -> &[SearchResult] {
        &self.last_search_results
    }

    /// 高亮搜索结果
    pub async fn highlight_results(
        &self,
        page_number: u32,
        query: &str,
    ) -> Vec<SearchHighlight> {
        self.last_search_results
            .iter()
            .filter(|result| result.page_number == page_number)
            .map(|result| SearchHighlight {
                x: result.x,
                y: result.y,
                width: result.width,
                height: result.height,
                text: query.to_string(),
            })
            .collect()
    }
}

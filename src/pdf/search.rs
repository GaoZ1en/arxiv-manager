use std::sync::Arc;
use anyhow::Result;

use super::PdfRenderer;

/// 搜索结果
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub page_number: u32,
    pub text: String,
    pub context: String,
    pub position: (f32, f32), // x, y coordinates
    pub highlighted_text: String,
}

/// PDF 搜索引擎
pub struct PdfSearchEngine {
    renderer: Arc<PdfRenderer>,
}

impl PdfSearchEngine {
    /// 创建新的搜索引擎
    pub fn new(renderer: Arc<PdfRenderer>) -> Self {
        Self { renderer }
    }

    /// 在整个文档中搜索文本
    pub async fn search_document(&self, search_term: &str) -> Result<Vec<SearchResult>> {
        if search_term.is_empty() {
            return Ok(Vec::new());
        }

        let page_count = self.renderer.page_count()
            .map_err(|e| anyhow::anyhow!("Failed to get page count: {}", e))?;

        let mut results = Vec::new();
        
        for page_num in 1..=page_count {
            match self.search_page(page_num, search_term).await {
                Ok(mut page_results) => results.append(&mut page_results),
                Err(e) => {
                    eprintln!("Error searching page {}: {}", page_num, e);
                    continue;
                }
            }
        }

        Ok(results)
    }

    /// 在指定页面中搜索文本
    pub async fn search_page(&self, page_num: u32, search_term: &str) -> Result<Vec<SearchResult>> {
        if search_term.is_empty() {
            return Ok(Vec::new());
        }

        // 提取页面文本
        let text = self.renderer.extract_text(page_num)
            .map_err(|e| anyhow::anyhow!("Failed to extract text from page {}: {}", page_num, e))?;

        let mut results = Vec::new();
        let search_term_lower = search_term.to_lowercase();
        let text_lower = text.to_lowercase();

        // 简单的文本搜索（不使用正则表达式）
        let mut start_pos = 0;
        while let Some(match_pos) = text_lower[start_pos..].find(&search_term_lower) {
            let absolute_pos = start_pos + match_pos;
            
            // 提取上下文（前后各50个字符）
            let context_start = if absolute_pos >= 50 { absolute_pos - 50 } else { 0 };
            let context_end = std::cmp::min(absolute_pos + search_term.len() + 50, text.len());
            let context = text[context_start..context_end].to_string();

            // 提取匹配的原始文本（保持大小写）
            let original_match = text[absolute_pos..absolute_pos + search_term.len()].to_string();

            results.push(SearchResult {
                page_number: page_num,
                text: original_match.clone(),
                context,
                position: (0.0, 0.0), // 坐标需要从渲染器获取，这里暂时使用占位符
                highlighted_text: original_match,
            });

            start_pos = absolute_pos + 1; // 继续搜索下一个匹配
        }

        Ok(results)
    }

    /// 计算文档中搜索词的总出现次数
    pub async fn count_total_matches(&self, search_term: &str) -> Result<usize> {
        if search_term.is_empty() {
            return Ok(0);
        }

        let page_count = self.renderer.page_count()
            .map_err(|e| anyhow::anyhow!("Failed to get page count: {}", e))?;

        let mut total_count = 0;
        
        for page_num in 1..=page_count {
            match self.count_page_matches(page_num, search_term).await {
                Ok(count) => total_count += count,
                Err(e) => {
                    eprintln!("Error counting matches on page {}: {}", page_num, e);
                    continue;
                }
            }
        }

        Ok(total_count)
    }

    /// 计算指定页面中搜索词的出现次数
    pub async fn count_page_matches(&self, page_num: u32, search_term: &str) -> Result<usize> {
        if search_term.is_empty() {
            return Ok(0);
        }

        Ok(self.renderer.count_search_results(page_num, search_term))
    }

    /// 查找下一个匹配
    pub fn find_next_match(&self, current_page: u32, current_index: usize, results: &[SearchResult]) -> Option<(u32, usize)> {
        if results.is_empty() {
            return None;
        }

        // 在当前页面查找下一个匹配
        let current_page_results: Vec<_> = results.iter()
            .enumerate()
            .filter(|(_, result)| result.page_number == current_page)
            .collect();

        if let Some(&(next_index, _)) = current_page_results.iter()
            .find(|(i, _)| *i > current_index) {
            return Some((current_page, next_index));
        }

        // 如果当前页面没有更多匹配，查找下一页
        if let Some((next_index, result)) = results.iter()
            .enumerate()
            .find(|(_, result)| result.page_number > current_page) {
            return Some((result.page_number, next_index));
        }

        // 如果没有找到，返回第一个匹配（循环搜索）
        results.first().map(|result| (result.page_number, 0))
    }

    /// 查找上一个匹配
    pub fn find_previous_match(&self, current_page: u32, current_index: usize, results: &[SearchResult]) -> Option<(u32, usize)> {
        if results.is_empty() {
            return None;
        }

        // 在当前页面查找上一个匹配
        let current_page_results: Vec<_> = results.iter()
            .enumerate()
            .filter(|(_, result)| result.page_number == current_page)
            .collect();

        if let Some(&(prev_index, _)) = current_page_results.iter()
            .rev()
            .find(|(i, _)| *i < current_index) {
            return Some((current_page, prev_index));
        }

        // 如果当前页面没有更多匹配，查找上一页
        if let Some((prev_index, result)) = results.iter()
            .enumerate()
            .rev()
            .find(|(_, result)| result.page_number < current_page) {
            return Some((result.page_number, prev_index));
        }

        // 如果没有找到，返回最后一个匹配（循环搜索）
        results.last().map(|result| (result.page_number, results.len() - 1))
    }

    /// 过滤指定页面的搜索结果
    pub fn filter_page_results<'a>(&self, page_num: u32, results: &'a [SearchResult]) -> Vec<&'a SearchResult> {
        results.iter()
            .filter(|result| result.page_number == page_num)
            .collect()
    }

    /// 获取搜索结果的统计信息
    pub fn get_search_stats(&self, results: &[SearchResult]) -> SearchStats {
        let total_matches = results.len();
        let pages_with_matches: std::collections::HashSet<u32> = results.iter()
            .map(|r| r.page_number)
            .collect();
        
        SearchStats {
            total_matches,
            pages_with_matches: pages_with_matches.len(),
            pages: pages_with_matches.into_iter().collect(),
        }
    }
}

/// 搜索统计信息
#[derive(Debug, Clone)]
pub struct SearchStats {
    pub total_matches: usize,
    pub pages_with_matches: usize,
    pub pages: Vec<u32>,
}
// 搜索消息处理器
// 处理所有与搜索相关的消息
// 增强版：支持缓存、防抖动和高级搜索功能

use iced::Task;
use std::time::Instant;

use crate::core::{ArxivManager, ArxivPaper, SearchField, DateRange, SortBy, SortOrder, ArxivCategory, SearchConfig};
use crate::core::messages::Message;
use crate::core::app_state::SearchCacheItem;
use crate::search::api::client::search_arxiv_papers_advanced;

pub trait SearchHandler {
    fn handle_search_query_changed(&mut self, query: String) -> Task<Message>;
    fn handle_search_submitted(&mut self) -> Task<Message>;
    fn handle_search_completed(&mut self, result: Result<Vec<ArxivPaper>, String>) -> Task<Message>;
    fn handle_advanced_search_toggled(&mut self) -> Task<Message>;
    fn handle_search_field_changed(&mut self, field: SearchField) -> Task<Message>;
    fn handle_category_toggled(&mut self, category: ArxivCategory) -> Task<Message>;
    fn handle_date_range_changed(&mut self, range: DateRange) -> Task<Message>;
    fn handle_sort_by_changed(&mut self, sort_by: SortBy) -> Task<Message>;
    fn handle_sort_order_changed(&mut self, order: SortOrder) -> Task<Message>;
    fn handle_max_results_changed(&mut self, value: String) -> Task<Message>;
    fn handle_author_added(&mut self, author: String) -> Task<Message>;
    fn handle_author_removed(&mut self, index: usize) -> Task<Message>;
    fn handle_search_by_author(&mut self, author: String) -> Task<Message>;
    fn handle_load_more_results(&mut self) -> Task<Message>;
    fn handle_load_more_completed(&mut self, result: Result<Vec<ArxivPaper>, String>) -> Task<Message>;
}

impl SearchHandler for ArxivManager {
    fn handle_search_query_changed(&mut self, query: String) -> Task<Message> {
        let start_time = Instant::now();
        
        // 更新两个查询字段以保持同步
        self.search_query = query.clone();
        self.search_config.query = query.clone();
        
        // 更新搜索建议
        self.update_search_suggestions(&query);
        
        // 记录最后搜索时间，用于防抖动
        self.last_search_time = Some(Instant::now());
        
        // 如果查询为空，清空结果
        if self.search_config.query.trim().is_empty() {
            self.search_results.clear();
            self.search_error = None;
            self.show_search_suggestions = false;
            return Task::none();
        }
        
        // 显示搜索建议
        self.show_search_suggestions = true;
        
        // 性能优化：检查智能缓存
        let cache_key = self.generate_cache_key();
        
        // 首先检查精确匹配
        if let Some(cached_item) = self.search_cache.get(&cache_key) {
            if cached_item.timestamp.elapsed().as_secs() < 300 {
                self.search_results = cached_item.results.clone();
                self.search_error = None;
                self.is_searching = false;
                
                // 更新性能统计（缓存命中，内联避免borrowing冲突）
                self.search_performance_stats.total_searches += 1;
                self.search_performance_stats.cache_hits += 1;
                self.search_performance_stats.last_search_duration = Some(start_time.elapsed());
                
                // 更新平均响应时间（移动平均）
                let new_time = start_time.elapsed().as_millis() as f64;
                let current_avg = self.search_performance_stats.average_response_time;
                let total = self.search_performance_stats.total_searches as f64;
                
                self.search_performance_stats.average_response_time = 
                    (current_avg * (total - 1.0) + new_time) / total;
                
                return Task::none();
            }
        }
        
        // 检查相似查询缓存
        let similar_queries = self.find_similar_cached_queries(&query, 0.8);
        if !similar_queries.is_empty() {
            // 添加到预加载队列进行后台搜索
            if !self.preload_queue.contains(&query) {
                self.preload_queue.push(query.clone());
            }
        }
        
        // 更新查询频率（避免borrowing冲突）
        if !query.trim().is_empty() {
            let normalized_query = query.trim().to_lowercase();
            *self.query_frequency.entry(normalized_query).or_insert(0) += 1;
        }
        
        Task::none()
    }

    fn handle_search_submitted(&mut self) -> Task<Message> {
        let start_time = Instant::now();
        
        // 添加到搜索历史
        self.add_to_search_history(self.search_config.query.clone());
        
        // 隐藏搜索建议
        self.show_search_suggestions = false;
        
        if !self.search_config.query.trim().is_empty() {
            // 性能优化：多层缓存检查
            let cache_key = self.generate_cache_key();
            
            // 1. 检查精确匹配缓存
            if let Some(cached_item) = self.search_cache.get(&cache_key) {
                if cached_item.timestamp.elapsed().as_secs() < 300 {
                    self.search_results = cached_item.results.clone();
                    self.search_error = None;
                    self.is_searching = false;
                    
                    // 重置分页状态
                    self.current_page = 0;
                    self.total_results_loaded = self.search_results.len() as u32;
                    self.has_more_results = self.search_results.len() >= self.search_config.max_results as usize;
                    
                    // 更新性能统计（缓存命中，内联避免borrowing冲突）
                    self.search_performance_stats.total_searches += 1;
                    self.search_performance_stats.cache_hits += 1;
                    self.search_performance_stats.last_search_duration = Some(start_time.elapsed());
                    
                    // 更新平均响应时间（移动平均）
                    let new_time = start_time.elapsed().as_millis() as f64;
                    let current_avg = self.search_performance_stats.average_response_time;
                    let total = self.search_performance_stats.total_searches as f64;
                    
                    self.search_performance_stats.average_response_time = 
                        (current_avg * (total - 1.0) + new_time) / total;
                    
                    return Task::none();
                }
            }
            
            // 2. 检查相似查询，提供部分结果
            let similar_queries = self.find_similar_cached_queries(&self.search_config.query, 0.7);
            if !similar_queries.is_empty() {
                if let Some(similar_cache) = self.search_cache.get(&similar_queries[0]) {
                    if similar_cache.timestamp.elapsed().as_secs() < 600 { // 相似查询缓存时间稍长
                        // 使用相似查询结果作为临时结果
                        self.search_results = similar_cache.results.clone();
                        self.search_error = Some("Showing similar results while searching...".to_string());
                    }
                }
            }
            
            // 3. 执行新搜索
            self.is_searching = true;
            self.search_error = None;
            
            // 更新查询频率和预测（避免borrowing冲突）
            let current_query = self.search_config.query.clone();
            {
                // 在独立的作用域中更新频率
                *self.query_frequency.entry(current_query.clone()).or_insert(0) += 1;
            }
            
            // 预测下一步可能的查询并预加载
            let predicted_queries = {
                let mut predictions = Vec::new();
                
                // 基于历史查询模式预测
                for history_query in &self.search_history {
                    if history_query.starts_with(&current_query) && history_query != &current_query {
                        predictions.push(history_query.clone());
                    }
                }
                
                // 基于相似查询预测
                let similar_queries = self.find_similar_cached_queries(&current_query, 0.6);
                predictions.extend(similar_queries);
                
                // 去重并限制数量
                predictions.sort();
                predictions.dedup();
                predictions.truncate(5);
                
                predictions
            };
            
            for predicted in predicted_queries {
                if !self.preload_queue.contains(&predicted) {
                    self.preload_queue.push(predicted);
                }
            }
            
            let config = self.search_config.clone();
            Task::perform(
                search_arxiv_papers_advanced(config),
                Message::SearchCompleted
            )
        } else {
            Task::none()
        }
    }

    fn handle_search_completed(&mut self, result: Result<Vec<ArxivPaper>, String>) -> Task<Message> {
        let search_start_time = self.last_search_time.unwrap_or_else(Instant::now);
        let search_duration = search_start_time.elapsed();
        
        self.is_searching = false;
        match result {
            Ok(papers) => {
                self.search_results = papers.clone();
                self.search_error = None;
                
                // 重置分页状态
                self.current_page = 0;
                self.total_results_loaded = papers.len() as u32;
                self.has_more_results = papers.len() >= self.search_config.max_results as usize;
                self.is_loading_more = false;
                
                // 缓存搜索结果
                let cache_key = self.generate_cache_key();
                let cache_item = SearchCacheItem {
                    results: papers.clone(),
                    timestamp: Instant::now(),
                    config: self.search_config.clone(),
                };
                self.search_cache.insert(cache_key, cache_item);
                
                // 更新性能统计（内联避免borrowing冲突）
                self.search_performance_stats.total_searches += 1;
                self.search_performance_stats.last_search_duration = Some(search_duration);
                
                // 更新平均响应时间（移动平均）
                let new_time = search_duration.as_millis() as f64;
                let current_avg = self.search_performance_stats.average_response_time;
                let total = self.search_performance_stats.total_searches as f64;
                
                self.search_performance_stats.average_response_time = 
                    (current_avg * (total - 1.0) + new_time) / total;
                
                // 执行智能缓存清理
                self.smart_cache_cleanup();
                
                // 更新相似查询映射
                let query = self.search_config.query.clone();
                let similar_queries = self.find_similar_cached_queries(&query, 0.6);
                self.query_similarity_cache.insert(query, similar_queries);
            }
            Err(error) => {
                self.search_error = Some(error);
                self.search_results.clear();
                
                // 清空分页状态
                self.current_page = 0;
                self.total_results_loaded = 0;
                self.has_more_results = false;
                self.is_loading_more = false;
                
                // 仍然更新性能统计（失败的搜索，内联避免borrowing冲突）
                self.search_performance_stats.total_searches += 1;
                self.search_performance_stats.last_search_duration = Some(search_duration);
                
                // 更新平均响应时间（移动平均）
                let new_time = search_duration.as_millis() as f64;
                let current_avg = self.search_performance_stats.average_response_time;
                let total = self.search_performance_stats.total_searches as f64;
                
                self.search_performance_stats.average_response_time = 
                    (current_avg * (total - 1.0) + new_time) / total;
            }
        }
        Task::none()
    }

    fn handle_advanced_search_toggled(&mut self) -> Task<Message> {
        self.advanced_search_visible = !self.advanced_search_visible;
        Task::none()
    }

    fn handle_search_field_changed(&mut self, field: SearchField) -> Task<Message> {
        self.search_config.search_in = field;
        Task::none()
    }

    fn handle_category_toggled(&mut self, category: ArxivCategory) -> Task<Message> {
        if let Some(pos) = self.search_config.categories.iter().position(|x| x == &category) {
            self.search_config.categories.remove(pos);
        } else {
            self.search_config.categories.push(category);
        }
        Task::none()
    }

    fn handle_date_range_changed(&mut self, range: DateRange) -> Task<Message> {
        self.search_config.date_range = range;
        Task::none()
    }

    fn handle_sort_by_changed(&mut self, sort_by: SortBy) -> Task<Message> {
        self.search_config.sort_by = sort_by;
        Task::none()
    }

    fn handle_sort_order_changed(&mut self, order: SortOrder) -> Task<Message> {
        self.search_config.sort_order = order;
        Task::none()
    }

    fn handle_max_results_changed(&mut self, value: String) -> Task<Message> {
        if let Ok(num) = value.parse::<u32>() {
            self.search_config.max_results = num.min(100).max(1);
        }
        Task::none()
    }

    fn handle_author_added(&mut self, author: String) -> Task<Message> {
        if !author.trim().is_empty() && !self.search_config.authors.contains(&author) {
            self.search_config.authors.push(author);
            // 清空作者输入框
            self.author_input.clear();
        }
        Task::none()
    }

    fn handle_author_removed(&mut self, index: usize) -> Task<Message> {
        if index < self.search_config.authors.len() {
            self.search_config.authors.remove(index);
        }
        Task::none()
    }

    fn handle_search_by_author(&mut self, author: String) -> Task<Message> {
        // 清空当前搜索条件
        self.search_config = SearchConfig::default();
        
        // 设置作者搜索 - 使用查询字段来确保搜索能够执行
        let author_query = format!("au:{}", author.trim());
        self.search_query = author_query.clone();
        self.search_config.query = author_query;
        self.search_config.search_in = SearchField::Authors;
        self.search_config.max_results = 50; // 作者搜索通常返回更多结果
        
        // 添加到搜索历史
        self.add_to_search_history(format!("author:{}", author.trim()));
        
        // 设置搜索状态
        self.is_searching = true;
        self.last_search_time = Some(std::time::Instant::now());
        
        // 直接执行搜索，绕过空查询检查
        let config = self.search_config.clone();
        println!("🚀 Executing search with config: {:?}", config);
        
        Task::perform(
            search_arxiv_papers_advanced(config),
            Message::SearchCompleted
        )
    }

    fn handle_load_more_results(&mut self) -> Task<Message> {
        // 防止重复加载
        if self.is_loading_more || !self.has_more_results {
            return Task::none();
        }

        self.is_loading_more = true;
        self.current_page += 1;

        // 构建查询配置，包含分页信息
        let mut load_more_config = self.search_config.clone();
        load_more_config.start_index = self.current_page * self.search_config.max_results;
        
        // 执行异步搜索
        Task::perform(
            async move {
                crate::search::api::client::search_arxiv_papers_advanced(load_more_config).await
            },
            Message::LoadMoreCompleted,
        )
    }

    fn handle_load_more_completed(&mut self, result: Result<Vec<ArxivPaper>, String>) -> Task<Message> {
        self.is_loading_more = false;

        match result {
            Ok(new_papers) => {
                // 如果返回的论文数量少于预期，说明没有更多结果了
                if new_papers.len() < self.search_config.max_results as usize {
                    self.has_more_results = false;
                }

                // 过滤重复的论文（基于ID）
                let existing_ids: std::collections::HashSet<_> = 
                    self.search_results.iter().map(|p| &p.id).collect();
                
                let new_unique_papers: Vec<_> = new_papers
                    .into_iter()
                    .filter(|p| !existing_ids.contains(&p.id))
                    .collect();

                // 添加新论文到搜索结果
                self.search_results.extend(new_unique_papers.clone());
                self.total_results_loaded += new_unique_papers.len() as u32;

                // 更新缓存
                let cache_key = self.generate_cache_key();
                let cache_item = SearchCacheItem {
                    results: self.search_results.clone(),
                    timestamp: Instant::now(),
                    config: self.search_config.clone(),
                };
                self.search_cache.insert(cache_key, cache_item);

                Task::none()
            }
            Err(e) => {
                self.search_error = Some(format!("Load more failed: {}", e));
                Task::none()
            }
        }
    }
}

// 辅助方法实现 - 独立的 impl 块
impl ArxivManager {
    /// 生成搜索缓存键
    fn generate_cache_key(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}",
            self.search_config.query,
            self.search_config.search_in.as_str(),
            format!("{:?}", self.search_config.date_range),
            format!("{:?}", self.search_config.sort_by),
            format!("{:?}", self.search_config.sort_order),
            self.search_config.max_results
        )
    }
}

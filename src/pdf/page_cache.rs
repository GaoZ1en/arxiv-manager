// PDF 页面缓存模块
// 实现智能页面缓存和预渲染

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::PdfPage;

/// 缓存配置
#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub max_size: usize,
    pub prerender_ahead: u32,
    pub prerender_behind: u32,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_size: 20,
            prerender_ahead: 3,
            prerender_behind: 2,
        }
    }
}

/// 缓存条目
#[derive(Debug, Clone)]
struct CacheEntry {
    page: PdfPage,
    access_time: std::time::Instant,
    zoom_level: f32,
}

/// 页面缓存
pub struct PageCache {
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    config: CacheConfig,
    stats: Arc<RwLock<CacheStats>>,
}

/// 缓存统计
#[derive(Debug, Default, Clone)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub current_size: usize,
}

impl PageCache {
    /// 创建新的页面缓存
    pub fn new(config: CacheConfig) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(RwLock::new(CacheStats::default())),
        }
    }

    /// 获取缓存键
    fn cache_key(&self, page_num: u32, zoom_level: f32) -> String {
        format!("{}:{:.2}", page_num, zoom_level)
    }

    /// 获取页面（如果在缓存中）
    pub async fn get_page(&self, page_num: u32, zoom_level: f32) -> Option<PdfPage> {
        let key = self.cache_key(page_num, zoom_level);
        let mut cache = self.cache.write().await;
        
        if let Some(entry) = cache.get_mut(&key) {
            // 检查缩放级别是否匹配（允许小的浮点误差）
            if (entry.zoom_level - zoom_level).abs() < 0.01 {
                entry.access_time = std::time::Instant::now();
                
                // 更新统计
                let mut stats = self.stats.write().await;
                stats.hits += 1;
                
                return Some(entry.page.clone());
            }
        }

        // 更新统计
        let mut stats = self.stats.write().await;
        stats.misses += 1;
        
        None
    }

    /// 存储页面到缓存
    pub async fn put_page(&self, page: PdfPage, zoom_level: f32) {
        let key = self.cache_key(page.page_number, zoom_level);
        let mut cache = self.cache.write().await;

        // 如果缓存已满，删除最旧的条目
        if cache.len() >= self.config.max_size {
            self.evict_oldest(&mut cache).await;
        }

        let entry = CacheEntry {
            page,
            access_time: std::time::Instant::now(),
            zoom_level,
        };

        cache.insert(key, entry);
        
        // 更新统计
        let mut stats = self.stats.write().await;
        stats.current_size = cache.len();
    }

    /// 清理缓存中的过期条目
    async fn evict_oldest(&self, cache: &mut HashMap<String, CacheEntry>) {
        if cache.is_empty() {
            return;
        }

        // 找到最旧的条目
        let oldest_key = cache.iter()
            .min_by_key(|(_, entry)| entry.access_time)
            .map(|(key, _)| key.clone());

        if let Some(key) = oldest_key {
            cache.remove(&key);
            
            // 更新统计
            let mut stats = self.stats.write().await;
            stats.evictions += 1;
        }
    }

    /// 清空缓存
    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
        
        let mut stats = self.stats.write().await;
        stats.current_size = 0;
    }

    /// 预渲染页面（用于预加载）
    pub fn should_prerender(&self, current_page: u32, total_pages: u32) -> Vec<u32> {
        let mut pages_to_render = Vec::new();
        
        // 前面的页面
        for i in 1..=self.config.prerender_behind {
            if current_page > i {
                pages_to_render.push(current_page - i);
            }
        }
        
        // 后面的页面
        for i in 1..=self.config.prerender_ahead {
            if current_page + i <= total_pages {
                pages_to_render.push(current_page + i);
            }
        }
        
        pages_to_render
    }

    /// 获取缓存统计
    pub async fn get_stats(&self) -> CacheStats {
        self.stats.read().await.clone()
    }

    /// 获取缓存使用率
    pub async fn get_usage_ratio(&self) -> f32 {
        let cache = self.cache.read().await;
        cache.len() as f32 / self.config.max_size as f32
    }

    /// 检查页面是否在缓存中
    pub async fn contains_page(&self, page_num: u32, zoom_level: f32) -> bool {
        let key = self.cache_key(page_num, zoom_level);
        let cache = self.cache.read().await;
        
        cache.get(&key)
            .map(|entry| (entry.zoom_level - zoom_level).abs() < 0.01)
            .unwrap_or(false)
    }

    /// 移除特定页面的缓存
    pub async fn remove_page(&self, page_num: u32, zoom_level: f32) {
        let key = self.cache_key(page_num, zoom_level);
        let mut cache = self.cache.write().await;
        
        if cache.remove(&key).is_some() {
            let mut stats = self.stats.write().await;
            stats.current_size = cache.len();
        }
    }

    /// 获取缓存中的页面列表
    pub async fn get_cached_pages(&self) -> Vec<(u32, f32)> {
        let cache = self.cache.read().await;
        
        cache.values()
            .map(|entry| (entry.page.page_number, entry.zoom_level))
            .collect()
    }
}

// PDF 页面缓存模块 - 简化版本

use std::collections::HashMap;
use anyhow::Result;

use super::{PageInfo, ImageData};

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
    page_info: PageInfo,
    access_time: std::time::Instant,
    access_count: u32,
}

/// 页面缓存
pub struct PageCache {
    cache: HashMap<String, CacheEntry>,
    config: CacheConfig,
}

impl PageCache {
    /// 创建新的页面缓存
    pub fn new(config: CacheConfig) -> Self {
        Self {
            cache: HashMap::new(),
            config,
        }
    }

    /// 生成缓存键
    fn make_key(page_number: u32, zoom: f32, width: u32, height: u32) -> String {
        format!("{}:{}:{}:{}", page_number, zoom, width, height)
    }

    /// 获取缓存的页面
    pub fn get(&mut self, page_number: u32, zoom: f32, width: u32, height: u32) -> Option<PageInfo> {
        let key = Self::make_key(page_number, zoom, width, height);
        
        if let Some(entry) = self.cache.get_mut(&key) {
            entry.access_time = std::time::Instant::now();
            entry.access_count += 1;
            Some(entry.page_info.clone())
        } else {
            None
        }
    }

    /// 缓存页面
    pub fn insert(&mut self, page_number: u32, zoom: f32, width: u32, height: u32, page_info: PageInfo) {
        let key = Self::make_key(page_number, zoom, width, height);
        
        // 检查是否需要清理缓存
        if self.cache.len() >= self.config.max_size {
            self.cleanup();
        }

        let entry = CacheEntry {
            page_info,
            access_time: std::time::Instant::now(),
            access_count: 1,
        };

        self.cache.insert(key, entry);
    }

    /// 清理缓存
    fn cleanup(&mut self) {
        if self.cache.len() < self.config.max_size {
            return;
        }

        // 找到最久未访问的条目
        let mut entries: Vec<_> = self.cache.iter().collect();
        entries.sort_by(|a, b| a.1.access_time.cmp(&b.1.access_time));

        // 移除最久未访问的条目
        let remove_count = self.cache.len() - self.config.max_size / 2;
        for (key, _) in entries.iter().take(remove_count) {
            self.cache.remove(*key);
        }
    }

    /// 清除所有缓存
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// 获取缓存统计
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            total_entries: self.cache.len(),
            max_size: self.config.max_size,
        }
    }
}

/// 缓存统计
#[derive(Debug, Default)]
pub struct CacheStats {
    pub total_entries: usize,
    pub max_size: usize,
}

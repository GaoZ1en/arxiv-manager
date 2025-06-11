
//! 事件系统模块
//! 
//! 提供应用程序的事件管理和消息传递机制

pub mod search_events;
pub mod download_events;
pub mod ui_events;

pub use search_events::*;
pub use download_events::*;
pub use ui_events::*;

use crate::core::ArxivPaper;
// use crate::database::PaperRecord;
// use crate::config::Config;

// 临时类型定义以避免循环导入
#[derive(Debug, Clone)]
pub struct PaperRecord {
    pub id: i64,
    pub arxiv_id: String,
    pub title: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    // 简化配置结构
}
use std::path::PathBuf;

/// 应用程序主事件类型
#[derive(Debug, Clone)]
pub enum AppEvent {
    /// 搜索相关事件
    Search(SearchEvent),
    
    /// 下载相关事件
    Download(DownloadEvent),
    
    /// UI相关事件
    Ui(UiEvent),
    
    /// 数据库相关事件
    Database(DatabaseEvent),
    
    /// 配置相关事件
    Config(ConfigEvent),
    
    /// 系统事件
    System(SystemEvent),
}

/// 数据库相关事件
#[derive(Debug, Clone)]
pub enum DatabaseEvent {
    /// 加载最近论文
    LoadRecentPapers,
    
    /// 最近论文已加载
    RecentPapersLoaded(Vec<PaperRecord>),
    
    /// 保存论文
    SavePaper(ArxivPaper),
    
    /// 论文已保存
    PaperSaved(String), // arxiv_id
    
    /// 删除论文
    DeletePaper(String), // arxiv_id
    
    /// 论文已删除
    PaperDeleted(String), // arxiv_id
    
    /// 数据库错误
    Error(String),
    
    /// 数据库初始化
    Initialize,
    
    /// 数据库已初始化
    Initialized,
    
    /// 执行备份
    Backup(PathBuf),
    
    /// 备份完成
    BackupCompleted(PathBuf),
    
    /// 导入数据
    Import(PathBuf),
    
    /// 导入完成
    ImportCompleted(usize), // 导入的记录数
}

/// 配置相关事件
#[derive(Debug, Clone)]
pub enum ConfigEvent {
    /// 加载配置
    Load,
    
    /// 配置已加载
    Loaded(Config),
    
    /// 保存配置
    Save(Config),
    
    /// 配置已保存
    Saved,
    
    /// 重置配置
    Reset,
    
    /// 配置已重置
    ResetCompleted,
    
    /// 配置更改
    Changed(Config),
    
    /// 配置验证
    Validate(Config),
    
    /// 配置验证结果
    ValidationResult(bool, Vec<String>), // 是否有效，错误消息列表
    
    /// 配置错误
    Error(String),
}

/// 系统事件
#[derive(Debug, Clone)]
pub enum SystemEvent {
    /// 应用程序启动
    Startup,
    
    /// 应用程序关闭
    Shutdown,
    
    /// 窗口调整大小
    WindowResized(u32, u32), // width, height
    
    /// 窗口最小化
    WindowMinimized,
    
    /// 窗口最大化
    WindowMaximized,
    
    /// 窗口恢复
    WindowRestored,
    
    /// 应用程序获得焦点
    FocusGained,
    
    /// 应用程序失去焦点
    FocusLost,
    
    /// 网络状态变化
    NetworkStatusChanged(NetworkStatus),
    
    /// 磁盘空间不足
    LowDiskSpace(PathBuf, u64), // 路径，剩余字节数
    
    /// 内存使用警告
    HighMemoryUsage(u64), // 内存使用量（字节）
    
    /// 错误发生
    Error(SystemError),
    
    /// 调试信息
    Debug(String),
}

/// 网络状态
#[derive(Debug, Clone, PartialEq)]
pub enum NetworkStatus {
    Online,
    Offline,
    Limited, // 受限连接
}

/// 系统错误
#[derive(Debug, Clone)]
pub struct SystemError {
    pub message: String,
    pub error_type: SystemErrorType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub context: Option<String>,
}

/// 系统错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum SystemErrorType {
    FileSystem,
    Network,
    Database,
    Configuration,
    Memory,
    Permission,
    Unknown,
}

/// 事件监听器特征
pub trait EventListener<T> {
    /// 处理事件
    fn handle_event(&mut self, event: &T) -> Result<(), Box<dyn std::error::Error>>;
}

/// 事件总线
pub struct EventBus {
    /// 搜索事件监听器
    search_listeners: Vec<Box<dyn EventListener<SearchEvent>>>,
    
    /// 下载事件监听器
    download_listeners: Vec<Box<dyn EventListener<DownloadEvent>>>,
    
    /// UI事件监听器
    ui_listeners: Vec<Box<dyn EventListener<UiEvent>>>,
    
    /// 数据库事件监听器
    database_listeners: Vec<Box<dyn EventListener<DatabaseEvent>>>,
    
    /// 配置事件监听器
    config_listeners: Vec<Box<dyn EventListener<ConfigEvent>>>,
    
    /// 系统事件监听器
    system_listeners: Vec<Box<dyn EventListener<SystemEvent>>>,
}

impl EventBus {
    /// 创建新的事件总线
    pub fn new() -> Self {
        Self {
            search_listeners: Vec::new(),
            download_listeners: Vec::new(),
            ui_listeners: Vec::new(),
            database_listeners: Vec::new(),
            config_listeners: Vec::new(),
            system_listeners: Vec::new(),
        }
    }
    
    /// 发布事件
    pub fn publish(&mut self, event: AppEvent) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            AppEvent::Search(event) => {
                for listener in &mut self.search_listeners {
                    listener.handle_event(&event)?;
                }
            }
            AppEvent::Download(event) => {
                for listener in &mut self.download_listeners {
                    listener.handle_event(&event)?;
                }
            }
            AppEvent::Ui(event) => {
                for listener in &mut self.ui_listeners {
                    listener.handle_event(&event)?;
                }
            }
            AppEvent::Database(event) => {
                for listener in &mut self.database_listeners {
                    listener.handle_event(&event)?;
                }
            }
            AppEvent::Config(event) => {
                for listener in &mut self.config_listeners {
                    listener.handle_event(&event)?;
                }
            }
            AppEvent::System(event) => {
                for listener in &mut self.system_listeners {
                    listener.handle_event(&event)?;
                }
            }
        }
        Ok(())
    }
    
    /// 添加搜索事件监听器
    pub fn add_search_listener(&mut self, listener: Box<dyn EventListener<SearchEvent>>) {
        self.search_listeners.push(listener);
    }
    
    /// 添加下载事件监听器
    pub fn add_download_listener(&mut self, listener: Box<dyn EventListener<DownloadEvent>>) {
        self.download_listeners.push(listener);
    }
    
    /// 添加UI事件监听器
    pub fn add_ui_listener(&mut self, listener: Box<dyn EventListener<UiEvent>>) {
        self.ui_listeners.push(listener);
    }
    
    /// 添加数据库事件监听器
    pub fn add_database_listener(&mut self, listener: Box<dyn EventListener<DatabaseEvent>>) {
        self.database_listeners.push(listener);
    }
    
    /// 添加配置事件监听器
    pub fn add_config_listener(&mut self, listener: Box<dyn EventListener<ConfigEvent>>) {
        self.config_listeners.push(listener);
    }
    
    /// 添加系统事件监听器
    pub fn add_system_listener(&mut self, listener: Box<dyn EventListener<SystemEvent>>) {
        self.system_listeners.push(listener);
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

/// 事件处理结果
#[derive(Debug, Clone)]
pub enum EventResult {
    /// 事件处理成功
    Success,
    
    /// 事件处理失败
    Error(String),
    
    /// 事件被忽略
    Ignored,
    
    /// 需要异步处理
    Async,
}

/// 批量事件处理器
pub struct BatchEventProcessor {
    events: Vec<AppEvent>,
    batch_size: usize,
    auto_flush: bool,
}

impl BatchEventProcessor {
    /// 创建新的批量事件处理器
    pub fn new(batch_size: usize, auto_flush: bool) -> Self {
        Self {
            events: Vec::with_capacity(batch_size),
            batch_size,
            auto_flush,
        }
    }
    
    /// 添加事件到批次
    pub fn add_event(&mut self, event: AppEvent) {
        self.events.push(event);
        
        if self.auto_flush && self.events.len() >= self.batch_size {
            // 这里应该触发批量处理
            // self.flush();
        }
    }
    
    /// 刷新批次（处理所有事件）
    pub fn flush(&mut self, event_bus: &mut EventBus) -> Result<Vec<EventResult>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        
        for event in self.events.drain(..) {
            match event_bus.publish(event) {
                Ok(_) => results.push(EventResult::Success),
                Err(e) => results.push(EventResult::Error(e.to_string())),
            }
        }
        
        Ok(results)
    }
    
    /// 获取当前批次大小
    pub fn current_batch_size(&self) -> usize {
        self.events.len()
    }
    
    /// 清空批次
    pub fn clear(&mut self) {
        self.events.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct TestSearchListener {
        received_events: Vec<SearchEvent>,
    }
    
    impl EventListener<SearchEvent> for TestSearchListener {
        fn handle_event(&mut self, event: &SearchEvent) -> Result<(), Box<dyn std::error::Error>> {
            self.received_events.push(event.clone());
            Ok(())
        }
    }
    
    #[test]
    fn test_event_bus_creation() {
        let event_bus = EventBus::new();
        assert_eq!(event_bus.search_listeners.len(), 0);
    }
    
    #[test]
    fn test_batch_processor() {
        let mut processor = BatchEventProcessor::new(2, false);
        assert_eq!(processor.current_batch_size(), 0);
        
        processor.add_event(AppEvent::System(SystemEvent::Startup));
        assert_eq!(processor.current_batch_size(), 1);
        
        processor.clear();
        assert_eq!(processor.current_batch_size(), 0);
    }
}

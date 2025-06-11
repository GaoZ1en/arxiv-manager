//! UI事件处理模块
//! 
//! 处理与用户界面交互相关的所有事件，包括：
//! - 标签页管理
//! - 侧边栏状态
//! - 命令面板
//! - 主题切换
//! - 快捷键事件

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// UI相关事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UiEvent {
    /// 标签页事件
    Tab(TabEvent),
    /// 侧边栏事件
    Sidebar(SidebarEvent),
    /// 命令面板事件
    CommandPalette(CommandPaletteEvent),
    /// 主题事件
    Theme(ThemeEvent),
    /// 快捷键事件
    Shortcut(ShortcutEvent),
    /// 设置事件
    Settings(SettingsEvent),
    /// 窗口事件
    Window(WindowEvent),
}

/// 标签页事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TabEvent {
    /// 创建新标签页
    Create {
        tab_type: TabType,
        title: String,
    },
    /// 切换到指定标签页
    Switch { index: usize },
    /// 关闭标签页
    Close { index: usize },
    /// 重命名标签页
    Rename {
        index: usize,
        new_title: String,
    },
    /// 移动标签页
    Move {
        from_index: usize,
        to_index: usize,
    },
    /// 标签页内容更新
    ContentUpdate {
        index: usize,
        content_type: String,
        data: HashMap<String, String>,
    },
}

/// 标签页类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TabType {
    Search,
    Library,
    Downloads,
    Settings,
    PaperDetail,
    SearchResult,
}

/// 侧边栏事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SidebarEvent {
    /// 切换显示/隐藏
    Toggle,
    /// 展开/折叠
    Expand { expanded: bool },
    /// 选择项目
    SelectItem { item_id: String },
    /// 搜索历史选择
    SelectHistory { history_id: String },
    /// 收藏夹操作
    Favorites(FavoriteEvent),
}

/// 收藏夹事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FavoriteEvent {
    Add { paper_id: String },
    Remove { paper_id: String },
    CreateFolder { name: String },
    MoveToFolder { paper_id: String, folder_id: String },
}

/// 命令面板事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandPaletteEvent {
    /// 显示命令面板
    Show,
    /// 隐藏命令面板
    Hide,
    /// 搜索命令
    SearchCommand { query: String },
    /// 执行命令
    ExecuteCommand { command_id: String },
    /// 更新建议
    UpdateSuggestions { suggestions: Vec<CommandSuggestion> },
}

/// 命令建议
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandSuggestion {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub shortcut: Option<String>,
    pub category: String,
}

/// 主题事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThemeEvent {
    /// 切换主题
    Switch { theme_name: String },
    /// 切换暗色/亮色模式
    ToggleDarkMode,
    /// 自定义主题颜色
    CustomizeColor {
        color_key: String,
        color_value: String,
    },
    /// 重置主题
    Reset,
    /// 应用系统主题
    ApplySystem,
}

/// 快捷键事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShortcutEvent {
    /// 注册新快捷键
    Register {
        action: String,
        keys: Vec<String>,
    },
    /// 触发快捷键
    Trigger { action: String },
    /// 更新快捷键绑定
    Update {
        action: String,
        new_keys: Vec<String>,
    },
    /// 重置所有快捷键
    ResetAll,
    /// 快捷键冲突
    Conflict {
        action1: String,
        action2: String,
        keys: Vec<String>,
    },
}

/// 设置事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SettingsEvent {
    /// 更新设置
    Update {
        category: String,
        key: String,
        value: String,
    },
    /// 重置设置
    Reset { category: Option<String> },
    /// 导入设置
    Import { file_path: String },
    /// 导出设置
    Export { file_path: String },
    /// 验证设置
    Validate { category: String },
}

/// 窗口事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WindowEvent {
    /// 窗口大小改变
    Resize { width: u32, height: u32 },
    /// 窗口移动
    Move { x: i32, y: i32 },
    /// 最小化
    Minimize,
    /// 最大化
    Maximize,
    /// 全屏
    Fullscreen { enabled: bool },
    /// 窗口关闭
    Close,
    /// 焦点变化
    Focus { focused: bool },
}

/// UI事件构建器
#[derive(Debug, Default)]
pub struct UiEventBuilder {
    event_type: Option<String>,
    target: Option<String>,
    data: HashMap<String, String>,
    timestamp: Option<SystemTime>,
}

impl UiEventBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn event_type(mut self, event_type: impl Into<String>) -> Self {
        self.event_type = Some(event_type.into());
        self
    }

    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    pub fn data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.data.insert(key.into(), value.into());
        self
    }

    pub fn timestamp(mut self, timestamp: SystemTime) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn build_tab_event(self, tab_event: TabEvent) -> UiEvent {
        UiEvent::Tab(tab_event)
    }

    pub fn build_sidebar_event(self, sidebar_event: SidebarEvent) -> UiEvent {
        UiEvent::Sidebar(sidebar_event)
    }

    pub fn build_theme_event(self, theme_event: ThemeEvent) -> UiEvent {
        UiEvent::Theme(theme_event)
    }
}

/// UI事件聚合器 - 统计和分析UI事件
#[derive(Debug)]
pub struct UiEventAggregator {
    tab_events: HashMap<String, u32>,
    sidebar_interactions: u32,
    command_palette_usage: u32,
    theme_switches: u32,
    shortcut_usage: HashMap<String, u32>,
    session_start: SystemTime,
}

impl UiEventAggregator {
    pub fn new() -> Self {
        Self {
            tab_events: HashMap::new(),
            sidebar_interactions: 0,
            command_palette_usage: 0,
            theme_switches: 0,
            shortcut_usage: HashMap::new(),
            session_start: SystemTime::now(),
        }
    }

    /// 处理UI事件并更新统计
    pub fn handle_event(&mut self, event: &UiEvent) {
        match event {
            UiEvent::Tab(tab_event) => {
                let event_type = match tab_event {
                    TabEvent::Create { .. } => "create",
                    TabEvent::Switch { .. } => "switch",
                    TabEvent::Close { .. } => "close",
                    TabEvent::Rename { .. } => "rename",
                    TabEvent::Move { .. } => "move",
                    TabEvent::ContentUpdate { .. } => "content_update",
                };
                *self.tab_events.entry(event_type.to_string()).or_insert(0) += 1;
            }
            UiEvent::Sidebar(_) => {
                self.sidebar_interactions += 1;
            }
            UiEvent::CommandPalette(_) => {
                self.command_palette_usage += 1;
            }
            UiEvent::Theme(_) => {
                self.theme_switches += 1;
            }
            UiEvent::Shortcut(ShortcutEvent::Trigger { action }) => {
                *self.shortcut_usage.entry(action.clone()).or_insert(0) += 1;
            }
            _ => {}
        }
    }

    /// 获取会话统计
    pub fn get_session_stats(&self) -> UiSessionStats {
        UiSessionStats {
            session_duration: self.session_start.elapsed().unwrap_or(Duration::ZERO),
            tab_events: self.tab_events.clone(),
            sidebar_interactions: self.sidebar_interactions,
            command_palette_usage: self.command_palette_usage,
            theme_switches: self.theme_switches,
            shortcut_usage: self.shortcut_usage.clone(),
        }
    }

    /// 重置统计
    pub fn reset(&mut self) {
        self.tab_events.clear();
        self.sidebar_interactions = 0;
        self.command_palette_usage = 0;
        self.theme_switches = 0;
        self.shortcut_usage.clear();
        self.session_start = SystemTime::now();
    }

    /// 获取最常用的快捷键
    pub fn get_most_used_shortcuts(&self, limit: usize) -> Vec<(String, u32)> {
        let mut shortcuts: Vec<_> = self.shortcut_usage.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        shortcuts.sort_by(|a, b| b.1.cmp(&a.1));
        shortcuts.truncate(limit);
        shortcuts
    }

    /// 获取标签页使用模式
    pub fn get_tab_usage_pattern(&self) -> HashMap<String, f64> {
        let total: u32 = self.tab_events.values().sum();
        if total == 0 {
            return HashMap::new();
        }

        self.tab_events.iter()
            .map(|(k, v)| (k.clone(), *v as f64 / total as f64))
            .collect()
    }
}

/// UI会话统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSessionStats {
    pub session_duration: Duration,
    pub tab_events: HashMap<String, u32>,
    pub sidebar_interactions: u32,
    pub command_palette_usage: u32,
    pub theme_switches: u32,
    pub shortcut_usage: HashMap<String, u32>,
}

/// UI事件会话管理器
#[derive(Debug)]
pub struct UiEventSession {
    pub id: String,
    pub start_time: SystemTime,
    pub events: Vec<UiEvent>,
    pub aggregator: UiEventAggregator,
}

impl UiEventSession {
    pub fn new(id: String) -> Self {
        Self {
            id,
            start_time: SystemTime::now(),
            events: Vec::new(),
            aggregator: UiEventAggregator::new(),
        }
    }

    /// 添加事件到会话
    pub fn add_event(&mut self, event: UiEvent) {
        self.aggregator.handle_event(&event);
        self.events.push(event);
    }

    /// 获取会话持续时间
    pub fn duration(&self) -> Duration {
        self.start_time.elapsed().unwrap_or(Duration::ZERO)
    }

    /// 获取事件数量
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// 导出会话数据
    pub fn export_session(&self) -> Result<String, Box<dyn std::error::Error>> {
        let session_data = serde_json::to_string_pretty(&SessionExport {
            id: self.id.clone(),
            start_time: self.start_time,
            duration: self.duration(),
            events: &self.events,
            stats: self.aggregator.get_session_stats(),
        })?;
        Ok(session_data)
    }
}

/// 会话导出数据结构
#[derive(Debug, Serialize)]
struct SessionExport<'a> {
    id: String,
    start_time: SystemTime,
    duration: Duration,
    events: &'a [UiEvent],
    stats: UiSessionStats,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_event_builder() {
        let event = UiEventBuilder::new()
            .event_type("tab_create")
            .target("search_tab")
            .data("title", "新搜索")
            .build_tab_event(TabEvent::Create {
                tab_type: TabType::Search,
                title: "新搜索".to_string(),
            });

        match event {
            UiEvent::Tab(TabEvent::Create { tab_type, title }) => {
                assert!(matches!(tab_type, TabType::Search));
                assert_eq!(title, "新搜索");
            }
            _ => panic!("Expected TabEvent::Create"),
        }
    }

    #[test]
    fn test_ui_event_aggregator() {
        let mut aggregator = UiEventAggregator::new();
        
        // 添加一些事件
        let tab_create = UiEvent::Tab(TabEvent::Create {
            tab_type: TabType::Search,
            title: "Test".to_string(),
        });
        let tab_switch = UiEvent::Tab(TabEvent::Switch { index: 1 });
        let shortcut = UiEvent::Shortcut(ShortcutEvent::Trigger {
            action: "search".to_string(),
        });

        aggregator.handle_event(&tab_create);
        aggregator.handle_event(&tab_switch);
        aggregator.handle_event(&shortcut);

        let stats = aggregator.get_session_stats();
        assert_eq!(stats.tab_events.get("create"), Some(&1));
        assert_eq!(stats.tab_events.get("switch"), Some(&1));
        assert_eq!(stats.shortcut_usage.get("search"), Some(&1));
    }

    #[test]
    fn test_ui_event_session() {
        let mut session = UiEventSession::new("test_session".to_string());
        
        let event = UiEvent::Tab(TabEvent::Create {
            tab_type: TabType::Search,
            title: "Test".to_string(),
        });
        
        session.add_event(event);
        assert_eq!(session.event_count(), 1);
        
        let export = session.export_session();
        assert!(export.is_ok());
    }

    #[test]
    fn test_most_used_shortcuts() {
        let mut aggregator = UiEventAggregator::new();
        
        // 添加多个快捷键事件
        for _ in 0..5 {
            aggregator.handle_event(&UiEvent::Shortcut(ShortcutEvent::Trigger {
                action: "search".to_string(),
            }));
        }
        
        for _ in 0..3 {
            aggregator.handle_event(&UiEvent::Shortcut(ShortcutEvent::Trigger {
                action: "download".to_string(),
            }));
        }
        
        let most_used = aggregator.get_most_used_shortcuts(10);
        assert_eq!(most_used.len(), 2);
        assert_eq!(most_used[0], ("search".to_string(), 5));
        assert_eq!(most_used[1], ("download".to_string(), 3));
    }
}

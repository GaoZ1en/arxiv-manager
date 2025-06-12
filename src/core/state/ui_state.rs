// UI状态管理 - 界面相关状态

use std::time::Instant;
use crate::core::{Tab, TabContent, AppSettings};

/// UI相关状态
#[derive(Debug)]
pub struct UiState {
    // 标签页状态
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
    #[allow(dead_code)]
    pub next_tab_id: usize,
    
    // 界面显示状态
    pub sidebar_visible: bool,
    pub advanced_search_visible: bool,
    
    // 命令栏状态
    pub command_palette_visible: bool,
    pub command_palette_input: String,
    pub command_suggestions: Vec<crate::core::Command>,
    pub selected_command_index: Option<usize>,
    
    // 快捷键编辑状态
    #[allow(dead_code)]
    pub editing_shortcut: Option<String>,
    #[allow(dead_code)]
    pub shortcut_input: String,
    
    // 交互状态
    pub last_interaction: Option<Instant>,
    
    // 应用设置
    #[allow(dead_code)]
    pub settings: AppSettings,
}

impl UiState {
    pub fn new() -> Self {
        let tabs = vec![
            Tab::new(0, "Search".to_string(), TabContent::Search),
            Tab::new(1, "Library".to_string(), TabContent::Library),
            Tab::new(2, "Downloads".to_string(), TabContent::Downloads),
            Tab::new(3, "Settings".to_string(), TabContent::Settings),
        ];
        
        Self {
            tabs,
            active_tab: 0,
            next_tab_id: 4,
            sidebar_visible: true,
            advanced_search_visible: false,
            command_palette_visible: false,
            command_palette_input: String::new(),
            command_suggestions: Vec::new(),
            selected_command_index: None,
            editing_shortcut: None,
            shortcut_input: String::new(),
            last_interaction: None,
            settings: AppSettings::default(),
        }
    }
    
    /// 获取当前活动标签
    #[allow(dead_code)]
    pub fn current_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.active_tab)
    }
    
    /// 获取当前活动标签（可变引用）
    #[allow(dead_code)]
    pub fn current_tab_mut(&mut self) -> Option<&mut Tab> {
        self.tabs.get_mut(self.active_tab)
    }
    
    /// 切换侧边栏显示状态
    #[allow(dead_code)]
    pub fn toggle_sidebar(&mut self) {
        self.sidebar_visible = !self.sidebar_visible;
    }
    
    /// 切换高级搜索显示状态
    #[allow(dead_code)]
    pub fn toggle_advanced_search(&mut self) {
        self.advanced_search_visible = !self.advanced_search_visible;
    }
    
    /// 切换命令栏显示状态
    #[allow(dead_code)]
    pub fn toggle_command_palette(&mut self) {
        self.command_palette_visible = !self.command_palette_visible;
        
        if self.command_palette_visible {
            self.command_palette_input.clear();
            self.command_suggestions.clear();
            self.selected_command_index = None;
        }
    }
    
    /// 更新最后交互时间
    #[allow(dead_code)]
    pub fn update_last_interaction(&mut self) {
        self.last_interaction = Some(Instant::now());
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self::new()
    }
}

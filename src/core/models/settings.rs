// 设置和快捷键相关的数据模型

use super::{Theme, Language, SearchField, SortBy, SortOrder};

// 应用设置
#[derive(Debug, Clone)]
pub struct AppSettings {
    pub theme: Theme,
    pub download_directory: String,
    pub auto_download: bool,
    pub max_concurrent_downloads: u32,
    pub show_abstracts_in_search: bool,
    pub default_search_field: SearchField,
    pub default_sort_by: SortBy,
    pub default_sort_order: SortOrder,
    pub default_max_results: u32,
    pub auto_save_searches: bool,
    pub notification_enabled: bool,
    pub check_updates: bool,
    pub language: Language,
    pub shortcuts: KeyboardShortcuts,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: Theme::ModernDark,
            download_directory: "downloads".to_string(),
            auto_download: false,
            max_concurrent_downloads: 3,
            show_abstracts_in_search: true,
            default_search_field: SearchField::All,
            default_sort_by: SortBy::Relevance,
            default_sort_order: SortOrder::Descending,
            default_max_results: 20,
            auto_save_searches: false,
            notification_enabled: true,
            check_updates: true,
            language: Language::English,
            shortcuts: KeyboardShortcuts::default(),
        }
    }
}

// 键盘快捷键配置
#[derive(Debug, Clone)]
pub struct KeyboardShortcuts {
    pub toggle_command_palette: ShortcutKey,
    pub focus_search: ShortcutKey,
    pub quick_save_paper: ShortcutKey,
    pub quick_download_paper: ShortcutKey,
    pub toggle_sidebar: ShortcutKey,
    pub next_tab: ShortcutKey,
    pub previous_tab: ShortcutKey,
    pub close_tab: ShortcutKey,
    pub new_tab: ShortcutKey,
    pub go_to_search: ShortcutKey,
    pub go_to_library: ShortcutKey,
    pub go_to_downloads: ShortcutKey,
    pub go_to_settings: ShortcutKey,
}

impl Default for KeyboardShortcuts {
    fn default() -> Self {
        Self {
            toggle_command_palette: ShortcutKey::new("Ctrl+Shift+P"),
            focus_search: ShortcutKey::new("Ctrl+F"),
            quick_save_paper: ShortcutKey::new("Ctrl+S"),
            quick_download_paper: ShortcutKey::new("Ctrl+D"),
            toggle_sidebar: ShortcutKey::new("Ctrl+B"),
            next_tab: ShortcutKey::new("Ctrl+Tab"),
            previous_tab: ShortcutKey::new("Ctrl+Shift+Tab"),
            close_tab: ShortcutKey::new("Ctrl+W"),
            new_tab: ShortcutKey::new("Ctrl+T"),
            go_to_search: ShortcutKey::new("Ctrl+1"),
            go_to_library: ShortcutKey::new("Ctrl+2"),
            go_to_downloads: ShortcutKey::new("Ctrl+3"),
            go_to_settings: ShortcutKey::new("Ctrl+4"),
        }
    }
}

impl KeyboardShortcuts {
    pub fn get_all_actions(&self) -> Vec<(&'static str, &str, &ShortcutKey)> {
        vec![
            ("toggle_command_palette", "切换命令面板", &self.toggle_command_palette),
            ("focus_search", "聚焦搜索框", &self.focus_search),
            ("quick_save_paper", "快速保存论文", &self.quick_save_paper),
            ("quick_download_paper", "快速下载论文", &self.quick_download_paper),
            ("toggle_sidebar", "切换侧边栏", &self.toggle_sidebar),
            ("next_tab", "下一个标签页", &self.next_tab),
            ("previous_tab", "上一个标签页", &self.previous_tab),
            ("close_tab", "关闭标签页", &self.close_tab),
            ("new_tab", "新建标签页", &self.new_tab),
            ("go_to_search", "转到搜索", &self.go_to_search),
            ("go_to_library", "转到论文库", &self.go_to_library),
            ("go_to_downloads", "转到下载", &self.go_to_downloads),
            ("go_to_settings", "转到设置", &self.go_to_settings),
        ]
    }
}

#[derive(Debug, Clone)]
pub struct ShortcutKey {
    pub display: String,
    #[allow(dead_code)]
    pub modifiers: Vec<String>,
    #[allow(dead_code)]
    pub key: String,
}

impl ShortcutKey {
    pub fn new(display: &str) -> Self {
        let parts: Vec<&str> = display.split('+').collect();
        let mut modifiers = Vec::new();
        let mut key = String::new();
        
        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                key = part.to_string();
            } else {
                modifiers.push(part.to_string());
            }
        }
        
        Self {
            display: display.to_string(),
            modifiers,
            key,
        }
    }

    pub fn is_valid_shortcut(shortcut: &str) -> bool {
        !shortcut.trim().is_empty() && shortcut.contains('+')
    }

    #[allow(dead_code)]
    pub fn parse_shortcut(shortcut: &str) -> Option<Self> {
        if Self::is_valid_shortcut(shortcut) {
            Some(Self::new(shortcut))
        } else {
            None
        }
    }
}

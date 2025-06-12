// 快捷键消息处理器  
// 处理所有与快捷键配置相关的消息

use iced::Task;

use crate::core::{ArxivManager, ShortcutKey};
use crate::core::messages::Message;

pub trait ShortcutHandler {
    fn handle_shortcut_edit_started(&mut self, action: String) -> Task<Message>;
    fn handle_shortcut_edit_cancelled(&mut self) -> Task<Message>;
    fn handle_shortcut_input_changed(&mut self, input: String) -> Task<Message>;
    fn handle_shortcut_confirmed(&mut self) -> Task<Message>;
    fn handle_shortcuts_reset(&mut self) -> Task<Message>;
    
    // 辅助方法
    fn update_shortcut(&mut self, action: &str, shortcut: &str);
}

impl ShortcutHandler for ArxivManager {
    fn handle_shortcut_edit_started(&mut self, action: String) -> Task<Message> {
        self.editing_shortcut = Some(action.clone());
        
        // 预填充当前快捷键
        let current_shortcut = match action.as_str() {
            "toggle_command_palette" => &self.settings.shortcuts.toggle_command_palette.display,
            "focus_search" => &self.settings.shortcuts.focus_search.display,
            "quick_save_paper" => &self.settings.shortcuts.quick_save_paper.display,
            "quick_download_paper" => &self.settings.shortcuts.quick_download_paper.display,
            "toggle_sidebar" => &self.settings.shortcuts.toggle_sidebar.display,
            "next_tab" => &self.settings.shortcuts.next_tab.display,
            "previous_tab" => &self.settings.shortcuts.previous_tab.display,
            "close_tab" => &self.settings.shortcuts.close_tab.display,
            "new_tab" => &self.settings.shortcuts.new_tab.display,
            "go_to_search" => &self.settings.shortcuts.go_to_search.display,
            "go_to_library" => &self.settings.shortcuts.go_to_library.display,
            "go_to_downloads" => &self.settings.shortcuts.go_to_downloads.display,
            "go_to_settings" => &self.settings.shortcuts.go_to_settings.display,
            _ => "",
        };
        
        self.shortcut_input = current_shortcut.to_string();
        Task::none()
    }

    fn handle_shortcut_edit_cancelled(&mut self) -> Task<Message> {
        self.editing_shortcut = None;
        self.shortcut_input.clear();
        Task::none()
    }

    fn handle_shortcut_input_changed(&mut self, input: String) -> Task<Message> {
        self.shortcut_input = input;
        Task::none()
    }

    #[allow(dead_code)]
    fn handle_shortcut_confirmed(&mut self) -> Task<Message> {
        // 克隆值以避免借用检查器问题
        if let Some(action) = self.editing_shortcut.clone() {
            let input = self.shortcut_input.clone();
            if ShortcutKey::is_valid_shortcut(&input) {
                self.update_shortcut(&action, &input);
                self.editing_shortcut = None;
                self.shortcut_input.clear();
            }
        }
        Task::none()
    }

    fn handle_shortcuts_reset(&mut self) -> Task<Message> {
        use crate::core::models::KeyboardShortcuts;
        self.settings.shortcuts = KeyboardShortcuts::default();
        Task::none()
    }

    // 辅助方法实现
    #[allow(dead_code)]
    fn update_shortcut(&mut self, action: &str, shortcut: &str) {
        let new_shortcut = ShortcutKey::new(shortcut);
        
        match action {
            "toggle_command_palette" => self.settings.shortcuts.toggle_command_palette = new_shortcut,
            "focus_search" => self.settings.shortcuts.focus_search = new_shortcut,
            "quick_save_paper" => self.settings.shortcuts.quick_save_paper = new_shortcut,
            "quick_download_paper" => self.settings.shortcuts.quick_download_paper = new_shortcut,
            "toggle_sidebar" => self.settings.shortcuts.toggle_sidebar = new_shortcut,
            "next_tab" => self.settings.shortcuts.next_tab = new_shortcut,
            "previous_tab" => self.settings.shortcuts.previous_tab = new_shortcut,
            "close_tab" => self.settings.shortcuts.close_tab = new_shortcut,
            "new_tab" => self.settings.shortcuts.new_tab = new_shortcut,
            "go_to_search" => self.settings.shortcuts.go_to_search = new_shortcut,
            "go_to_library" => self.settings.shortcuts.go_to_library = new_shortcut,
            "go_to_downloads" => self.settings.shortcuts.go_to_downloads = new_shortcut,
            "go_to_settings" => self.settings.shortcuts.go_to_settings = new_shortcut,
            _ => {}
        }
    }
}

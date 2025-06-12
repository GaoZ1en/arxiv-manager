// 设置消息处理器
// 处理所有与应用设置相关的消息

use iced::Task;

use crate::core::ArxivManager;
use crate::core::messages::Message;
use crate::core::models::{Theme, Language, SearchField, SortBy, SortOrder};

pub trait SettingsHandler {
    fn handle_theme_changed(&mut self, theme: Theme) -> Task<Message>;
    fn handle_language_changed(&mut self, language: Language) -> Task<Message>;
    fn handle_download_directory_changed(&mut self, path: String) -> Task<Message>;
    fn handle_auto_download_toggled(&mut self) -> Task<Message>;
    fn handle_max_concurrent_downloads_changed(&mut self, value: String) -> Task<Message>;
    fn handle_show_abstracts_toggled(&mut self) -> Task<Message>;
    fn handle_default_search_field_changed(&mut self, field: SearchField) -> Task<Message>;
    fn handle_default_sort_by_changed(&mut self, sort_by: SortBy) -> Task<Message>;
    fn handle_default_sort_order_changed(&mut self, order: SortOrder) -> Task<Message>;
    fn handle_default_max_results_changed(&mut self, value: String) -> Task<Message>;
    fn handle_auto_save_searches_toggled(&mut self) -> Task<Message>;
    fn handle_notification_toggled(&mut self) -> Task<Message>;
    fn handle_check_updates_toggled(&mut self) -> Task<Message>;
    fn handle_settings_reset(&mut self) -> Task<Message>;
    fn handle_settings_export(&mut self) -> Task<Message>;
    fn handle_settings_import(&mut self, path: String) -> Task<Message>;
    // 新增字体和缩放处理方法
    fn handle_font_family_changed(&mut self, font_family: String) -> Task<Message>;
    fn handle_font_size_changed(&mut self, font_size: String) -> Task<Message>;
    fn handle_ui_scale_changed(&mut self, ui_scale: String) -> Task<Message>;
}

impl SettingsHandler for ArxivManager {
    fn handle_theme_changed(&mut self, theme: Theme) -> Task<Message> {
        self.settings.theme = theme;
        Task::none()
    }

    fn handle_language_changed(&mut self, language: Language) -> Task<Message> {
        self.settings.language = language;
        Task::none()
    }

    fn handle_download_directory_changed(&mut self, path: String) -> Task<Message> {
        self.settings.download_directory = path;
        Task::none()
    }

    fn handle_auto_download_toggled(&mut self) -> Task<Message> {
        self.settings.auto_download = !self.settings.auto_download;
        Task::none()
    }

    fn handle_max_concurrent_downloads_changed(&mut self, value: String) -> Task<Message> {
        if let Ok(num) = value.parse::<u32>() {
            self.settings.max_concurrent_downloads = num.clamp(1, 10);
        }
        Task::none()
    }

    fn handle_show_abstracts_toggled(&mut self) -> Task<Message> {
        self.settings.show_abstracts_in_search = !self.settings.show_abstracts_in_search;
        Task::none()
    }

    fn handle_default_search_field_changed(&mut self, field: SearchField) -> Task<Message> {
        self.settings.default_search_field = field;
        Task::none()
    }

    fn handle_default_sort_by_changed(&mut self, sort_by: SortBy) -> Task<Message> {
        self.settings.default_sort_by = sort_by;
        Task::none()
    }

    fn handle_default_sort_order_changed(&mut self, order: SortOrder) -> Task<Message> {
        self.settings.default_sort_order = order;
        Task::none()
    }

    fn handle_default_max_results_changed(&mut self, value: String) -> Task<Message> {
        if let Ok(num) = value.parse::<u32>() {
            self.settings.default_max_results = num.clamp(1, 100);
        }
        Task::none()
    }

    fn handle_auto_save_searches_toggled(&mut self) -> Task<Message> {
        self.settings.auto_save_searches = !self.settings.auto_save_searches;
        Task::none()
    }

    fn handle_notification_toggled(&mut self) -> Task<Message> {
        self.settings.notification_enabled = !self.settings.notification_enabled;
        Task::none()
    }

    fn handle_check_updates_toggled(&mut self) -> Task<Message> {
        self.settings.check_updates = !self.settings.check_updates;
        Task::none()
    }

    fn handle_settings_reset(&mut self) -> Task<Message> {
        use crate::core::models::AppSettings;
        self.settings = AppSettings::default();
        Task::none()
    }

    fn handle_settings_export(&mut self) -> Task<Message> {
        // TODO: 实现设置导出功能
        println!("Exporting settings...");
        Task::none()
    }

    fn handle_settings_import(&mut self, path: String) -> Task<Message> {
        // TODO: 实现设置导入功能
        println!("Importing settings from: {}", path);
        Task::none()
    }

    fn handle_font_family_changed(&mut self, font_family: String) -> Task<Message> {
        self.settings.font_family = font_family;
        Task::none()
    }

    fn handle_font_size_changed(&mut self, font_size: String) -> Task<Message> {
        if let Ok(size) = font_size.parse::<f32>() {
            self.settings.font_size = size.clamp(8.0, 72.0);
        }
        Task::none()
    }

    fn handle_ui_scale_changed(&mut self, ui_scale: String) -> Task<Message> {
        if let Ok(scale) = ui_scale.parse::<f32>() {
            self.settings.ui_scale = scale.clamp(0.5, 3.0);
        }
        Task::none()
    }
}

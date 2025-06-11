// 下载设置页面 - 下载目录和相关配置

use iced::widget::{text_input, checkbox};
use iced::Element;

use crate::core::app_state::ArxivManager;
use crate::core::messages::Message;
use crate::ui::style::{text_input_dynamic_style, checkbox_dynamic_style};
use super::components::settings_section::create_settings_section_with_colors;
use super::components::setting_row::create_setting_row;

/// 创建下载设置区域
pub fn create_downloads_section(app: &ArxivManager) -> Element<'_, Message> {
    let theme_colors = app.theme_colors();
    let current_font = app.current_font();
    let base_font_size = app.current_font_size();
    
    create_settings_section_with_colors(
        "📥 Downloads",
        theme_colors.success_color,
        vec![
            create_setting_row(
                "Download Directory:",
                text_input("Path to download directory", &app.settings.download_directory)
                    .on_input(Message::DownloadDirectoryChanged)
                    .style(text_input_dynamic_style(&app.settings.theme))
                    .size(base_font_size)
                    .font(current_font)
                    .into(),
                app
            ),
            create_setting_row(
                "Auto Download:",
                checkbox("Automatically download papers when saved", app.settings.auto_download)
                    .on_toggle(|_| Message::AutoDownloadToggled)
                    .style(checkbox_dynamic_style(&app.settings.theme))
                    .size(base_font_size)
                    .font(current_font)
                    .text_size(base_font_size)
                    .into(),
                app
            ),
            create_setting_row(
                "Max Concurrent Downloads:",
                text_input("1-10", &app.settings.max_concurrent_downloads.to_string())
                    .on_input(Message::MaxConcurrentDownloadsChanged)
                    .style(text_input_dynamic_style(&app.settings.theme))
                    .size(base_font_size)
                    .font(current_font)
                    .into(),
                app
            ),
        ],
        app,
    )
}

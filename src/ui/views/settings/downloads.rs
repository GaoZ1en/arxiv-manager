// 下载设置页面 - 下载目录和相关配置

use iced::widget::{text_input, checkbox};
use iced::{Element, Background, Border, Color};

use crate::core::app_state::ArxivManager;
use crate::core::messages::Message;
use crate::ui::theme::*;
use super::components::settings_section::create_settings_section;
use super::components::setting_row::create_setting_row;

/// 创建下载设置区域
pub fn create_downloads_section(app: &ArxivManager) -> Element<Message> {
    create_settings_section(
        "Downloads",
        GRUVBOX_GREEN,
        vec![
            create_setting_row(
                "Download Directory:",
                text_input("Path to download directory", &app.settings.download_directory)
                    .on_input(Message::DownloadDirectoryChanged)
                    .style(text_input_style())
                    .into()
            ),
            create_setting_row(
                "Auto Download:",
                checkbox("Automatically download papers when saved", app.settings.auto_download)
                    .on_toggle(|_| Message::AutoDownloadToggled)
                    .style(checkbox_style())
                    .into()
            ),
            create_setting_row(
                "Max Concurrent Downloads:",
                text_input("1-10", &app.settings.max_concurrent_downloads.to_string())
                    .on_input(Message::MaxConcurrentDownloadsChanged)
                    .style(text_input_style())
                    .into()
            ),
        ]
    )
}

/// TextInput组件的样式
fn text_input_style() -> impl Fn(&iced::Theme, iced::widget::text_input::Status) -> iced::widget::text_input::Style {
    |_theme, status| iced::widget::text_input::Style {
        background: Background::Color(GRUVBOX_BG),
        border: Border {
            color: match status {
                iced::widget::text_input::Status::Focused => GRUVBOX_GREEN,
                _ => GRUVBOX_BORDER,
            },
            width: 1.0,
            radius: 4.0.into(),
        },
        icon: Color::TRANSPARENT,
        placeholder: GRUVBOX_TEXT_MUTED,
        value: GRUVBOX_TEXT,
        selection: GRUVBOX_GREEN,
    }
}

/// Checkbox组件的样式
fn checkbox_style() -> impl Fn(&iced::Theme, iced::widget::checkbox::Status) -> iced::widget::checkbox::Style {
    |_theme, _status| iced::widget::checkbox::Style {
        background: Background::Color(GRUVBOX_BG),
        icon_color: GRUVBOX_GREEN,
        border: Border {
            color: GRUVBOX_BORDER,
            width: 1.0,
            radius: 2.0.into(),
        },
        text_color: Some(GRUVBOX_TEXT),
    }
}

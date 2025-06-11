// 下载设置页面 - 下载目录和相关配置

use iced::widget::{text_input, checkbox};
use iced::{Element, Background, Border, Color};

use crate::core::app_state::ArxivManager;
use crate::core::messages::Message;
use super::components::settings_section::{create_settings_section, create_settings_section_with_colors};
use super::components::setting_row::create_setting_row;

/// 创建下载设置区域
pub fn create_downloads_section(app: &ArxivManager) -> Element<'_, Message> {
    let theme_colors = app.theme_colors();
    
    create_settings_section_with_colors(
        "📥 Downloads",
        theme_colors.success_color,
        vec![
            create_setting_row(
                "Download Directory:",
                text_input("Path to download directory", &app.settings.download_directory)
                    .on_input(Message::DownloadDirectoryChanged)
                    .style(text_input_dynamic_style(&app.settings.theme))
                    .into()
            ),
            create_setting_row(
                "Auto Download:",
                checkbox("Automatically download papers when saved", app.settings.auto_download)
                    .on_toggle(|_| Message::AutoDownloadToggled)
                    .style(checkbox_dynamic_style(&app.settings.theme))
                    .into()
            ),
            create_setting_row(
                "Max Concurrent Downloads:",
                text_input("1-10", &app.settings.max_concurrent_downloads.to_string())
                    .on_input(Message::MaxConcurrentDownloadsChanged)
                    .style(text_input_dynamic_style(&app.settings.theme))
                    .into()
            ),
        ],
        theme_colors,
    )
}

/// TextInput组件的动态样式
fn text_input_dynamic_style(theme: &crate::core::models::Theme) -> impl Fn(&iced::Theme, iced::widget::text_input::Status) -> iced::widget::text_input::Style {
    use crate::ui::theme::get_theme_colors;
    let colors = get_theme_colors(theme);
    move |_theme, status| iced::widget::text_input::Style {
        background: Background::Color(colors.dark_bg),
        border: Border {
            color: match status {
                iced::widget::text_input::Status::Focused => colors.success_color,
                _ => colors.border_color,
            },
            width: 1.0,
            radius: 4.0.into(),
        },
        icon: Color::TRANSPARENT,
        placeholder: colors.text_muted,
        value: colors.text_primary,
        selection: colors.success_color,
    }
}

/// Checkbox组件的动态样式
fn checkbox_dynamic_style(theme: &crate::core::models::Theme) -> impl Fn(&iced::Theme, iced::widget::checkbox::Status) -> iced::widget::checkbox::Style {
    use crate::ui::theme::get_theme_colors;
    let colors = get_theme_colors(theme);
    move |_theme, _status| iced::widget::checkbox::Style {
        background: Background::Color(colors.dark_bg),
        icon_color: colors.success_color,
        border: Border {
            color: colors.border_color,
            width: 1.0,
            radius: 2.0.into(),
        },
        text_color: Some(colors.text_primary),
    }
}

// 外观设置页面 - 主题和语言配置

use iced::widget::pick_list;
use iced::{Element, Background, Border};

use crate::core::app_state::ArxivManager;
use crate::core::models::{Theme, Language};
use crate::core::messages::Message;
use crate::ui::theme::*;
use super::components::settings_section::create_settings_section;
use super::components::setting_row::create_setting_row;

/// 创建外观设置区域
pub fn create_appearance_section(app: &ArxivManager) -> Element<'_, Message> {
    create_settings_section(
        "Appearance",
        GRUVBOX_BLUE,
        vec![
            create_setting_row(
                "Theme:",
                pick_list(
                    Theme::all_variants(),
                    Some(app.settings.theme.clone()),
                    Message::ThemeChanged,
                )
                .placeholder("Select theme...")
                .style(pick_list_style())
                .into()
            ),
            create_setting_row(
                "Language:",
                pick_list(
                    Language::all_variants(),
                    Some(app.settings.language.clone()),
                    Message::LanguageChanged,
                )
                .placeholder("Select language...")
                .style(pick_list_style())
                .into()
            ),
        ]
    )
}

/// PickList组件的样式
fn pick_list_style() -> impl Fn(&iced::Theme, iced::widget::pick_list::Status) -> iced::widget::pick_list::Style {
    |_theme, status| iced::widget::pick_list::Style {
        text_color: GRUVBOX_TEXT,
        background: Background::Color(GRUVBOX_BG),
        border: Border {
            color: match status {
                iced::widget::pick_list::Status::Active => GRUVBOX_BORDER,
                iced::widget::pick_list::Status::Hovered => GRUVBOX_GREEN,
                iced::widget::pick_list::Status::Opened => GRUVBOX_GREEN,
            },
            width: 1.0,
            radius: 4.0.into(),
        },
        handle_color: GRUVBOX_TEXT,
        placeholder_color: GRUVBOX_TEXT_MUTED,
    }
}

// 外观设置页面 - 主题和语言配置

use iced::widget::{pick_list, container, column, row, text, horizontal_space};
use iced::{Element, Background, Border};

use crate::core::app_state::ArxivManager;
use crate::core::models::{Theme, Language};
use crate::core::messages::Message;
use crate::ui::theme::{get_theme_preview};
use crate::ui::style::pick_list_dynamic_style;
use super::components::settings_section::{create_settings_section, create_settings_section_with_colors};
use super::components::setting_row::create_setting_row;

/// 创建外观设置区域
pub fn create_appearance_section(app: &ArxivManager) -> Element<'_, Message> {
    let theme_colors = app.theme_colors();
    
    create_settings_section_with_colors(
        "🎨 Appearance",
        theme_colors.info_color,
        vec![
            create_theme_selector_row(app),
            create_setting_row(
                "Language:",
                pick_list(
                    Language::all_variants(),
                    Some(app.settings.language.clone()),
                    Message::LanguageChanged,
                )
                .placeholder("Select language...")
                .style(pick_list_dynamic_style(&app.settings.theme))
                .into()
            ),
            create_theme_preview_row(&app.settings.theme),
        ],
        theme_colors,
    )
}

/// 创建主题选择器行，包含分类显示
fn create_theme_selector_row(app: &ArxivManager) -> Element<'_, Message> {
    let theme_colors = app.theme_colors();
    
    let theme_selector = pick_list(
        Theme::all_variants(),
        Some(app.settings.theme.clone()),
        Message::ThemeChanged,
    )
    .placeholder("Select theme...")
    .style(pick_list_dynamic_style(&app.settings.theme));

    let current_theme_info = container(
        column![
            text(app.settings.theme.display_name()).size(14).color(theme_colors.text_primary),
            text(format!("Category: {}", app.settings.theme.category())).size(12).color(theme_colors.text_muted),
        ]
        .spacing(4)
    )
    .padding(8)
    .style(theme_info_dynamic_style(&app.settings.theme));

    create_setting_row(
        "Theme:",
        column![
            theme_selector,
            current_theme_info
        ]
        .spacing(8)
        .into()
    )
}

/// 创建主题预览行
fn create_theme_preview_row(theme: &Theme) -> Element<'_, Message> {
    let (bg_color, text_color, accent_color) = get_theme_preview(theme);
    
    let preview_container = container(
        row![
            // 背景色预览
            container(text("BG").color(text_color).size(12))
                .padding(8)
                .style(move |_| container::Style {
                    background: Some(Background::Color(bg_color)),
                    border: Border {
                        width: 1.0,
                        color: accent_color,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                }),
            
            // 文本色预览  
            container(text("Text").color(text_color).size(12))
                .padding(8)
                .style(move |_| container::Style {
                    background: Some(Background::Color(bg_color)),
                    border: Border {
                        width: 1.0,
                        color: text_color,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                }),
                
            // 强调色预览
            container(text("Accent").color(bg_color).size(12))
                .padding(8)
                .style(move |_| container::Style {
                    background: Some(Background::Color(accent_color)),
                    border: Border {
                        width: 1.0,
                        color: accent_color,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                }),
                
            horizontal_space(),
            
            // 主题类型指示器
            container(
                text(if theme.is_dark() { "🌙 Dark" } else { "☀️ Light" })
                    .size(12)
                    .color(text_color) // 使用动态文本颜色
            ).padding(4),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center)
    )
    .padding(12)
    .style(preview_container_dynamic_style(theme));

    create_setting_row(
        "Preview:",
        preview_container.into()
    )
}

/// 主题信息容器动态样式
fn theme_info_dynamic_style(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    use crate::ui::theme::get_theme_colors;
    let colors = get_theme_colors(theme);
    move |_theme| container::Style {
        background: Some(Background::Color(colors.dark_bg_secondary)),
        border: Border {
            color: colors.border_color,
            width: 1.0,
            radius: 6.0.into(),
        },
        text_color: Some(colors.text_primary),
        shadow: iced::Shadow::default(),
    }
}

/// 预览容器动态样式
fn preview_container_dynamic_style(theme: &Theme) -> impl Fn(&iced::Theme) -> container::Style {
    use crate::ui::theme::get_theme_colors;
    let colors = get_theme_colors(theme);
    move |_theme| container::Style {
        background: Some(Background::Color(colors.sidebar_bg)),
        border: Border {
            color: colors.border_color,
            width: 1.0,
            radius: 8.0.into(),
        },
        text_color: Some(colors.text_primary),
        shadow: iced::Shadow::default(),
    }
}

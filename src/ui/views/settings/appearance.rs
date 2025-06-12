// 外观设置页面 - 主题、语言、字体和缩放配置

use iced::widget::{pick_list, container, column, row, text, horizontal_space, text_input, slider};
use iced::{Element, Background, Border};

use crate::core::app_state::ArxivManager;
use crate::core::models::{Theme, Language};
use crate::core::messages::Message;
use crate::ui::theme::{get_theme_preview};
use crate::ui::style::{pick_list_dynamic_style, text_input_dynamic_style};
use super::components::settings_section::create_settings_section_with_colors;
use super::components::setting_row::create_setting_row;


/// 创建外观设置区域
pub fn create_appearance_section(app: &ArxivManager) -> Element<'_, Message> {
    let theme_colors = app.theme_colors();
    
    create_settings_section_with_colors(
        "Appearance",
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
                .text_size(app.current_font_size())
                .font(app.current_font())
                .into(),
                app
            ),
            create_theme_preview_row(&app.settings.theme, app),
            // 字体设置
            create_font_settings_row(app),
            // 缩放设置
            create_scale_settings_row(app),
        ],
        app,
    )
}

/// 创建主题选择器行，包含分类显示
fn create_theme_selector_row(app: &ArxivManager) -> Element<'_, Message> {
    let theme_colors = app.theme_colors();
    let current_font = app.current_font();
    let base_font_size = app.current_font_size();
    let scale = app.current_scale();
    
    let theme_selector = pick_list(
        Theme::all_variants(),
        Some(app.settings.theme.clone()),
        Message::ThemeChanged,
    )
    .placeholder("Select theme...")
    .style(pick_list_dynamic_style(&app.settings.theme))
    .text_size(base_font_size)
    .font(current_font);

    let current_theme_info = container(
        column![
            text(app.settings.theme.display_name())
                .size(base_font_size)
                .font(current_font)
                .color(theme_colors.text_primary),
            text(format!("Category: {}", app.settings.theme.category()))
                .size(base_font_size * 0.86)
                .font(current_font)
                .color(theme_colors.text_muted),
        ]
        .spacing(4.0 * scale)
    )
    .padding(8.0 * scale)
    .style(theme_info_dynamic_style(&app.settings.theme));

    create_setting_row(
        "Theme:",
        column![
            theme_selector,
            current_theme_info
        ]
        .spacing(8.0 * scale)
        .into(),
        app
    )
}

/// 创建主题预览行
fn create_theme_preview_row<'a>(theme: &Theme, app: &'a ArxivManager) -> Element<'a, Message> {
    let (bg_color, text_color, accent_color) = get_theme_preview(theme);
    let current_font = app.current_font();
    let base_font_size = app.current_font_size();
    let scale = app.current_scale();
    
    let preview_container = container(
        row![
            // 背景色预览
            container(text("BG")
                .color(text_color)
                .size(base_font_size * 0.86)
                .font(current_font))
                .padding(8.0 * scale)
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
            container(text("Text")
                .color(text_color)
                .size(base_font_size * 0.86)
                .font(current_font))
                .padding(8.0 * scale)
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
            container(text("Accent")
                .color(bg_color)
                .size(base_font_size * 0.86)
                .font(current_font))
                .padding(8.0 * scale)
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
                text(if theme.is_dark() { "Dark" } else { "Light" })
                    .size(base_font_size * 0.86)
                    .font(current_font)
                    .color(text_color) // 使用动态文本颜色
            ).padding(4.0 * scale),
        ]
        .spacing(8.0 * scale)
        .align_y(iced::Alignment::Center)
    )
    .padding(12.0 * scale)
    .style(preview_container_dynamic_style(theme));

    create_setting_row(
        "Preview:",
        preview_container.into(),
        app
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

/// 创建字体设置行
fn create_font_settings_row(app: &ArxivManager) -> Element<'_, Message> {
    let theme_colors = app.theme_colors();
    let current_font = app.current_font();
    let base_font_size = app.current_font_size();
    let scale = app.current_scale();
    
    let font_families = vec![
        "Nerd Font".to_string(),
        "System Default".to_string(),
        // 西文字体
        "Arial".to_string(),
        "Helvetica".to_string(),
        "Times New Roman".to_string(),
        "Courier New".to_string(),
        "Georgia".to_string(),
        "Verdana".to_string(),
        "Tahoma".to_string(),
        "Trebuchet MS".to_string(),
        "Segoe UI".to_string(),
        "Calibri".to_string(),
        "Cambria".to_string(),
        "Consolas".to_string(),
    ];
    
    create_setting_row(
        "Font Family:",
        column![
            pick_list(
                font_families,
                Some(app.settings.font_family.clone()),
                Message::FontFamilyChanged,
            )
            .placeholder("Select font family...")
            .style(pick_list_dynamic_style(&app.settings.theme))
            .text_size(base_font_size)
            .font(current_font),
            
            row![
                text("Font Size:")
                    .color(theme_colors.text_secondary)
                    .size(base_font_size)
                    .font(current_font),
                horizontal_space(),
                text_input("14", &app.settings.font_size.to_string())
                    .on_input(Message::FontSizeChanged)
                    .style(text_input_dynamic_style(&app.settings.theme))
                    .size(base_font_size)
                    .font(current_font)
                    .width((80.0 * scale) as u16),
                text("px")
                    .color(theme_colors.text_muted)
                    .size(base_font_size * 0.86)
                    .font(current_font),
            ]
            .spacing(8.0 * scale)
            .align_y(iced::Alignment::Center),
            
            slider(8.0..=72.0, app.settings.font_size, |value| Message::FontSizeChanged(value.to_string()))
                .step(1.0)
                .width(iced::Length::Fill),
        ]
        .spacing(8)
        .into(),
        app
    )
}

/// 创建缩放设置行
fn create_scale_settings_row(app: &ArxivManager) -> Element<'_, Message> {
    let theme_colors = app.theme_colors();
    let current_font = app.current_font();
    let base_font_size = app.current_font_size();
    let scale = app.current_scale();
    
    create_setting_row(
        "UI Scale:",
        column![
            row![
                text("Scale Factor:")
                    .color(theme_colors.text_secondary)
                    .size(base_font_size)
                    .font(current_font),
                horizontal_space(),
                text_input("1.0", &app.settings.ui_scale.to_string())
                    .on_input(Message::UIScaleChanged)
                    .style(text_input_dynamic_style(&app.settings.theme))
                    .size(base_font_size)
                    .font(current_font)
                    .width((80.0 * scale) as u16),
                text("x")
                    .color(theme_colors.text_muted)
                    .size(base_font_size * 0.86)
                    .font(current_font),
            ]
            .spacing(8.0 * scale)
            .align_y(iced::Alignment::Center),
            
            slider(0.5..=3.0, app.settings.ui_scale, |value| Message::UIScaleChanged(value.to_string()))
                .step(0.1)
                .width(iced::Length::Fill),
                
            text(format!("Current: {:.1}x ({}%)", app.settings.ui_scale, (app.settings.ui_scale * 100.0) as u32))
                .color(theme_colors.text_muted)
                .size(base_font_size * 0.86)
                .font(current_font),
        ]
        .spacing(8.0 * scale)
        .into(),
        app
    )
}

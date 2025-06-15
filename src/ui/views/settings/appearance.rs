// 外观设置页面 - 主题、语言、字体和缩放配置

use iced::widget::{pick_list, column, row, text, horizontal_space, text_input, slider};
use iced::Element;

use crate::core::app_state::ArxivManager;
use crate::core::models::{Theme, Language};
use crate::core::messages::Message;
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
    let current_font = app.current_font();
    let base_font_size = app.current_font_size();
    
    let theme_selector = pick_list(
        Theme::all_variants(),
        Some(app.settings.theme.clone()),
        Message::ThemeChanged,
    )
    .placeholder("Select theme...")
    .style(pick_list_dynamic_style(&app.settings.theme))
    .text_size(base_font_size)
    .font(current_font);

    create_setting_row(
        "Theme:",
        theme_selector.into(),
        app
    )
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

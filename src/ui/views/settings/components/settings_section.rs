// 设置区域组件 - 创建带标题和边框的设置区域

use iced::widget::{column, container, text, vertical_space};
use iced::{Element, Background, Border, Color, Shadow};

use crate::core::messages::Message;

use crate::ui::theme::{DARK_BG_SECONDARY, TEXT_PRIMARY};

/// 创建设置区域
/// 
/// # 参数
/// - `title`: 区域标题
/// - `accent_color`: 区域主题色（用于标题和边框）
/// - `items`: 区域内的设置项列表
/// - `app`: 应用状态，用于获取字体和缩放设置
pub fn create_settings_section_with_colors<'a>(
    title: &'a str, 
    accent_color: Color, 
    items: Vec<Element<'a, Message>>,
    app: &'a crate::core::app_state::ArxivManager,
) -> Element<'a, Message> {
    let theme_colors = app.theme_colors();
    let current_font = app.current_font();
    let base_font_size = app.current_font_size();
    let scale = app.current_scale();
    
    container(
        column![
            text(title)
                .color(accent_color)
                .size(base_font_size * 1.43)
                .font(current_font),
            vertical_space().height(10.0 * scale),
            column(items).spacing(15.0 * scale)
        ].spacing(5.0 * scale)
    )
    .padding(15.0 * scale)
    .style(move |_theme| iced::widget::container::Style {
        background: Some(Background::Color(theme_colors.dark_bg_secondary)),
        border: Border {
            color: accent_color,
            width: 1.0,
            radius: 8.0.into(),
        },
        text_color: Some(theme_colors.text_primary),
        shadow: Shadow::default(),
    })
    .into()
}

/// 创建设置区域（向后兼容）
/// 
/// # 参数
/// - `title`: 区域标题
/// - `color`: 区域主题色（用于标题和边框）
/// - `items`: 区域内的设置项列表
pub fn create_settings_section<'a>(
    title: &'a str, 
    color: Color, 
    items: Vec<Element<'a, Message>>
) -> Element<'a, Message> {
    container(
        column![
            text(title).color(color).size(20),
            vertical_space().height(10),
            column(items).spacing(15)
        ].spacing(5)
    )
    .padding(15)
    .style(move |_theme| iced::widget::container::Style {
        background: Some(Background::Color(DARK_BG_SECONDARY)),
        border: Border {
            color,
            width: 1.0,
            radius: 8.0.into(),
        },
        text_color: Some(TEXT_PRIMARY),
        shadow: Shadow::default(),
    })
    .into()
}

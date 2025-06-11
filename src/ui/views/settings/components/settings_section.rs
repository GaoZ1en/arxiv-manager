// 设置区域组件 - 创建带标题和边框的设置区域

use iced::widget::{column, container, text, vertical_space};
use iced::{Element, Background, Border, Color, Shadow};

use crate::core::messages::Message;
use crate::ui::theme::*;

/// 创建设置区域
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
        background: Some(Background::Color(GRUVBOX_SURFACE)),
        border: Border {
            color,
            width: 1.0,
            radius: 8.0.into(),
        },
        text_color: Some(GRUVBOX_TEXT),
        shadow: Shadow::default(),
    })
    .into()
}

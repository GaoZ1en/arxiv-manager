// Emoji文本渲染组件
// 支持混合文本和emoji显示

use iced::widget::{text};
use iced::{Element, Font};
use crate::core::messages::Message;
use crate::core::app_state::ArxivManager;

/// 创建支持emoji的文本组件
pub fn emoji_text<'a>(
    app: &'a ArxivManager,
    content: &str,
    size: f32,
) -> Element<'a, Message> {
    let theme_colors = app.theme_colors();
    emoji_text_colored(app, content, size, theme_colors.text_primary)
}

/// 创建带颜色的emoji文本
pub fn emoji_text_colored<'a>(
    app: &'a ArxivManager,
    content: &str,
    size: f32,
    color: iced::Color,
) -> Element<'a, Message> {
    // 简化实现：对于包含emoji的文本，使用emoji字体
    // 对于普通文本，使用常规字体
    let content_string = content.to_string(); // 转换为owned字符串
    if contains_emoji(&content_string) {
        text(content_string)
            .size(size)
            .font(app.emoji_font())
            .color(color)
            .into()
    } else {
        text(content_string)
            .size(size)
            .font(app.current_font())
            .color(color)
            .into()
    }
}

/// 检查字符串是否包含emoji
fn contains_emoji(text: &str) -> bool {
    text.chars().any(|c| {
        matches!(c as u32,
            // 基本emoji和符号
            0x1F600..=0x1F64F | // 表情符号
            0x1F300..=0x1F5FF | // 各种符号和象形文字
            0x1F680..=0x1F6FF | // 交通和地图符号
            0x1F1E0..=0x1F1FF | // 旗帜
            0x2600..=0x26FF   | // 杂项符号
            0x2700..=0x27BF   | // 装饰符号
            0xFE00..=0xFE0F   | // 变体选择器
            0x1F900..=0x1F9FF | // 补充符号和象形文字
            0x1F018..=0x1F270 | // 各种符号
            // 常见emoji字符
            0x203C | 0x2049 | 0x2122 | 0x2139 | 
            0x2194..=0x2199 | 0x21A9..=0x21AA | 
            0x231A..=0x231B | 0x2328 | 0x23CF |
            0x23E9..=0x23F3 | 0x23F8..=0x23FA |
            0x24C2 | 0x25AA..=0x25AB | 0x25B6 |
            0x25C0 | 0x25FB..=0x25FE | 
            0x2B05..=0x2B07 | 0x2B1B..=0x2B1C |
            0x2B50 | 0x2B55 | 0x3030 | 0x303D |
            0x3297 | 0x3299
        )
    })
}

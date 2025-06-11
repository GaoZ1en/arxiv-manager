// 设置行组件 - 创建标签和控件的设置行

use iced::widget::{container, row, text};
use iced::{Element, Length};

use crate::core::messages::Message;
use crate::core::app_state::ArxivManager;
use crate::ui::theme::*;

/// 创建设置行
/// 
/// # 参数
/// - `label`: 设置项的标签文本
/// - `control`: 设置项的控件（如输入框、下拉菜单等）
/// - `app`: 应用状态，用于获取字体和缩放设置
pub fn create_setting_row<'a>(label: &'a str, control: Element<'a, Message>, app: &'a ArxivManager) -> Element<'a, Message> {
    let theme_colors = app.theme_colors();
    let current_font = app.current_font();
    let base_font_size = app.current_font_size();
    let scale = app.current_scale();
    
    row![
        text(label)
            .color(theme_colors.text_primary)
            .size(base_font_size)
            .font(current_font)
            .width(Length::FillPortion(2)),
        container(control).width(Length::FillPortion(3))
    ]
    .spacing(15.0 * scale)
    .align_y(iced::Alignment::Center)
    .into()
}

/// 创建简单设置行（向后兼容）
pub fn create_simple_setting_row<'a>(label: &'a str, control: Element<'a, Message>) -> Element<'a, Message> {
    row![
        text(label)
            .color(GRUVBOX_TEXT)
            .size(14)
            .width(Length::FillPortion(2)),
        container(control).width(Length::FillPortion(3))
    ]
    .spacing(15)
    .align_y(iced::Alignment::Center)
    .into()
}

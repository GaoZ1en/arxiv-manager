// 设置行组件 - 创建标签和控件的设置行

use iced::widget::{container, row, text};
use iced::{Element, Length};

use crate::core::messages::Message;
use crate::ui::theme::*;

/// 创建设置行
/// 
/// # 参数
/// - `label`: 设置项的标签文本
/// - `control`: 设置项的控件（如输入框、下拉菜单等）
pub fn create_setting_row<'a>(label: &'a str, control: Element<'a, Message>) -> Element<'a, Message> {
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

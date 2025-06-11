// 标签栏组件

use iced::widget::{button, row, text, horizontal_space, container};
use iced::{Element, Length, Color, Background, Border, Shadow, Alignment};

use crate::core::app_state::ArxivManager;
use crate::core::models::TabContent;
use crate::core::messages::Message;
use crate::ui::style::button_secondary_style;
use crate::ui::theme::*;

pub struct TabBar;

impl TabBar {
    pub fn view(app: &ArxivManager) -> Element<'_, Message> {
        let mut tabs_row = row![].spacing(2);

        for (index, tab) in app.tabs.iter().enumerate() {
            let is_active = index == app.active_tab;
            
            let tab_button = button(
                row![
                    text(&tab.title).size(14),
                    if tab.closable {
                        button(text("×").size(12))
                            .on_press(Message::TabClose(index))
                            .style(button_secondary_style)
                            .padding([2, 6])
                            .into()
                    } else {
                        Element::<Message>::from(horizontal_space().width(0))
                    }
                ]
                .spacing(8)
                .align_y(Alignment::Center)
            )
            .on_press(Message::TabClicked(index))
            .padding([8, 16])
            .style(if is_active {
                |_theme: &iced::Theme, _status| iced::widget::button::Style {
                    background: Some(Background::Color(GRUVBOX_BLUE)),
                    text_color: Color::WHITE,
                    border: Border::default(),
                    shadow: Shadow::default(),
                }
            } else {
                |_theme: &iced::Theme, _status| iced::widget::button::Style {
                    background: Some(Background::Color(GRUVBOX_BG_LIGHT)),
                    text_color: GRUVBOX_TEXT,
                    border: Border::default(),
                    shadow: Shadow::default(),
                }
            });

            tabs_row = tabs_row.push(tab_button);
        }

        // 添加新标签页按钮
        let new_tab_button = button(text("+").size(16))
            .on_press(Message::NewTab(TabContent::Search))
            .padding([8, 12])
            .style(button_secondary_style);

        tabs_row = tabs_row.push(new_tab_button);

        container(tabs_row)
            .padding([4, 8])
            .width(Length::Fill)
            .style(|_theme| iced::widget::container::Style {
                background: Some(Background::Color(GRUVBOX_BG_LIGHT)),
                border: Border {
                    color: GRUVBOX_BORDER,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                text_color: Some(GRUVBOX_TEXT),
                shadow: Shadow::default(),
            })
            .into()
    }
}

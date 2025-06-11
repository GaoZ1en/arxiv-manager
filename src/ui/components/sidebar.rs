// 侧边栏组件

use iced::widget::{button, column, text, scrollable, horizontal_rule, container};
use iced::{Element, Length, Background, Border, Shadow};

use crate::core::app_state::ArxivManager;
use crate::core::models::TabContent;
use crate::core::messages::Message;
use crate::ui::style::button_secondary_style;
use crate::ui::theme::*;

pub struct Sidebar;

impl Sidebar {
    pub fn view(app: &ArxivManager) -> Element<'_, Message> {
        let toggle_button = button(text("☰").color(GRUVBOX_TEXT))
            .on_press(Message::SidebarToggled)
            .style(button_secondary_style);

        // 导航按钮
        let navigation_buttons = column![
            button(text("🔍 Search").color(GRUVBOX_TEXT))
                .on_press(Message::TabClicked(0))
                .width(Length::Fill)
                .style(button_secondary_style),
            button(text("📚 Library").color(GRUVBOX_TEXT))
                .on_press(Message::TabClicked(1))
                .width(Length::Fill)
                .style(button_secondary_style),
            button(text("📥 Downloads").color(GRUVBOX_TEXT))
                .on_press(Message::TabClicked(2))
                .width(Length::Fill)
                .style(button_secondary_style),
            button(text("⚙️ Settings").color(GRUVBOX_TEXT))
                .on_press(Message::TabClicked(3))
                .width(Length::Fill)
                .style(button_secondary_style),
        ]
        .spacing(8);

        let saved_papers_list = scrollable(
            column(
                app.saved_papers.iter().enumerate().map(|(index, paper)| {
                    button(text(&paper.title).color(GRUVBOX_TEXT))
                        .on_press(Message::NewTab(TabContent::PaperView(index)))
                        .width(Length::Fill)
                        .style(button_secondary_style)
                        .into()
                }).collect::<Vec<Element<Message>>>()
            ).spacing(4)
        );

        container(
            column![
                toggle_button,
                horizontal_rule(2),
                text("Saved Papers").color(GRUVBOX_TEXT).size(16),
                saved_papers_list,
                horizontal_rule(2),
                text("Navigation").color(GRUVBOX_TEXT).size(16),
                navigation_buttons,
            ]
            .spacing(16)
            .padding(16)
        )
        .width(280)
        .height(Length::Fill)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_SURFACE)),
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

// 论文库视图

use iced::widget::{column, container, scrollable};
use iced::{Element, Length, Background, Border, Shadow};

use crate::core::app_state::ArxivManager;
use crate::core::messages::Message;
use crate::ui::components::PaperCard;
use crate::ui::theme::*;

pub struct LibraryView;

impl LibraryView {
    pub fn view(app: &ArxivManager) -> Element<Message> {
        let content = if app.saved_papers.is_empty() {
            column![iced::widget::text("No saved papers").color(GRUVBOX_TEXT_MUTED)]
        } else {
            column(
                app.saved_papers.iter().map(|paper| {
                    PaperCard::view(app, paper, true)
                }).collect::<Vec<Element<Message>>>()
            ).spacing(10)
        };

        container(
            scrollable(content).height(Length::Fill)
        )
        .padding(20)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_BG)),
            border: Border::default(),
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }
}

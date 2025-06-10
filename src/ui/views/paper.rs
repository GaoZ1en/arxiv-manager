// 论文详情视图

use iced::widget::{column, container, scrollable, text, horizontal_rule};
use iced::{Element, Background, Border, Shadow};

use crate::core::models::ArxivPaper;
use crate::core::messages::Message;
use crate::ui::theme::*;

pub struct PaperView;

impl PaperView {
    pub fn view<'a>(paper: &'a ArxivPaper) -> Element<'a, Message> {
        let title = text(&paper.title)
            .color(GRUVBOX_TEXT)
            .size(24);

        let authors = text(paper.authors.join(", "))
            .color(GRUVBOX_TEXT_MUTED)
            .size(14);

        let published = text(format!("Published: {}", paper.published))
            .color(GRUVBOX_TEXT_MUTED)
            .size(12);

        let categories = text(format!("Categories: {}", paper.categories.join(", ")))
            .color(GRUVBOX_TEXT_MUTED)
            .size(12);

        let abstract_text = text(&paper.abstract_text)
            .color(GRUVBOX_TEXT)
            .size(14);

        container(
            scrollable(
                column![
                    title,
                    authors,
                    published,
                    categories,
                    horizontal_rule(1),
                    text("Abstract").color(GRUVBOX_TEXT).size(18),
                    abstract_text,
                ]
                .spacing(10)
            )
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

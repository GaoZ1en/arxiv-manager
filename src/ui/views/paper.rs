// 论文详情视图 - 现代IRC风格界面

use iced::widget::{column, container, scrollable, text, horizontal_rule};
use iced::Element;

use crate::core::models::{ArxivPaper, Theme as ThemeVariant};
use crate::core::messages::Message;
use crate::ui::theme::get_theme_colors;
use crate::ui::style::chat_container_dynamic_style;

pub struct PaperView;

impl PaperView {
    pub fn view<'a>(paper: &'a ArxivPaper, theme: &ThemeVariant) -> Element<'a, Message> {
        let theme_colors = get_theme_colors(theme);
        
        let title = text(&paper.title)
            .color(theme_colors.text_primary)
            .size(28)
            .width(iced::Length::Fill);

        let authors = text(paper.authors.join(", "))
            .color(theme_colors.text_secondary)
            .size(16)
            .width(iced::Length::Fill);

        let metadata_row = column![
            text(format!("📅 Published: {}", paper.published))
                .color(theme_colors.text_muted)
                .size(14),
            text(format!("🏷️ Categories: {}", paper.categories.join(", ")))
                .color(theme_colors.text_muted)
                .size(14),
        ]
        .spacing(8);

        let abstract_title = text("📄 Abstract")
            .color(theme_colors.info_color)
            .size(20)
            .width(iced::Length::Fill);

        let abstract_text = text(&paper.abstract_text)
            .color(theme_colors.text_primary)
            .size(15)
            .line_height(1.5);

        container(
            scrollable(
                column![
                    title,
                    authors,
                    iced::widget::Space::with_height(16),
                    metadata_row,
                    iced::widget::Space::with_height(20),
                    horizontal_rule(2),
                    iced::widget::Space::with_height(20),
                    abstract_title,
                    iced::widget::Space::with_height(12),
                    abstract_text,
                ]
                .spacing(12)
                .padding([24, 24])
            )
        )
        .style(chat_container_dynamic_style(theme))
        .padding(16)
        .into()
    }
}

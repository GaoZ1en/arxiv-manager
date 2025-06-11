// 论文详情视图 - 现代IRC风格界面

use iced::widget::{column, container, scrollable, text, horizontal_rule};
use iced::Element;

use crate::core::models::{ArxivPaper};
use crate::core::app_state::ArxivManager;
use crate::core::messages::Message;
use crate::ui::style::chat_container_dynamic_style;

pub struct PaperView;

impl PaperView {
    pub fn view<'a>(paper: &'a ArxivPaper, app: &'a ArxivManager) -> Element<'a, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let scale = app.current_scale();
        
        let title = text(&paper.title)
            .color(theme_colors.text_primary)
            .size(base_font_size * 2.0)
            .font(current_font)
            .width(iced::Length::Fill);

        let authors = text(paper.authors.join(", "))
            .color(theme_colors.text_secondary)
            .size(base_font_size * 1.14)
            .font(current_font)
            .width(iced::Length::Fill);

        let metadata_row = column![
            text(format!("📅 Published: {}", paper.published))
                .color(theme_colors.text_muted)
                .size(base_font_size)
                .font(current_font),
            text(format!("🏷️ Categories: {}", paper.categories.join(", ")))
                .color(theme_colors.text_muted)
                .size(base_font_size)
                .font(current_font),
        ]
        .spacing(8.0 * scale);

        let abstract_title = text("📄 Abstract")
            .color(theme_colors.info_color)
            .size(base_font_size * 1.43)
            .font(current_font)
            .width(iced::Length::Fill);

        let abstract_text = text(&paper.abstract_text)
            .color(theme_colors.text_primary)
            .size(base_font_size * 1.07)
            .font(current_font)
            .line_height(1.5);

        container(
            scrollable(
                column![
                    title,
                    authors,
                    iced::widget::Space::with_height(16.0 * scale),
                    metadata_row,
                    iced::widget::Space::with_height(20.0 * scale),
                    horizontal_rule(2),
                    iced::widget::Space::with_height(20.0 * scale),
                    abstract_title,
                    iced::widget::Space::with_height(12.0 * scale),
                    abstract_text,
                ]
                .spacing(12.0 * scale)
                .padding([24.0 * scale, 24.0 * scale])
            )
        )
        .style(chat_container_dynamic_style(&app.settings.theme))
        .padding(16.0 * scale)
        .into()
    }
}

// 现代化论文库视图 - IRC客户端风格

use iced::widget::{column, container, scrollable, text};
use iced::{Element, Length};

use crate::core::app_state::ArxivManager;
use crate::core::messages::Message;
use crate::ui::components::PaperCard;
use crate::ui::style::chat_container_dynamic_style;

pub struct LibraryView;

impl LibraryView {
    pub fn view(app: &ArxivManager) -> Element<'_, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let scale = app.current_scale();
        
        let content = if app.saved_papers.is_empty() {
            column![
                text("📚 Your Library")
                    .color(theme_colors.text_primary)
                    .size(base_font_size * 1.43)
                    .font(current_font),
                text("No saved papers yet")
                    .color(theme_colors.text_muted)
                    .size(base_font_size * 1.14)
                    .font(current_font),
                text("Papers you save from search will appear here")
                    .color(theme_colors.text_secondary)
                    .size(base_font_size)
                    .font(current_font)
            ]
            .spacing(12.0 * scale)
            .padding(32.0 * scale)
            .align_x(iced::Alignment::Center)
        } else {
            let papers_count = app.saved_papers.len();
            column![
                container(
                    text(format!("📚 Library ({} papers)", papers_count))
                        .color(theme_colors.text_primary)
                        .size(base_font_size * 1.29)
                        .font(current_font)
                )
                .padding(iced::Padding {
                    top: 16.0 * scale,
                    right: 16.0 * scale,
                    bottom: 8.0 * scale,
                    left: 16.0 * scale,
                }),
                
                container(
                    scrollable(
                        column(
                            app.saved_papers.iter().map(|paper| {
                                PaperCard::view(app, paper, true)
                            }).collect::<Vec<Element<Message>>>()
                        ).spacing(12)
                    )
                    .height(Length::Fill)
                )
                .padding(16)
            ]
        };

        container(content)
            .style(chat_container_dynamic_style(&app.settings.theme))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

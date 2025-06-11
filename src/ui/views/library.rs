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
        
        let content = if app.saved_papers.is_empty() {
            column![
                text("📚 Your Library")
                    .color(theme_colors.text_primary)
                    .size(20),
                text("No saved papers yet")
                    .color(theme_colors.text_muted)
                    .size(16),
                text("Papers you save from search will appear here")
                    .color(theme_colors.text_secondary)
                    .size(14)
            ]
            .spacing(12)
            .padding(32)
            .align_x(iced::Alignment::Center)
        } else {
            let papers_count = app.saved_papers.len();
            column![
                container(
                    text(format!("📚 Library ({} papers)", papers_count))
                        .color(theme_colors.text_primary)
                        .size(18)
                )
                .padding(iced::Padding {
                    top: 16.0,
                    right: 16.0,
                    bottom: 8.0,
                    left: 16.0,
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

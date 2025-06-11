// 现代化下载视图 - IRC客户端风格

use iced::widget::{column, container, scrollable, text};
use iced::{Element, Length};

use crate::core::app_state::ArxivManager;
use crate::core::messages::Message;
use crate::ui::components::PaperCard;
use crate::ui::style::chat_container_dynamic_style;

pub struct DownloadsView;

impl DownloadsView {
    pub fn view(app: &ArxivManager) -> Element<'_, Message> {
        let theme_colors = app.theme_colors();
        
        let content = if app.downloads.is_empty() {
            column![
                text("📥 Downloads")
                    .color(theme_colors.text_primary)
                    .size(20),
                text("No active downloads")
                    .color(theme_colors.text_muted)
                    .size(16),
                text("PDF downloads will appear here")
                    .color(theme_colors.text_secondary)
                    .size(14)
            ]
            .spacing(12)
            .padding(32)
            .align_x(iced::Alignment::Center)
        } else {
            let downloads_count = app.downloads.len();
            column![
                container(
                    text(format!("📥 Downloads ({} active)", downloads_count))
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
                            app.downloads.iter().map(|download| {
                                PaperCard::download_card(download, theme_colors)
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

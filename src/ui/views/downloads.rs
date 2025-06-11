// 下载视图

use iced::widget::{column, container, scrollable};
use iced::{Element, Length, Background, Border, Shadow};

use crate::core::app_state::ArxivManager;
use crate::core::messages::Message;
use crate::ui::components::PaperCard;
use crate::ui::theme::*;

pub struct DownloadsView;

impl DownloadsView {
    pub fn view(app: &ArxivManager) -> Element<'_, Message> {
        let content = if app.downloads.is_empty() {
            column![iced::widget::text("No downloads").color(GRUVBOX_TEXT_MUTED)]
        } else {
            column(
                app.downloads.iter().map(|download| {
                    PaperCard::download_card(download)
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

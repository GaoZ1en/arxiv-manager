// 论文卡片组件

use iced::widget::{button, column, row, text, vertical_space, progress_bar, container};
use iced::{Element, Color, Background, Border, Shadow};

use crate::core::app_state::ArxivManager;
use crate::core::models::{ArxivPaper, DownloadItem, DownloadStatus, TabContent};
use crate::core::messages::Message;
use crate::ui::style::{button_primary_style, button_secondary_style, button_danger_style};
use crate::ui::theme::*;

pub struct PaperCard;

impl PaperCard {
    pub fn view<'a>(app: &'a ArxivManager, paper: &'a ArxivPaper, is_saved: bool) -> Element<'a, Message> {
        let title = text(&paper.title)
            .color(GRUVBOX_TEXT)
            .size(16);

        let authors = text(paper.authors.join(", "))
            .color(GRUVBOX_TEXT_MUTED)
            .size(12);

        let buttons = if is_saved {
            row![
                button(text("Remove").color(Color::WHITE))
                    .on_press(Message::RemovePaper(paper.id.clone()))
                    .style(button_danger_style),
                button(text("Download").color(Color::BLACK))
                    .on_press(Message::DownloadPaper(paper.clone()))
                    .style(button_primary_style),
                button(text("View").color(GRUVBOX_TEXT))
                    .on_press(if let Some(index) = app.saved_papers.iter().position(|p| p.id == paper.id) {
                        Message::NewTab(TabContent::PaperView(index))
                    } else {
                        Message::NoOp
                    })
                    .style(button_secondary_style),
            ]
        } else {
            row![
                button(text("Save").color(Color::BLACK))
                    .on_press(Message::SavePaper(paper.clone()))
                    .style(button_primary_style),
                button(text("Download").color(GRUVBOX_TEXT))
                    .on_press(Message::DownloadPaper(paper.clone()))
                    .style(button_secondary_style),
            ]
        }
        .spacing(8);

        container(
            column![
                title,
                authors,
                vertical_space().height(8),
                buttons,
            ]
            .spacing(4)
        )
        .padding(12)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_SURFACE)),
            border: Border {
                color: GRUVBOX_BORDER,
                width: 1.0,
                radius: 8.0.into(),
            },
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    pub fn download_card<'a>(download: &'a DownloadItem) -> Element<'a, Message> {
        let title = text(&download.title)
            .color(GRUVBOX_TEXT)
            .size(14);

        let status_text = match &download.status {
            DownloadStatus::Pending => "Pending",
            DownloadStatus::Downloading => "Downloading",
            DownloadStatus::Completed => "Completed",
            DownloadStatus::Failed(_) => "Failed",
        };

        let status = text(status_text)
            .color(match download.status {
                DownloadStatus::Failed(_) => GRUVBOX_RED,
                DownloadStatus::Completed => GRUVBOX_GREEN,
                _ => GRUVBOX_TEXT_MUTED,
            })
            .size(12);

        let progress = if matches!(download.status, DownloadStatus::Downloading) {
            Some(progress_bar(0.0..=100.0, download.progress))
        } else {
            None
        };

        let mut content = column![title, status].spacing(4);
        
        if let Some(progress_bar) = progress {
            content = content.push(progress_bar);
        }

        container(content)
            .padding(12)
            .style(|_theme| iced::widget::container::Style {
                background: Some(Background::Color(GRUVBOX_SURFACE)),
                border: Border {
                    color: GRUVBOX_BORDER,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                text_color: Some(GRUVBOX_TEXT),
                shadow: Shadow::default(),
            })
            .into()
    }
}

// 论文卡片组件

use iced::widget::{button, column, row, text, vertical_space, progress_bar, container};
use iced::{Element, Color, Background, Border, Shadow};

use crate::core::app_state::ArxivManager;
use crate::core::models::{ArxivPaper, DownloadItem, DownloadStatus, TabContent};
use crate::core::messages::Message;
use crate::ui::style::{button_primary_style_dynamic, button_secondary_style_dynamic, button_danger_style_dynamic};

pub struct PaperCard;

impl PaperCard {
    pub fn view<'a>(app: &'a ArxivManager, paper: &'a ArxivPaper, is_saved: bool) -> Element<'a, Message> {
        let theme_colors = app.theme_colors();
        
        let title = text(&paper.title)
            .color(theme_colors.text_primary)
            .size(16);

        let authors = text(paper.authors.join(", "))
            .color(theme_colors.text_muted)
            .size(12);

        let buttons = if is_saved {
            row![
                button(text("Remove").color(Color::WHITE))
                    .on_press(Message::RemovePaper(paper.id.clone()))
                    .style(button_danger_style_dynamic(&app.settings.theme)),
                button(text("Download").color(Color::BLACK))
                    .on_press(Message::DownloadPaper(paper.clone()))
                    .style(button_primary_style_dynamic(&app.settings.theme)),
                button(text("View").color(theme_colors.text_primary))
                    .on_press(if let Some(index) = app.saved_papers.iter().position(|p| p.id == paper.id) {
                        Message::NewTab(TabContent::PaperView(index))
                    } else {
                        Message::NoOp
                    })
                    .style(button_secondary_style_dynamic(&app.settings.theme)),
            ]
        } else {
            row![
                button(text("Save").color(Color::BLACK))
                    .on_press(Message::SavePaper(paper.clone()))
                    .style(button_primary_style_dynamic(&app.settings.theme)),
                button(text("Download").color(theme_colors.text_primary))
                    .on_press(Message::DownloadPaper(paper.clone()))
                    .style(button_secondary_style_dynamic(&app.settings.theme)),
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
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(theme_colors.dark_bg_secondary)),
            border: Border {
                color: theme_colors.border_color,
                width: 1.0,
                radius: 8.0.into(),
            },
            text_color: Some(theme_colors.text_primary),
            shadow: Shadow::default(),
        })
        .into()
    }

    pub fn download_card<'a>(download: &'a DownloadItem, theme_colors: crate::ui::theme::ThemeColors) -> Element<'a, Message> {
        let title = text(&download.title)
            .color(theme_colors.text_primary)
            .size(14);

        let status_text = match &download.status {
            DownloadStatus::Pending => "Pending",
            DownloadStatus::Downloading => "Downloading",
            DownloadStatus::Completed => "Completed",
            DownloadStatus::Failed(_) => "Failed",
        };

        let status = text(status_text)
            .color(match download.status {
                DownloadStatus::Failed(_) => theme_colors.error_color,
                DownloadStatus::Completed => theme_colors.success_color,
                _ => theme_colors.text_muted,
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
            .style(move |_theme| iced::widget::container::Style {
                background: Some(Background::Color(theme_colors.dark_bg_secondary)),
                border: Border {
                    color: theme_colors.border_color,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                text_color: Some(theme_colors.text_primary),
                shadow: Shadow::default(),
            })
            .into()
    }
}

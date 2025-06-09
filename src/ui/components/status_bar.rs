use iced::{
    widget::{container, row, text, Row},
    Alignment, Element, Length, border,
};

use crate::{
    app::{ArxivManager, Message},
    database::DownloadStatus,
    ui::style,
};

pub struct StatusBar;

impl StatusBar {
    pub fn view(app: &ArxivManager) -> Element<Message> {
        let mut status_row = Row::new()
            .spacing(20)
            .align_items(Alignment::Center)
            .width(Length::Fill);

        // Connection status
        let connection_status = Self::create_connection_status(app);
        status_row = status_row.push(connection_status);

        // Downloads status
        let downloads_status = Self::create_downloads_status(app);
        status_row = status_row.push(downloads_status);

        // Library stats
        let library_stats = Self::create_library_stats(app);
        status_row = status_row.push(library_stats);

        // Spacer
        status_row = status_row.push(
            container(text(""))
                .width(Length::Fill)
        );

        // Memory usage (optional)
        let memory_status = Self::create_memory_status();
        status_row = status_row.push(memory_status);

        // Version info
        let version_info = Self::create_version_info();
        status_row = status_row.push(version_info);

        container(status_row)
            .padding([5, 15])
            .width(Length::Fill)
            .style(GruvboxStyle::status_bar())
            .into()
    }

    fn create_connection_status(app: &ArxivManager) -> Element<Message> {
        let (status_text, status_color) = if app.is_online {
            ("● 在线", GruvboxColors::Green)
        } else {
            ("● 离线", GruvboxColors::Red)
        };

        text(status_text)
            .size(12)
            .style(status_color)
            .into()
    }

    fn create_downloads_status(app: &ArxivManager) -> Element<Message> {
        let active_downloads = app.download_tasks.iter()
            .filter(|task| matches!(
                task.status, 
                DownloadStatus::Downloading
            ))
            .count();

        let status_text = if active_downloads > 0 {
            format!("下载中: {}", active_downloads)
        } else {
            "无活动下载".to_string()
        };

        let status_color = if active_downloads > 0 {
            GruvboxColors::Blue
        } else {
            GruvboxColors::Light4
        };

        text(status_text)
            .size(12)
            .style(status_color)
            .into()
    }

    fn create_library_stats(app: &ArxivManager) -> Element<Message> {
        let total_papers = app.total_papers.unwrap_or(0);
        let downloaded_papers = app.downloaded_papers.unwrap_or(0);

        let stats_text = format!(
            "论文库: {} 篇 (已下载: {} 篇)",
            total_papers,
            downloaded_papers
        );

        text(stats_text)
            .size(12)
            .style(GruvboxColors::Light3)
            .into()
    }

    fn create_memory_status() -> Element<'static, Message> {
        // 简单的内存使用显示（在实际应用中可以使用系统API获取真实内存使用）
        let memory_usage = Self::get_memory_usage();
        
        text(format!("内存: {:.1} MB", memory_usage))
            .size(12)
            .style(GruvboxColors::Light4)
            .into()
    }

    fn create_version_info() -> Element<'static, Message> {
        let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string());
        
        text(format!("v{}", version))
            .size(12)
            .style(style::text::default())
            .into()
    }

    fn get_memory_usage() -> f64 {
        // 这里返回一个模拟的内存使用量
        // 在实际应用中，可以使用 sysinfo crate 获取真实的内存使用情况
        #[cfg(unix)]
        {
            use std::fs;
            if let Ok(status) = fs::read_to_string("/proc/self/status") {
                for line in status.lines() {
                    if line.starts_with("VmRSS:") {
                        if let Some(kb_str) = line.split_whitespace().nth(1) {
                            if let Ok(kb) = kb_str.parse::<f64>() {
                                return kb / 1024.0; // Convert KB to MB
                            }
                        }
                    }
                }
            }
        }
        
        // Fallback: return a reasonable estimate
        32.5
    }
}

impl GruvboxStyle {
    pub fn status_bar() -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(StatusBarStyle))
    }
}

struct StatusBarStyle;

impl iced::widget::container::StyleSheet for StatusBarStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0x32 as f32 / 255.0,
                0x30 as f32 / 255.0,
                0x2f as f32 / 255.0,
            ))),
            border_color: iced::Color::from_rgb(
                0x50 as f32 / 255.0,
                0x49 as f32 / 255.0,
                0x45 as f32 / 255.0,
            ),
            border_width: 1.0,
            border_radius: border::Radius::from(0.0),
            text_color: None,
        }
    }
}

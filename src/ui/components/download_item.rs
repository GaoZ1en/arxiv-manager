use iced::{
    widget::{button, column, container, progress_bar, row, text, Column, Container, Row},
    Alignment, Element, Length,
};

use crate::{
    app::Message,
    downloader::DownloadTask,
    database::DownloadStatus,
    ui::style,
};

pub struct DownloadItem;

impl DownloadItem {
    pub fn view(task: &DownloadTask, index: usize) -> Element<Message> {
        let mut main_row = Row::new()
            .spacing(15)
            .align_items(Alignment::Center)
            .width(Length::Fill);

        // Paper info section
        let paper_info = Self::create_paper_info(task);
        main_row = main_row.push(paper_info);

        // Progress section
        let progress_section = Self::create_progress_section(task);
        main_row = main_row.push(progress_section);

        // Controls section
        let controls = Self::create_controls(task, index);
        main_row = main_row.push(controls);

        Container::new(main_row)
            .padding(15)
            .width(Length::Fill)
            .style(GruvboxStyle::card())
            .into()
    }

    fn create_paper_info(task: &DownloadTask) -> Column<Message> {
        let title = text(&task.paper_title)
            .size(14)
            .style(GruvboxColors::Light0);

        let url_text = text(&task.url)
            .size(12)
            .style(GruvboxColors::Light4);

        let file_info = text(format!(
            "保存位置: {}",
            task.file_path.display()
        ))
        .size(11)
        .style(GruvboxColors::Light4);

        column![title, url_text, file_info]
            .spacing(5)
            .width(Length::FillPortion(3))
    }

    fn create_progress_section(task: &DownloadTask) -> Column<Message> {
        let status_text = Self::get_status_text(&task.status);
        let status_color = Self::get_status_color(&task.status);

        let status_label = text(status_text)
            .size(12)
            .style(status_color);

        let mut progress_column = column![status_label].spacing(5);

        // Progress bar and details
        match task.status {
            DownloadStatus::Downloading => {
                let progress_bar = progress_bar(0.0..=100.0, task.progress)
                    .width(Length::Fill)
                    .height(Length::Fixed(6.0));

                let progress_text = text(format!("{:.1}%", task.progress))
                    .size(11)
                    .style(GruvboxColors::Light2);

                let speed_text = text(Self::format_speed(task.speed))
                    .size(11)
                    .style(GruvboxColors::Blue);

                let details_row = row![progress_text, speed_text]
                    .spacing(10)
                    .align_items(Alignment::Center);

                progress_column = progress_column
                    .push(progress_bar)
                    .push(details_row);
            }
            DownloadStatus::Completed => {
                let progress_bar = progress_bar(0.0..=100.0, 100.0)
                    .width(Length::Fill)
                    .height(Length::Fixed(6.0));

                progress_column = progress_column.push(progress_bar);
            }
            DownloadStatus::Failed => {
                if let Some(error) = &task.error_message {
                    let error_text = text(format!("错误: {}", error))
                        .size(11)
                        .style(GruvboxColors::Red);

                    progress_column = progress_column.push(error_text);
                }
            }
            _ => {}
        }

        progress_column.width(Length::FillPortion(2))
    }

    fn create_controls(task: &DownloadTask, index: usize) -> Row<Message> {
        let mut controls_row = Row::new().spacing(10);

        match task.status {
            DownloadStatus::Pending => {
                controls_row = controls_row.push(
                    button("开始")
                        .on_press(Message::StartDownload(index))
                        .style(GruvboxStyle::button())
                );
            }
            DownloadStatus::Downloading => {
                controls_row = controls_row.push(
                    button("暂停")
                        .on_press(Message::PauseDownload(index))
                        .style(GruvboxStyle::button())
                );
            }
            DownloadStatus::Paused => {
                controls_row = controls_row.push(
                    button("恢复")
                        .on_press(Message::ResumeDownload(index))
                        .style(GruvboxStyle::button())
                );
            }
            DownloadStatus::Completed => {
                controls_row = controls_row.push(
                    button("打开")
                        .on_press(Message::OpenDownloadedFile(index))
                        .style(GruvboxStyle::button())
                );
            }
            DownloadStatus::Failed => {
                controls_row = controls_row.push(
                    button("重试")
                        .on_press(Message::RetryDownload(index))
                        .style(GruvboxStyle::button())
                );
            }
        }

        // Remove button (always available)
        controls_row = controls_row.push(
            button("移除")
                .on_press(Message::RemoveDownload(index))
                .style(GruvboxStyle::danger_button())
        );

        controls_row.width(Length::Shrink)
    }

    fn get_status_text(status: &DownloadStatus) -> &'static str {
        match status {
            DownloadStatus::Pending => "等待中",
            DownloadStatus::Downloading => "下载中",
            DownloadStatus::Paused => "已暂停",
            DownloadStatus::Completed => "已完成",
            DownloadStatus::Failed => "失败",
        }
    }

    fn get_status_color(status: &DownloadStatus) -> GruvboxColors {
        match status {
            DownloadStatus::Pending => GruvboxColors::Yellow,
            DownloadStatus::Downloading => GruvboxColors::Blue,
            DownloadStatus::Paused => GruvboxColors::Orange,
            DownloadStatus::Completed => GruvboxColors::Green,
            DownloadStatus::Failed => GruvboxColors::Red,
        }
    }

    fn format_speed(bytes_per_sec: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        if bytes_per_sec >= GB {
            format!("{:.1} GB/s", bytes_per_sec as f64 / GB as f64)
        } else if bytes_per_sec >= MB {
            format!("{:.1} MB/s", bytes_per_sec as f64 / MB as f64)
        } else if bytes_per_sec >= KB {
            format!("{:.1} KB/s", bytes_per_sec as f64 / KB as f64)
        } else {
            format!("{} B/s", bytes_per_sec)
        }
    }
}

use iced::{
    widget::{button, column, container, progress_bar, row, scrollable, text, Column, Container},
    Alignment, Element, Length,
};

use crate::{
    app::{ArxivManager, Message},
    downloader::DownloadTask,
    database::DownloadStatus,
    ui::style,
};

use super::download_item::DownloadItem;

pub struct DownloadsView;

impl DownloadsView {
    pub fn view(app: &ArxivManager) -> Element<Message> {
        let mut content = Column::new()
            .padding(20)
            .spacing(20)
            .width(Length::Fill);

        // Header with controls
        let header = row![
            text("下载管理").size(24).style(GruvboxColors::Light0),
            button("清除已完成")
                .on_press(Message::ClearCompletedDownloads)
                .style(GruvboxStyle::button()),
            button("暂停全部")
                .on_press(Message::PauseAllDownloads)
                .style(GruvboxStyle::button()),
            button("恢复全部")
                .on_press(Message::ResumeAllDownloads)
                .style(GruvboxStyle::button()),
        ]
        .spacing(20)
        .align_items(Alignment::Center);

        content = content.push(header);

        // Downloads statistics
        let stats = Self::create_stats_view(app);
        content = content.push(stats);

        // Downloads list
        let downloads_list = Self::create_downloads_list(app);
        content = content.push(downloads_list);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(GruvboxStyle::container())
            .into()
    }

    fn create_stats_view(app: &ArxivManager) -> Container<Message> {
        let active_downloads = app.download_tasks.iter()
            .filter(|task| task.is_downloading())
            .count();

        let completed_downloads = app.download_tasks.iter()
            .filter(|task| task.is_completed())
            .count();

        let failed_downloads = app.download_tasks.iter()
            .filter(|task| task.is_failed())
            .count();

        let total_downloads = app.download_tasks.len();

        let stats_row = row![
            Self::stat_item("总计", &total_downloads.to_string()),
            Self::stat_item("进行中", &active_downloads.to_string()),
            Self::stat_item("已完成", &completed_downloads.to_string()),
            Self::stat_item("失败", &failed_downloads.to_string()),
        ]
        .spacing(40)
        .align_items(Alignment::Center);

        Container::new(stats_row)
            .padding(15)
            .style(GruvboxStyle::card())
    }

    fn stat_item<'a>(label: &'a str, value: &'a str) -> Column<'a, Message> {
        column![
            text(value)
                .size(20)
                .style(GruvboxColors::Blue),
            text(label)
                .size(12)
                .style(GruvboxColors::Light4),
        ]
        .align_items(Alignment::Center)
        .spacing(5)
    }

    fn create_downloads_list(app: &ArxivManager) -> Container<Message> {
        let mut downloads_column = Column::new().spacing(10);

        if app.download_tasks.is_empty() {
            let empty_message = container(
                text("暂无下载任务")
                    .size(16)
                    .style(GruvboxColors::Light4)
            )
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fixed(200.0));

            downloads_column = downloads_column.push(empty_message);
        } else {
            for (index, task) in app.download_tasks.iter().enumerate() {
                let download_item = DownloadItem::view(task, index);
                downloads_column = downloads_column.push(download_item);
            }
        }

        let scrollable = scrollable(downloads_column)
            .width(Length::Fill)
            .height(Length::Fill);

        Container::new(scrollable)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(GruvboxStyle::container())
    }
}

// Extension trait for DownloadTask status checking
trait DownloadTaskExt {
    fn is_downloading(&self) -> bool;
    fn is_completed(&self) -> bool;
    fn is_failed(&self) -> bool;
}

impl DownloadTaskExt for DownloadTask {
    fn is_downloading(&self) -> bool {
        // Since the status is wrapped in Arc<RwLock<>>, we need to handle this differently in UI context
        // For now, we'll provide a placeholder implementation
        false // TODO: Implement async status checking or use a different approach
    }

    fn is_completed(&self) -> bool {
        false // TODO: Implement async status checking or use a different approach
    }

    fn is_failed(&self) -> bool {
        false // TODO: Implement async status checking or use a different approach
    }
}

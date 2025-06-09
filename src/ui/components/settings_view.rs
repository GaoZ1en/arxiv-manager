use iced::{
    widget::{
        button, checkbox, column, container, pick_list, row, scrollable, text, text_input,
        Column, Container,
    },
    Alignment, Element, Length,
};

use crate::{
    app::{ArxivManager, Message},
    config::{AppConfig, DownloadSettings, UISettings},
    ui::style,
};

pub struct SettingsView;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
    Auto,
}

impl Theme {
    const ALL: [Theme; 3] = [Theme::Light, Theme::Dark, Theme::Auto];
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Theme::Light => "浅色",
                Theme::Dark => "深色",
                Theme::Auto => "自动",
            }
        )
    }
}

impl SettingsView {
    pub fn view(app: &ArxivManager) -> Element<Message> {
        let content = column![
            Self::create_header(),
            Self::create_general_settings(app),
            Self::create_download_settings(app),
            Self::create_ui_settings(app),
            Self::create_database_settings(app),
            Self::create_actions_section(),
        ]
        .spacing(20)
        .padding(20);

        let scrollable_content = scrollable(content)
            .width(Length::Fill)
            .height(Length::Fill);

        Container::new(scrollable_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(GruvboxStyle::container())
            .into()
    }

    fn create_header() -> Element<'static, Message> {
        text("设置")
            .size(24)
            .style(GruvboxColors::Light0)
            .into()
    }

    fn create_general_settings(app: &ArxivManager) -> Element<Message> {
        let section_title = text("常规设置")
            .size(18)
            .style(GruvboxColors::Light1);

        let language_row = row![
            text("语言:")
                .size(14)
                .style(GruvboxColors::Light2)
                .width(Length::Fixed(120.0)),
            pick_list(
                vec!["中文", "English"],
                Some("中文"),
                |_| Message::UpdateLanguage("zh-CN".to_string())
            )
            .style(GruvboxStyle::pick_list())
        ]
        .align_items(Alignment::Center)
        .spacing(10);

        let auto_check_updates = row![
            text("自动检查更新:")
                .size(14)
                .style(GruvboxColors::Light2)
                .width(Length::Fixed(120.0)),
            checkbox("启用", true, |_| Message::ToggleAutoUpdate)
                .style(GruvboxStyle::checkbox())
        ]
        .align_items(Alignment::Center)
        .spacing(10);

        let content = column![section_title, language_row, auto_check_updates]
            .spacing(15);

        Container::new(content)
            .padding(15)
            .style(GruvboxStyle::card())
            .into()
    }

    fn create_download_settings(app: &ArxivManager) -> Element<Message> {
        let section_title = text("下载设置")
            .size(18)
            .style(GruvboxColors::Light1);

        let download_dir_row = row![
            text("下载目录:")
                .size(14)
                .style(GruvboxColors::Light2)
                .width(Length::Fixed(120.0)),
            text_input(
                "选择下载目录",
                &app.config.download.download_directory.to_string_lossy()
            )
            .on_input(Message::UpdateDownloadDirectory)
            .style(GruvboxStyle::text_input()),
            button("浏览")
                .on_press(Message::BrowseDownloadDirectory)
                .style(GruvboxStyle::button())
        ]
        .align_items(Alignment::Center)
        .spacing(10);

        let concurrent_downloads_row = row![
            text("并发下载数:")
                .size(14)
                .style(GruvboxColors::Light2)
                .width(Length::Fixed(120.0)),
            text_input(
                "4",
                &app.config.download.max_concurrent_downloads.to_string()
            )
            .on_input(Message::UpdateConcurrentDownloads)
            .style(GruvboxStyle::text_input())
            .width(Length::Fixed(100.0))
        ]
        .align_items(Alignment::Center)
        .spacing(10);

        let retry_attempts_row = row![
            text("重试次数:")
                .size(14)
                .style(GruvboxColors::Light2)
                .width(Length::Fixed(120.0)),
            text_input("3", &app.config.download.retry_attempts.to_string())
                .on_input(Message::UpdateRetryAttempts)
                .style(GruvboxStyle::text_input())
                .width(Length::Fixed(100.0))
        ]
        .align_items(Alignment::Center)
        .spacing(10);

        let auto_organize = row![
            text("自动整理文件:")
                .size(14)
                .style(GruvboxColors::Light2)
                .width(Length::Fixed(120.0)),
            checkbox(
                "按分类自动创建子文件夹",
                app.config.download.auto_organize,
                |_| Message::ToggleAutoOrganize
            )
            .style(GruvboxStyle::checkbox())
        ]
        .align_items(Alignment::Center)
        .spacing(10);

        let content = column![
            section_title,
            download_dir_row,
            concurrent_downloads_row,
            retry_attempts_row,
            auto_organize
        ]
        .spacing(15);

        Container::new(content)
            .padding(15)
            .style(GruvboxStyle::card())
            .into()
    }

    fn create_ui_settings(app: &ArxivManager) -> Element<Message> {
        let section_title = text("界面设置")
            .size(18)
            .style(GruvboxColors::Light1);

        let theme_row = row![
            text("主题:")
                .size(14)
                .style(GruvboxColors::Light2)
                .width(Length::Fixed(120.0)),
            pick_list(
                &Theme::ALL,
                Some(Theme::Dark), // 默认使用深色主题
                |theme| Message::UpdateTheme(format!("{:?}", theme))
            )
            .style(GruvboxStyle::pick_list())
        ]
        .align_items(Alignment::Center)
        .spacing(10);

        let font_size_row = row![
            text("字体大小:")
                .size(14)
                .style(GruvboxColors::Light2)
                .width(Length::Fixed(120.0)),
            text_input("14", &app.config.ui.font_size.to_string())
                .on_input(Message::UpdateFontSize)
                .style(GruvboxStyle::text_input())
                .width(Length::Fixed(100.0))
        ]
        .align_items(Alignment::Center)
        .spacing(10);

        let cards_per_row_row = row![
            text("每行卡片数:")
                .size(14)
                .style(GruvboxColors::Light2)
                .width(Length::Fixed(120.0)),
            text_input("2", &app.config.ui.cards_per_row.to_string())
                .on_input(Message::UpdateCardsPerRow)
                .style(GruvboxStyle::text_input())
                .width(Length::Fixed(100.0))
        ]
        .align_items(Alignment::Center)
        .spacing(10);

        let show_thumbnails = row![
            text("显示缩略图:")
                .size(14)
                .style(GruvboxColors::Light2)
                .width(Length::Fixed(120.0)),
            checkbox(
                "启用",
                app.config.ui.show_thumbnails,
                |_| Message::ToggleShowThumbnails
            )
            .style(GruvboxStyle::checkbox())
        ]
        .align_items(Alignment::Center)
        .spacing(10);

        let content = column![
            section_title,
            theme_row,
            font_size_row,
            cards_per_row_row,
            show_thumbnails
        ]
        .spacing(15);

        Container::new(content)
            .padding(15)
            .style(GruvboxStyle::card())
            .into()
    }

    fn create_database_settings(app: &ArxivManager) -> Element<Message> {
        let section_title = text("数据库设置")
            .size(18)
            .style(GruvboxColors::Light1);

        let db_path_row = row![
            text("数据库路径:")
                .size(14)
                .style(GruvboxColors::Light2)
                .width(Length::Fixed(120.0)),
            text_input(
                "数据库路径",
                &app.config.database.database_path.to_string_lossy()
            )
            .style(GruvboxStyle::text_input()),
        ]
        .align_items(Alignment::Center)
        .spacing(10);

        let backup_row = row![
            text("自动备份:")
                .size(14)
                .style(GruvboxColors::Light2)
                .width(Length::Fixed(120.0)),
            checkbox("启用", true, |_| Message::ToggleAutoBackup)
                .style(GruvboxStyle::checkbox())
        ]
        .align_items(Alignment::Center)
        .spacing(10);

        let actions_row = row![
            button("备份数据库")
                .on_press(Message::BackupDatabase)
                .style(GruvboxStyle::button()),
            button("恢复数据库")
                .on_press(Message::RestoreDatabase)
                .style(GruvboxStyle::button()),
            button("清理数据库")
                .on_press(Message::CleanDatabase)
                .style(GruvboxStyle::danger_button()),
        ]
        .spacing(10);

        let content = column![section_title, db_path_row, backup_row, actions_row]
            .spacing(15);

        Container::new(content)
            .padding(15)
            .style(GruvboxStyle::card())
            .into()
    }

    fn create_actions_section() -> Element<'static, Message> {
        let section_title = text("操作")
            .size(18)
            .style(GruvboxColors::Light1);

        let actions_row = row![
            button("保存设置")
                .on_press(Message::SaveSettings)
                .style(GruvboxStyle::button()),
            button("重置为默认值")
                .on_press(Message::ResetSettings)
                .style(GruvboxStyle::secondary_button()),
            button("导出设置")
                .on_press(Message::ExportSettings)
                .style(GruvboxStyle::button()),
            button("导入设置")
                .on_press(Message::ImportSettings)
                .style(GruvboxStyle::button()),
        ]
        .spacing(10);

        let content = column![section_title, actions_row].spacing(15);

        Container::new(content)
            .padding(15)
            .style(GruvboxStyle::card())
            .into()
    }
}

use iced::widget::{container, text, column, row, button, checkbox};
use iced::{Element, Length, Alignment};
use crate::app::{AppMessage, AppState};

pub fn settings_view(state: &AppState) -> Element<AppMessage> {
    let title = text("设置")
        .size(24);
    
    // Theme settings
    let theme_section = column![
        text("主题设置").size(18),
        row![
            text("深色主题:").size(14),
            checkbox("", state.theme_dark)
                .on_toggle(|_| AppMessage::ThemeToggled)
        ]
        .spacing(10)
        .align_y(Alignment::Center),
    ]
    .spacing(10);
    
    // Download settings
    let download_section = column![
        text("下载设置").size(18),
        row![
            text("最大并发下载数:").size(14),
            text(format!("{}", state.config.download.max_concurrent_downloads)).size(14)
        ]
        .spacing(10)
        .align_y(Alignment::Center),
        row![
            text("下载目录:").size(14),
            text(state.config.download.download_dir.to_string_lossy()).size(14)
        ]
        .spacing(10)
        .align_y(Alignment::Center),
        row![
            text("自动整理文件:").size(14),
            checkbox("", state.config.download.auto_organize)
        ]
        .spacing(10)
        .align_y(Alignment::Center),
    ]
    .spacing(10);
    
    // UI settings
    let ui_section = column![
        text("界面设置").size(18),
        row![
            text("字体大小:").size(14),
            text(format!("{:.1}", state.config.ui.font_size)).size(14)
        ]
        .spacing(10)
        .align_y(Alignment::Center),
        row![
            text("Vim 模式:").size(14),
            checkbox("", state.config.ui.vim_mode)
        ]
        .spacing(10)
        .align_y(Alignment::Center),
    ]
    .spacing(10);
    
    // Database settings
    let database_section = column![
        text("数据库设置").size(18),
        row![
            text("数据库路径:").size(14),
            text(state.config.database.db_path.to_string_lossy()).size(14)
        ]
        .spacing(10)
        .align_y(Alignment::Center),
        row![
            text("启用备份:").size(14),
            checkbox("", state.config.database.enable_backup)
        ]
        .spacing(10)
        .align_y(Alignment::Center),
    ]
    .spacing(10);
    
    let save_button = button(text("保存设置").size(16))
        .on_press(AppMessage::SettingsChanged(state.config.clone()))
        .padding([10, 20]);
    
    let content = column![
        title,
        container(theme_section).padding(15),
        container(download_section).padding(15),
        container(ui_section).padding(15),
        container(database_section).padding(15),
        save_button
    ]
    .spacing(15)
    .padding(20);
    
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

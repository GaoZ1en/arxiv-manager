use iced::{
    widget::{container, row, column, text, button, text_input, scrollable, Space, progress_bar},
    Element, Length, Alignment, Background, Color, Theme,
};

use crate::app::{ArxivManager, View, Message};
use crate::ui::style;
use super::components::{sidebar, search_view, library_view, downloads_view, settings_view, status_bar, reader_view};

/// 主视图函数
pub fn main_view(app: &ArxivManager) -> Element<Message> {
    let content = row![
        // 侧边栏
        sidebar::Sidebar::view(app),
        
        // 主内容区域
        main_content(app),
    ]
    .spacing(0)
    .width(Length::Fill)
    .height(Length::Fill);

    let mut layout = column![content];

    // 添加状态栏
    layout = layout.push(status_bar::StatusBar::view(app));

    container(layout)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(style::GruvboxStyle::container())
        .into()
}

/// 主内容区域
fn main_content(app: &ArxivManager) -> Element<Message> {
    let content = match app.current_view() {
        View::Search => search_view::SearchView::view(app),
        View::Library => library_view::LibraryView::view(app),
        View::Downloads => downloads_view::DownloadsView::view(app),
        View::Settings => settings_view::SettingsView::view(app),
        View::Reader => reader_view::reader_view(app),
    };

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .style(style::GruvboxStyle::content())
        .into()
        .into()
}



/// 加载指示器
pub fn loading_indicator() -> Element<'static, Message> {
    container(
        column![
            text("加载中...")
                .size(16)
                .style(style::GruvboxColors::Light3),
            Space::with_height(10),
            progress_bar(0.0..=1.0, 0.5)
                .width(200)
        ]
        .align_items(Alignment::Center)
        .spacing(10)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}

/// 空状态视图
pub fn empty_state(message: &str, icon: &str) -> Element<'static, Message> {
    container(
        column![
            text(icon)
                .size(48)
                .style(style::GruvboxColors::Light4),
            Space::with_height(20),
            text(message)
                .size(16)
                .style(style::GruvboxColors::Light3),
        ]
        .align_items(Alignment::Center)
        .spacing(10)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}

/// 错误视图
pub fn error_view(error: &str) -> Element<'static, Message> {
    container(
        column![
            text("⚠")
                .size(48)
                .style(style::GruvboxColors::Red),
            Space::with_height(20),
            text("发生错误")
                .size(18)
                .style(style::GruvboxColors::Red),
            Space::with_height(10),
            text(error)
                .size(14)
                .style(style::GruvboxColors::Light3),
            Space::with_height(20),
            button("重试")
                .style(style::GruvboxStyle::button())
                .on_press(Message::HideError),
        ]
        .align_items(Alignment::Center)
        .spacing(10)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}

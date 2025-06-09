use iced::{
    widget::{
        button, column, container, row, scrollable, text, text_input,
        horizontal_space, vertical_space, progress_bar, Space,
    },
    Element, Length, Color, Background, Alignment,
};

use crate::app::{ArxivManager, Message};
use crate::ui::style;

/// PDF 阅读器视图
pub fn reader_view(app: &ArxivManager) -> Element<Message> {
    let content = if let Some(selected_index) = app.selected_paper {
        // 显示 PDF 阅读器界面
        let header = row![
            button("← 返回")
                .on_press(Message::ChangeView(crate::app::View::Library))
                .style(style::button::secondary),
            horizontal_space(Length::Fill),
            text("PDF 阅读器").size(20),
            horizontal_space(Length::Fill),
            button("全屏")
                .on_press(Message::ShowStatus("全屏模式暂未实现".to_string()))
                .style(style::button::secondary),
        ]
        .align_items(Alignment::Center)
        .spacing(10);

        let toolbar = row![
            button("放大")
                .on_press(Message::ShowStatus("放大功能暂未实现".to_string()))
                .style(style::button::secondary),
            button("缩小")
                .on_press(Message::ShowStatus("缩小功能暂未实现".to_string()))
                .style(style::button::secondary),
            text_input("页码", "1")
                .on_input(|_| Message::ShowStatus("页面跳转暂未实现".to_string()))
                .width(Length::Fixed(80.0)),
            text(" / 10"),
            horizontal_space(Length::Fill),
            button("上一页")
                .on_press(Message::ShowStatus("翻页功能暂未实现".to_string()))
                .style(style::button::secondary),
            button("下一页")
                .on_press(Message::ShowStatus("翻页功能暂未实现".to_string()))
                .style(style::button::secondary),
        ]
        .align_items(Alignment::Center)
        .spacing(10);

        // PDF 显示区域（占位符）
        let pdf_area = container(
            column![
                vertical_space(Length::Fixed(50.0)),
                text("PDF 阅读器")
                    .size(24)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
                vertical_space(Length::Fixed(20.0)),
                text("PDF 显示功能正在开发中...")
                    .size(16)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
                text("将在后续版本中提供完整的 PDF 阅读功能")
                    .size(14)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
                vertical_space(Length::Fixed(30.0)),
                progress_bar(0.0..=100.0, 65.0),
                text("开发进度: 65%")
                    .size(12)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
            ]
            .align_items(Alignment::Center)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme| container::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
            border_radius: 5.0.into(),
            border_width: 1.0,
            border_color: Color::from_rgb(0.3, 0.3, 0.3),
            ..container::Appearance::default()
        });

        column![
            header,
            toolbar,
            pdf_area,
        ]
        .spacing(10)
        .padding(20)
    } else {
        // 没有选择论文时的提示
        container(
            column![
                vertical_space(Length::Fixed(100.0)),
                text("请先从文献库选择要阅读的论文")
                    .size(18)
                    .horizontal_alignment(iced::alignment::Horizontal::Center),
                vertical_space(Length::Fixed(20.0)),
                button("前往文献库")
                    .on_press(Message::ChangeView(crate::app::View::Library))
                    .style(style::button::primary),
            ]
            .align_items(Alignment::Center)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    };

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// 阅读进度侧边栏
pub fn reading_sidebar() -> Element<'static, Message> {
    let content = column![
        text("阅读进度").size(16),
        vertical_space(Length::Fixed(10.0)),
        progress_bar(0.0..=100.0, 35.0),
        text("35% 已完成").size(12),
        vertical_space(Length::Fixed(20.0)),
        text("目录").size(16),
        // 目录列表占位符
        scrollable(
            column![
                button("1. 引言")
                    .on_press(Message::ShowStatus("目录跳转暂未实现".to_string()))
                    .style(style::button::text)
                    .width(Length::Fill),
                button("2. 相关工作")
                    .on_press(Message::ShowStatus("目录跳转暂未实现".to_string()))
                    .style(style::button::text)
                    .width(Length::Fill),
                button("3. 方法")
                    .on_press(Message::ShowStatus("目录跳转暂未实现".to_string()))
                    .style(style::button::text)
                    .width(Length::Fill),
                button("4. 实验")
                    .on_press(Message::ShowStatus("目录跳转暂未实现".to_string()))
                    .style(style::button::text)
                    .width(Length::Fill),
                button("5. 结论")
                    .on_press(Message::ShowStatus("目录跳转暂未实现".to_string()))
                    .style(style::button::text)
                    .width(Length::Fill),
            ]
            .spacing(5)
        ),
        vertical_space(Length::Fixed(20.0)),
        text("笔记").size(16),
        text_input("添加笔记...", "")
            .on_input(|_| Message::ShowStatus("笔记功能暂未实现".to_string())),
    ]
    .spacing(10)
    .padding(20);

    container(content)
        .width(Length::Fixed(300.0))
        .height(Length::Fill)
        .style(|theme| container::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.98, 0.98, 0.98))),
            border_radius: 5.0.into(),
            border_width: 1.0,
            border_color: Color::from_rgb(0.3, 0.3, 0.3),
            ..container::Appearance::default()
        })
        .into()
}

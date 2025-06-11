// 现代化侧边栏组件 - IRC客户端风格

use iced::widget::{button, column, text, scrollable, container, row, horizontal_space, vertical_space};
use iced::{Element, Length, Padding, Alignment};

use crate::core::app_state::ArxivManager;
use crate::core::models::TabContent;
use crate::core::messages::Message;
use crate::ui::style::{sidebar_item_style_dynamic, sidebar_container_dynamic_style};
use crate::ui::theme::ThemeColors;

pub struct Sidebar;

impl Sidebar {
    pub fn view(app: &ArxivManager) -> Element<'_, Message> {
        let theme_colors = app.theme_colors();
        
        // 顶部用户区域
        let user_section = container(
            column![
                row![
                    text("📖").size(20),
                    text("ArXiv Manager")
                            .color(theme_colors.text_primary)
                            .size(16)
                            .font(iced::Font {
                                weight: iced::font::Weight::Bold,
                                ..Default::default()
                            }),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
                text("Academic Paper Manager")
                    .color(theme_colors.text_muted)
                    .size(12),
            ]
            .spacing(4)
        )
        .padding(Padding::new(16.0).bottom(12.0));

        // 导航区域
        let navigation_section = container(
            column![
                // 标题
                container(
                    text("NAVIGATION")
                            .color(theme_colors.text_muted)
                            .size(11)
                            .font(iced::Font {
                                weight: iced::font::Weight::Bold,
                                ..Default::default()
                            })
                )
                .padding(Padding::new(16.0).bottom(8.0).top(0.0)),
                
                // 导航按钮
                sidebar_nav_item("🔍", "Search", 0, app.active_tab == 0, &theme_colors, &app.settings.theme),
                sidebar_nav_item("📚", "Library", 1, app.active_tab == 1, &theme_colors, &app.settings.theme),
                sidebar_nav_item("📥", "Downloads", 2, app.active_tab == 2, &theme_colors, &app.settings.theme),
                sidebar_nav_item("⚙️", "Settings", 3, app.active_tab == 3, &theme_colors, &app.settings.theme),
            ]
            .spacing(2)
        );

        // 收藏的论文区域
        let papers_section = if !app.saved_papers.is_empty() {
            container(
                column![
                    // 标题
                    container(
                        text("SAVED PAPERS")
                            .color(theme_colors.text_muted)
                            .size(11)
                            .font(iced::Font {
                                weight: iced::font::Weight::Bold,
                                ..Default::default()
                            })
                    )
                    .padding(Padding::new(16.0).bottom(8.0).top(16.0)),
                    
                    // 论文列表
                    scrollable(
                        column(
                            app.saved_papers.iter().enumerate().map(|(index, paper)| {
                                paper_item(&paper.title, index, &theme_colors, &app.settings.theme)
                            }).collect::<Vec<Element<Message>>>()
                        ).spacing(2)
                    )
                    .height(Length::Fill)
                ]
            )
        } else {
            container(
                column![
                    container(
                        text("SAVED PAPERS")
                            .color(theme_colors.text_muted)
                            .size(11)
                            .font(iced::Font {
                                weight: iced::font::Weight::Bold,
                                ..Default::default()
                            })
                    )
                    .padding(Padding::new(16.0).bottom(8.0).top(16.0)),
                    
                    container(
                        text("No saved papers yet")
                            .color(theme_colors.text_muted)
                            .size(13)
                    )
                    .padding(Padding::new(16.0))
                ]
            )
        };

        // 底部状态区域
        let status_section = container(
            column![
                container(
                    row![
                        text("●").color(theme_colors.success_color).size(12),
                        text("Ready")
                            .color(theme_colors.text_secondary)
                            .size(12),
                        horizontal_space(),
                        button(text("⚙").color(theme_colors.text_muted))
                            .on_press(Message::TabClicked(3))
                            .style(sidebar_item_style_dynamic(&app.settings.theme))
                            .padding(4),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center)
                )
                .padding(Padding::new(16.0).top(8.0))
            ]
        );

        container(
            column![
                user_section,
                navigation_section,
                papers_section,
                vertical_space(),
                status_section,
            ]
        )
        .width(280)
        .height(Length::Fill)
        .style(sidebar_container_dynamic_style(&app.settings.theme))
        .into()
    }
}

// 导航项目组件
fn sidebar_nav_item<'a>(icon: &'a str, label: &'a str, tab_index: usize, is_active: bool, theme_colors: &ThemeColors, theme: &crate::core::models::Theme) -> Element<'a, Message> {
    let text_color = if is_active { theme_colors.text_primary } else { theme_colors.text_secondary };
    
    if is_active {
        let accent_border = theme_colors.accent_border;
        let theme_clone = theme.clone();
        button(
            row![
                text(icon).size(16).color(text_color),
                text(label)
                    .color(text_color)
                    .size(14)
                    .font(iced::Font {
                        weight: iced::font::Weight::Medium,
                        ..Default::default()
                    }),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
        )
        .on_press(Message::TabClicked(tab_index))
        .width(Length::Fill)
        .style(move |_: &iced::Theme, status| {
            let mut base_style = sidebar_item_style_dynamic(&theme_clone)(&iced::Theme::default(), status);
            base_style.background = Some(iced::Background::Color(accent_border));
            base_style
        })
        .padding(Padding::new(12.0).left(16.0).right(16.0))
        .into()
    } else {
        button(
            row![
                text(icon).size(16).color(text_color),
                text(label)
                    .color(text_color)
                    .size(14)
                    .font(iced::Font {
                        weight: iced::font::Weight::Normal,
                        ..Default::default()
                    }),
            ]
            .spacing(12)
            .align_y(Alignment::Center)
        )
        .on_press(Message::TabClicked(tab_index))
        .width(Length::Fill)
        .style(sidebar_item_style_dynamic(theme))
        .padding(Padding::new(12.0).left(16.0).right(16.0))
        .into()
    }
}

// 论文项目组件
fn paper_item<'a>(title: &'a str, index: usize, theme_colors: &ThemeColors, theme: &crate::core::models::Theme) -> Element<'a, Message> {
    let truncated_title = if title.len() > 35 {
        format!("{}...", &title[..32])
    } else {
        title.to_string()
    };

    button(
        row![
            text("📄").color(theme_colors.text_muted).size(14),
            text(truncated_title)
                .color(theme_colors.text_secondary)
                .size(13),
        ]
        .spacing(10)
        .align_y(Alignment::Center)
    )
    .on_press(Message::NewTab(TabContent::PaperView(index)))
    .width(Length::Fill)
    .style(sidebar_item_style_dynamic(theme))
    .padding(Padding::new(8.0).left(16.0).right(16.0))
    .into()
}

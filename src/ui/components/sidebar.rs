// 现代化侧边栏组件 - IRC客户端风格

use iced::widget::{button, column, text, scrollable, container, row, horizontal_space, vertical_space};
use iced::{Element, Length, Padding, Alignment};

use crate::core::app_state::ArxivManager;
use crate::core::models::TabContent;
use crate::core::messages::Message;
use crate::ui::style::{sidebar_item_style_dynamic, sidebar_container_dynamic_style};
use crate::ui::theme::ThemeColors;
use crate::ui::components::{emoji_text, emoji_text_colored};

pub struct Sidebar;

impl Sidebar {
    pub fn view(app: &ArxivManager) -> Element<'_, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        
        // 顶部用户区域
        let user_section = container(
            column![
                row![
                    emoji_text_colored(app, "📖", base_font_size * 1.4, theme_colors.text_primary),
                    text("ArXiv Manager")
                            .color(theme_colors.text_primary)
                            .size(base_font_size * 1.14)
                            .font(iced::Font {
                                weight: iced::font::Weight::Bold,
                                ..current_font
                            }),
                ]
                .spacing(12.0 * app.current_scale())
                .align_y(Alignment::Center),
                text("Academic Paper Manager")
                    .color(theme_colors.text_muted)
                    .size(base_font_size * 0.86)
                    .font(current_font),
            ]
            .spacing(4.0 * app.current_scale())
        )
        .padding(Padding::new(16.0 * app.current_scale()).bottom(12.0 * app.current_scale()));

        // 导航区域
        let navigation_section = container(
            column![
                // 标题
                container(
                    text("NAVIGATION")
                            .color(theme_colors.text_muted)
                            .size(base_font_size * 0.78)
                            .font(iced::Font {
                                weight: iced::font::Weight::Bold,
                                ..current_font
                            })
                )
                .padding(Padding::new(16.0 * app.current_scale()).bottom(8.0 * app.current_scale()).top(0.0)),
                
                // 导航按钮
                sidebar_nav_item("🔍", "Search", 0, app.active_tab == 0, &theme_colors, &app.settings.theme, app),
                sidebar_nav_item("📚", "Library", 1, app.active_tab == 1, &theme_colors, &app.settings.theme, app),
                sidebar_nav_item("📥", "Downloads", 2, app.active_tab == 2, &theme_colors, &app.settings.theme, app),
                sidebar_nav_item("⚙️", "Settings", 3, app.active_tab == 3, &theme_colors, &app.settings.theme, app),
            ]
            .spacing(2.0 * app.current_scale())
        );

        // 收藏的论文区域
        let papers_section = if !app.saved_papers.is_empty() {
            container(
                column![
                    // 标题
                    container(
                        text("SAVED PAPERS")
                            .color(theme_colors.text_muted)
                            .size(base_font_size * 0.78)
                            .font(iced::Font {
                                weight: iced::font::Weight::Bold,
                                ..current_font
                            })
                    )
                    .padding(Padding::new(16.0 * app.current_scale()).bottom(8.0 * app.current_scale()).top(16.0 * app.current_scale())),
                    
                    // 论文列表
                    scrollable(
                        column(
                            app.saved_papers.iter().enumerate().map(|(index, paper)| {
                                paper_item(&paper.title, index, &theme_colors, &app.settings.theme, app)
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
                            .size(base_font_size * 0.78)
                            .font(iced::Font {
                                weight: iced::font::Weight::Bold,
                                ..current_font
                            })
                    )
                    .padding(Padding::new(16.0 * app.current_scale()).bottom(8.0 * app.current_scale()).top(16.0 * app.current_scale())),
                    
                    container(
                        text("No saved papers yet")
                            .color(theme_colors.text_muted)
                            .size(base_font_size * 0.93)
                            .font(current_font)
                    )
                    .padding(Padding::new(16.0 * app.current_scale()))
                ]
            )
        };

        // 底部状态区域
        let status_section = container(
            column![
                container(
                    row![
                        text("●").color(theme_colors.success_color).size(base_font_size * 0.86).font(current_font),
                        text("Ready")
                            .color(theme_colors.text_secondary)
                            .size(base_font_size * 0.86)
                            .font(current_font),
                        horizontal_space(),
                        button(text("⚙").color(theme_colors.text_muted).font(current_font))
                            .on_press(Message::TabClicked(3))
                            .style(sidebar_item_style_dynamic(&app.settings.theme))
                            .padding(4.0 * app.current_scale()),
                    ]
                    .spacing(8.0 * app.current_scale())
                    .align_y(Alignment::Center)
                )
                .padding(Padding::new(16.0 * app.current_scale()).top(8.0 * app.current_scale()))
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
        .width((280.0 * app.current_scale()) as u16)
        .height(Length::Fill)
        .style(sidebar_container_dynamic_style(&app.settings.theme))
        .into()
    }
}

// 导航项目组件
fn sidebar_nav_item<'a>(icon: &'a str, label: &'a str, tab_index: usize, is_active: bool, theme_colors: &ThemeColors, theme: &crate::core::models::Theme, app: &'a ArxivManager) -> Element<'a, Message> {
    let text_color = if is_active { theme_colors.text_primary } else { theme_colors.text_secondary };
    let current_font = app.current_font();
    let base_font_size = app.current_font_size();
    let scale = app.current_scale();
    
    if is_active {
        let accent_border = theme_colors.accent_border;
        let theme_clone = theme.clone();
        button(
            row![
                emoji_text_colored(app, icon, base_font_size * 1.14, text_color),
                text(label)
                    .color(text_color)
                    .size(base_font_size)
                    .font(current_font),
            ]
            .spacing(12.0 * scale)
            .align_y(Alignment::Center)
        )
        .on_press(Message::TabClicked(tab_index))
        .width(Length::Fill)
        .style(move |_: &iced::Theme, status| {
            let mut base_style = sidebar_item_style_dynamic(&theme_clone)(&iced::Theme::default(), status);
            base_style.background = Some(iced::Background::Color(accent_border));
            base_style
        })
        .padding(Padding::new(12.0 * scale).left(16.0 * scale).right(16.0 * scale))
        .into()
    } else {
        button(
            row![
                emoji_text_colored(app, icon, base_font_size * 1.14, text_color),
                text(label)
                    .color(text_color)
                    .size(base_font_size)
                    .font(current_font),
            ]
            .spacing(12.0 * scale)
            .align_y(Alignment::Center)
        )
        .on_press(Message::TabClicked(tab_index))
        .width(Length::Fill)
        .style(sidebar_item_style_dynamic(theme))
        .padding(Padding::new(12.0 * scale).left(16.0 * scale).right(16.0 * scale))
        .into()
    }
}

// 论文项目组件
fn paper_item<'a>(title: &'a str, index: usize, theme_colors: &ThemeColors, theme: &crate::core::models::Theme, app: &'a ArxivManager) -> Element<'a, Message> {
    let truncated_title = if title.len() > 35 {
        format!("{}...", &title[..32])
    } else {
        title.to_string()
    };
    
    let current_font = app.current_font();
    let base_font_size = app.current_font_size();
    let scale = app.current_scale();

    button(
        row![
            text("📄").color(theme_colors.text_muted).size(base_font_size).font(current_font),
            text(truncated_title)
                .color(theme_colors.text_secondary)
                .size(base_font_size * 0.93)
                .font(current_font),
        ]
        .spacing(10.0 * scale)
        .align_y(Alignment::Center)
    )
    .on_press(Message::NewTab(TabContent::PaperView(index)))
    .width(Length::Fill)
    .style(sidebar_item_style_dynamic(theme))
    .padding(Padding::new(8.0 * scale).left(16.0 * scale).right(16.0 * scale))
    .into()
}

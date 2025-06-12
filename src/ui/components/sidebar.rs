// 现代化侧边栏组件 - IRC客户端风格

use iced::widget::{button, column, scrollable, container, row, horizontal_space, vertical_space, text};
use iced::{Element, Length, Padding, Alignment};

use crate::core::app_state::ArxivManager;
use crate::core::models::TabContent;
use crate::core::messages::Message;
use crate::ui::style::{sidebar_item_style_dynamic, sidebar_container_dynamic_style, scrollable_style_dynamic};
use crate::ui::theme::ThemeColors;


pub struct Sidebar;

impl Sidebar {
    pub fn view(app: &ArxivManager) -> Element<'_, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        
        // 顶部用户区域 - 已删除所有标题文本
        let user_section = container(
            column![]
        )
        .padding(Padding::new(8.0 * app.current_scale()).bottom(8.0 * app.current_scale()));

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
                .padding(Padding::new(8.0 * app.current_scale()).bottom(4.0 * app.current_scale()).top(0.0)),
                
                // 导航按钮 - Nerd Font图标
                sidebar_nav_item(" SEARCH", TabContent::Search, is_current_content(&app, &TabContent::Search), &theme_colors, &app.settings.theme, app),
                sidebar_nav_item(" LIBRARY", TabContent::Library, is_current_content(&app, &TabContent::Library), &theme_colors, &app.settings.theme, app),
                sidebar_nav_item(" DOWNLOADS", TabContent::Downloads, is_current_content(&app, &TabContent::Downloads), &theme_colors, &app.settings.theme, app),
                sidebar_nav_item(" SETTINGS", TabContent::Settings, is_current_content(&app, &TabContent::Settings), &theme_colors, &app.settings.theme, app),
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
                    .padding(Padding::new(8.0 * app.current_scale()).bottom(4.0 * app.current_scale()).top(8.0 * app.current_scale())),
                    
                    // 论文列表
                    scrollable(
                        column(
                            app.saved_papers.iter().enumerate().map(|(index, paper)| {
                                paper_item(&paper.title, index, &theme_colors, &app.settings.theme, app)
                            }).collect::<Vec<Element<Message>>>()
                        ).spacing(2)
                    )
                    .style(scrollable_style_dynamic(&app.settings.theme))
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
                    .padding(Padding::new(8.0 * app.current_scale()).bottom(4.0 * app.current_scale()).top(8.0 * app.current_scale())),
                    
                    container(
                        text("No saved papers yet")
                            .color(theme_colors.text_muted)
                            .size(base_font_size * 0.93)
                            .font(current_font)
                    )
                    .padding(Padding::new(8.0 * app.current_scale()))
                ]
            )
        };

        // 底部状态区域
        let status_section = container(
            column![
                container(
                    row![
                        text("READY").color(theme_colors.success_color).size(base_font_size * 0.86).font(current_font),
                        horizontal_space(),
                        button(text("Settings").color(theme_colors.text_muted).size(base_font_size * 0.86).font(current_font))
                            .on_press(if app.tabs.iter().any(|tab| tab.content == TabContent::Settings) {
                                if let Some(index) = app.tabs.iter().position(|tab| tab.content == TabContent::Settings) {
                                    Message::TabClicked(index)
                                } else {
                                    Message::NewTab(TabContent::Settings)
                                }
                            } else {
                                Message::NewTab(TabContent::Settings)
                            })
                            .style(sidebar_item_style_dynamic(&app.settings.theme))
                            .padding(8.0 * app.current_scale()),
                    ]
                    .spacing(8.0 * app.current_scale())
                    .align_y(Alignment::Center)
                )
                .padding(Padding::new(8.0 * app.current_scale()).top(4.0 * app.current_scale()))
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
        .width((140.0 * app.current_scale()) as u16)
        .height(Length::Fill)
        .style(sidebar_container_dynamic_style(&app.settings.theme))
        .into()
    }
}

// 检查当前内容是否激活的辅助函数
fn is_current_content(app: &ArxivManager, content: &TabContent) -> bool {
    if let Some(current_tab) = app.tabs.get(app.active_tab) {
        &current_tab.content == content
    } else {
        false
    }
}

// 导航项目组件 - 只有文本，无图标
fn sidebar_nav_item<'a>(label: &'a str, content: TabContent, is_active: bool, theme_colors: &ThemeColors, theme: &crate::core::models::Theme, app: &'a ArxivManager) -> Element<'a, Message> {
    let text_color = if is_active { theme_colors.text_primary } else { theme_colors.text_secondary };
    let current_font = app.current_font();
    let base_font_size = app.current_font_size();
    let scale = app.current_scale();
    
    // 找到对应的标签页索引，如果不存在则创建新标签页
    let message = if let Some(tab_index) = app.tabs.iter().position(|tab| tab.content == content) {
        Message::TabClicked(tab_index)
    } else {
        Message::NewTab(content)
    };
    
    if is_active {
        let accent_border = theme_colors.accent_border;
        let theme_clone = theme.clone();
        button(
            text(label)
                .color(text_color)
                .size(base_font_size)
                .font(current_font)
        )
        .on_press(message)
        .width(Length::Fill)
        .style(move |_: &iced::Theme, status| {
            let mut base_style = sidebar_item_style_dynamic(&theme_clone)(&iced::Theme::default(), status);
            base_style.background = Some(iced::Background::Color(accent_border));
            base_style
        })
        .padding(Padding::new(8.0 * scale).left(8.0 * scale).right(8.0 * scale))
        .into()
    } else {
        button(
            text(label)
                .color(text_color)
                .size(base_font_size)
                .font(current_font)
        )
        .on_press(message)
        .width(Length::Fill)
        .style(sidebar_item_style_dynamic(theme))
        .padding(Padding::new(8.0 * scale).left(8.0 * scale).right(8.0 * scale))
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
            text("DOC").color(theme_colors.text_muted).size(base_font_size).font(current_font),
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
    .padding(Padding::new(8.0 * scale).left(8.0 * scale).right(8.0 * scale))
    .into()
}

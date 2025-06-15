// 现代化搜索视图 - IRC客户端风格

use iced::widget::{
    button, column, container, row, text, text_input, scrollable, 
    horizontal_rule, pick_list, checkbox, Space
};
use iced::{Element, Length, Background, Border, Shadow};

use crate::core::app_state::ArxivManager;
use crate::core::models::{SearchField, SortBy, SortOrder, DateRange, ArxivCategory};
use crate::core::messages::Message;
use crate::ui::style::{button_primary_style_dynamic, button_secondary_style_dynamic, text_input_dynamic_style, chat_container_dynamic_style, pick_list_dynamic_style, scrollable_tab_style_dynamic_with_fade, ultra_thin_vertical_scrollbar};
use crate::ui::components::WaterfallLayout;

pub struct SearchView;

impl SearchView {
    pub fn view(app: &ArxivManager) -> Element<'_, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let scale = app.current_scale();
        
        let search_input = text_input("Search arXiv papers...", &app.search_query)
            .on_input(Message::SearchQueryChanged)
            .on_submit(Message::SearchSubmitted)
            .style(text_input_dynamic_style(&app.settings.theme))
            .padding(12.0 * scale)
            .size(base_font_size)
            .font(current_font);

        let search_button = button(
            text("Search")
                .color(theme_colors.text_primary)
                .size(base_font_size)
                .font(current_font)
        )
        .on_press(Message::SearchSubmitted)
        .style(button_primary_style_dynamic(&app.settings.theme))
        .padding([10.0 * scale, 8.0 * scale]);

        let advanced_toggle = button(
            text(if app.advanced_search_visible { "Hide Advanced" } else { "Advanced" })
                .color(theme_colors.text_secondary)
                .size(base_font_size)
                .font(current_font)
        )
        .on_press(Message::AdvancedSearchToggled)
        .style(button_secondary_style_dynamic(&app.settings.theme))
        .padding([10.0 * scale, 8.0 * scale]);

        let search_bar = row![search_input, search_button, advanced_toggle]
            .spacing(12.0 * scale)
            .padding(16.0 * scale)
            .align_y(iced::Alignment::Center);

        let mut main_content = vec![search_bar.into()];

        // 搜索建议
        if app.show_search_suggestions && !app.search_suggestions.is_empty() {
            let suggestions = column(
                app.search_suggestions.iter().map(|suggestion| {
                    let (display_text, message) = if suggestion.starts_with("Search by author: ") {
                        let author_name = suggestion[18..].to_string(); // 去掉"Search by author: "前缀
                        (suggestion.clone(), Message::SearchByAuthor(author_name))
                    } else {
                        (suggestion.clone(), Message::SearchSuggestionSelected(suggestion.clone()))
                    };
                    
                    button(
                        text(display_text)
                            .size(base_font_size)
                            .font(current_font)
                    )
                    .on_press(message)
                    .width(Length::Fill)
                    .style(button_secondary_style_dynamic(&app.settings.theme))
                    .into()
                }).collect::<Vec<Element<Message>>>()
            )
            .spacing(4.0 * scale)
            .padding([8.0 * scale, 16.0 * scale]);

            let suggestions_container = container(suggestions)
                .style(move |_theme| iced::widget::container::Style {
                    background: Some(Background::Color(theme_colors.sidebar_bg)),
                    border: Border {
                        color: theme_colors.border_color,
                        width: 1.0,
                        radius: 8.0.into(),
                    },
                    text_color: Some(theme_colors.text_primary),
                    shadow: Shadow::default(),
                });

            main_content.push(suggestions_container.into());
        }

        // 高级搜索面板
        if app.advanced_search_visible {
            main_content.push(Self::advanced_search_panel(app));
        }

        main_content.push(
            horizontal_rule(1)
                .style(move |_theme| iced::widget::rule::Style {
                    color: theme_colors.border_color,
                    width: 1,
                    radius: 0.0.into(),
                    fill_mode: iced::widget::rule::FillMode::Full,
                })
                .into()
        );

        let results_content = if app.is_searching {
            column![
                text("Searching...")
                    .color(theme_colors.text_secondary)
                    .size(base_font_size * 1.14)
                    .font(current_font)
            ]
            .spacing(16.0 * scale)
            .padding(24.0 * scale)
            .align_x(iced::Alignment::Center)
        } else if let Some(error) = &app.search_error {
            column![
                text("Error:")
                    .color(theme_colors.error_color)
                    .size(base_font_size * 1.14)
                    .font(current_font),
                text(error)
                    .color(theme_colors.text_primary)
                    .size(base_font_size)
                    .font(current_font)
            ]
            .spacing(8.0 * scale)
            .padding(24.0 * scale)
        } else if app.search_results.is_empty() {
            column![
                text("No results found")
                    .color(theme_colors.text_muted)
                    .size(base_font_size * 1.14)
                    .font(current_font)
            ]
            .spacing(16.0 * scale)
            .padding(24.0 * scale)
            .align_x(iced::Alignment::Center)
        } else {
            // 使用智能瀑布流布局显示搜索结果（搜索视图专用）
            let waterfall = WaterfallLayout::search_view(app, &app.search_results);
            
            let mut results_column = vec![waterfall.into()];
            
            // 只显示加载状态指示器，不显示手动按钮
            if app.is_loading_more {
                results_column.push(
                    container(
                        text("Loading more results...")
                            .color(theme_colors.text_muted)
                            .size(base_font_size)
                            .font(current_font)
                    )
                    .padding(16.0 * scale)
                    .width(Length::Fill)
                    .align_x(iced::Alignment::Center)
                    .into()
                );
            } else if !app.has_more_results && app.total_results_loaded > 0 {
                results_column.push(
                    container(
                        text(format!("Showing all {} results", app.total_results_loaded))
                            .color(theme_colors.text_muted)
                            .size(base_font_size * 0.9)
                            .font(current_font)
                    )
                    .padding(16.0 * scale)
                    .width(Length::Fill)
                    .align_x(iced::Alignment::Center)
                    .into()
                );
            }
            
            column(results_column)
                .spacing(8.0 * scale)
        };

        main_content.push(
            scrollable(
                container(results_content)
                    .padding(16.0 * scale)
                    .width(Length::Fill)
            )
            .direction(ultra_thin_vertical_scrollbar())
            .style(scrollable_tab_style_dynamic_with_fade(
                &app.settings.theme, 
                app.get_scrollbar_alpha("search_view")
            ))
            .height(Length::Fill)
            .on_scroll(|viewport| {
                // 记录滚动条活动并检测是否滚动到了底部
                let scroll_threshold = 0.9; // 90%位置就开始加载
                if viewport.relative_offset().y >= scroll_threshold {
                    Message::ScrolledToBottom
                } else {
                    // 发送滚动条活动消息
                    Message::ScrollbarActivity("search_view".to_string())
                }
            })
            .into()
        );

        container(column(main_content))
            .style(chat_container_dynamic_style(&app.settings.theme))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn advanced_search_panel(app: &ArxivManager) -> Element<'_, Message> {
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let scale = app.current_scale();
        
        // 搜索字段选择
        let search_field_list = pick_list(
            SearchField::all_variants(),
            Some(app.search_config.search_in.clone()),
            Message::SearchFieldChanged,
        )
        .placeholder("Search in...")
        .style(pick_list_dynamic_style(&app.settings.theme))
        .text_size(base_font_size)
        .font(current_font)
        .padding(8.0 * scale);

        // 排序选项
        let sort_by_list = pick_list(
            SortBy::all_variants(),
            Some(app.search_config.sort_by.clone()),
            Message::SortByChanged,
        )
        .placeholder("Sort by...")
        .style(pick_list_dynamic_style(&app.settings.theme))
        .text_size(base_font_size)
        .font(current_font)
        .padding(8.0 * scale);

        let sort_order_list = pick_list(
            SortOrder::all_variants(),
            Some(app.search_config.sort_order.clone()),
            Message::SortOrderChanged,
        )
        .placeholder("Order...")
        .style(pick_list_dynamic_style(&app.settings.theme))
        .text_size(base_font_size)
        .font(current_font)
        .padding(8.0 * scale);

        // 最大结果数
        let max_results_input = text_input(
            "Max results (1-100)", 
            &app.search_config.max_results.to_string()
        )
        .on_input(Message::MaxResultsChanged)
        .style(text_input_dynamic_style(&app.settings.theme))
        .padding(8.0 * scale)
        .size(base_font_size)
        .font(current_font);

        // 日期范围选择 (使用pick_list而不是单独的输入框)
        let date_range_list = pick_list(
            vec![
                DateRange::Any,
                DateRange::LastWeek,
                DateRange::LastMonth,
                DateRange::LastYear,
            ],
            Some(app.search_config.date_range.clone()),
            Message::DateRangeChanged,
        )
        .placeholder("Date range...")
        .style(pick_list_dynamic_style(&app.settings.theme))
        .text_size(base_font_size)
        .font(current_font)
        .padding(8.0 * scale);

        // 类别选择
        let categories_section = {
            let popular_categories = ArxivCategory::popular_categories();
            let mut category_rows = Vec::new();
            for chunk in popular_categories.chunks(2) {
                let mut row_items = Vec::new();
                for category in chunk {
                    let is_selected = app.search_config.categories.contains(category);
                    let category_clone = category.clone();
                    let checkbox = checkbox(
                        category.display_name(),
                        is_selected
                    )
                    .on_toggle(move |_| Message::CategoryToggled(category_clone))
                    .size(16.0 * scale)
                    .spacing(8.0 * scale)
                    .text_size(base_font_size)
                    .font(current_font);
                    
                    row_items.push(container(checkbox).width(Length::FillPortion(1)).into());
                }
                // 如果行只有一个项目，添加一个空的填充项
                if row_items.len() == 1 {
                    row_items.push(container(Space::new(0, 0)).width(Length::FillPortion(1)).into());
                }
                category_rows.push(row(row_items).spacing(12.0 * scale).into());
            }
            column(category_rows).spacing(8.0 * scale)
        };

        container(
            column![
                row![
                    text("Search in:")
                        .color(theme_colors.text_secondary)
                        .size(base_font_size)
                        .font(current_font),
                    search_field_list
                ]
                .spacing(12.0 * scale)
                .align_y(iced::Alignment::Center),
                
                row![
                    text("Sort by:")
                        .color(theme_colors.text_secondary)
                        .size(base_font_size)
                        .font(current_font),
                    sort_by_list,
                    sort_order_list
                ]
                .spacing(12.0 * scale)
                .align_y(iced::Alignment::Center),
                
                row![
                    text("Max results:")
                        .color(theme_colors.text_secondary)
                        .size(base_font_size)
                        .font(current_font),
                    max_results_input
                ]
                .spacing(12.0 * scale)
                .align_y(iced::Alignment::Center),
                
                row![
                    text("Date range:")
                        .color(theme_colors.text_secondary)
                        .size(base_font_size)
                        .font(current_font),
                    date_range_list
                ]
                .spacing(12.0 * scale)
                .align_y(iced::Alignment::Center),

                // 作者搜索部分
                column![
                    text("Authors:")
                        .color(theme_colors.text_secondary)
                        .size(base_font_size)
                        .font(current_font),
                    row![
                        text_input("Enter author name (e.g., Geoffrey Hinton)", &app.author_input)
                            .on_input(Message::AuthorInputChanged)
                            .style(text_input_dynamic_style(&app.settings.theme))
                            .padding(8.0 * scale)
                            .size(base_font_size)
                            .font(current_font),
                        button(
                            text("Add")
                                .size(base_font_size)
                                .font(current_font)
                        )
                        .on_press(Message::AuthorAdded(app.author_input.clone()))
                        .style(button_secondary_style_dynamic(&app.settings.theme))
                        .padding(8.0 * scale),
                        button(
                            text("Search by Author")
                                .size(base_font_size)
                                .font(current_font)
                        )
                        .on_press(Message::SearchByAuthor(app.author_input.clone()))
                        .style(button_primary_style_dynamic(&app.settings.theme))
                        .padding(8.0 * scale)
                    ]
                    .spacing(8.0 * scale)
                    .align_y(iced::Alignment::Center),
                    // 显示已添加的作者
                    if !app.search_config.authors.is_empty() {
                        column(
                            app.search_config.authors.iter().enumerate().map(|(index, author)| {
                                row![
                                    text(author.clone())
                                        .size(base_font_size)
                                        .font(current_font)
                                        .color(theme_colors.text_primary),
                                    button(
                                        text("×")
                                            .size(base_font_size)
                                            .font(current_font)
                                    )
                                    .on_press(Message::AuthorRemoved(index))
                                    .style(button_secondary_style_dynamic(&app.settings.theme))
                                    .padding(4.0 * scale)
                                ]
                                .spacing(8.0 * scale)
                                .align_y(iced::Alignment::Center)
                                .into()
                            }).collect::<Vec<Element<Message>>>()
                        )
                        .spacing(4.0 * scale)
                        .into()
                    } else {
                        Element::from(Space::new(0, 0))
                    }
                ]
                .spacing(8.0 * scale),

                // 类别选择部分
                column![
                    text("Categories:")
                        .color(theme_colors.text_secondary)
                        .size(base_font_size)
                        .font(current_font),
                    categories_section
                ]
                .spacing(8.0 * scale),
            ]
            .spacing(16.0 * scale)
            .padding(16.0 * scale)
        )
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(theme_colors.sidebar_bg)),
            border: Border {
                color: theme_colors.border_color,
                width: 1.0,
                radius: 8.0.into(),
            },
            text_color: Some(theme_colors.text_primary),
            shadow: Shadow::default(),
        })
        .into()
    }
}

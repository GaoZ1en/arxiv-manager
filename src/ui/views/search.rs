// 现代化搜索视图 - IRC客户端风格

use iced::widget::{
    button, column, container, row, text, text_input, scrollable, 
    horizontal_rule, pick_list
};
use iced::{Element, Length, Background, Border, Shadow};

use crate::core::app_state::ArxivManager;
use crate::core::models::{SearchField, SortBy, SortOrder, DateRange};
use crate::core::messages::Message;
use crate::ui::style::{button_primary_style_dynamic, button_secondary_style_dynamic, text_input_dynamic_style, chat_container_dynamic_style, pick_list_dynamic_style};
use crate::ui::components::PaperCard;

pub struct SearchView;

impl SearchView {
    pub fn view(app: &ArxivManager) -> Element<'_, Message> {
        let theme_colors = app.theme_colors();
        
        let search_input = text_input("Search arXiv papers...", &app.search_query)
            .on_input(Message::SearchQueryChanged)
            .on_submit(Message::SearchSubmitted)
            .style(text_input_dynamic_style(&app.settings.theme))
            .padding(12)
            .size(14);

        let search_button = button(
            text("Search")
                .color(theme_colors.text_primary)
                .size(14)
        )
        .on_press(Message::SearchSubmitted)
        .style(button_primary_style_dynamic(&app.settings.theme))
        .padding([10, 16]);

        let advanced_toggle = button(
            text(if app.advanced_search_visible { "Hide Advanced" } else { "Advanced" })
                .color(theme_colors.text_secondary)
                .size(14)
        )
        .on_press(Message::AdvancedSearchToggled)
        .style(button_secondary_style_dynamic(&app.settings.theme))
        .padding([10, 16]);

        let search_bar = row![search_input, search_button, advanced_toggle]
            .spacing(12)
            .padding(16)
            .align_y(iced::Alignment::Center);

        let mut main_content = vec![search_bar.into()];

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
                    .size(16)
            ]
            .spacing(16)
            .padding(24)
            .align_x(iced::Alignment::Center)
        } else if let Some(error) = &app.search_error {
            column![
                text("Error:")
                    .color(theme_colors.error_color)
                    .size(16),
                text(error)
                    .color(theme_colors.text_primary)
                    .size(14)
            ]
            .spacing(8)
            .padding(24)
        } else if app.search_results.is_empty() {
            column![
                text("No results found")
                    .color(theme_colors.text_muted)
                    .size(16)
            ]
            .spacing(16)
            .padding(24)
            .align_x(iced::Alignment::Center)
        } else {
            column(
                app.search_results.iter().map(|paper| {
                    PaperCard::view(app, paper, false)
                }).collect::<Vec<Element<Message>>>()
            ).spacing(12)
        };

        main_content.push(
            scrollable(
                container(results_content)
                    .padding(16)
                    .width(Length::Fill)
            )
            .height(Length::Fill)
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
        
        // 搜索字段选择
        let search_field_list = pick_list(
            SearchField::all_variants(),
            Some(app.search_config.search_in.clone()),
            Message::SearchFieldChanged,
        )
        .placeholder("Search in...")
        .style(pick_list_dynamic_style(&app.settings.theme))
        .padding(8);

        // 排序选项
        let sort_by_list = pick_list(
            SortBy::all_variants(),
            Some(app.search_config.sort_by.clone()),
            Message::SortByChanged,
        )
        .placeholder("Sort by...")
        .style(pick_list_dynamic_style(&app.settings.theme))
        .padding(8);

        let sort_order_list = pick_list(
            SortOrder::all_variants(),
            Some(app.search_config.sort_order.clone()),
            Message::SortOrderChanged,
        )
        .placeholder("Order...")
        .style(pick_list_dynamic_style(&app.settings.theme))
        .padding(8);

        // 最大结果数
        let max_results_input = text_input(
            "Max results (1-100)", 
            &app.search_config.max_results.to_string()
        )
        .on_input(Message::MaxResultsChanged)
        .style(text_input_dynamic_style(&app.settings.theme))
        .padding(8)
        .size(14);

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
        .padding(8);

        container(
            column![
                row![
                    text("Search in:")
                        .color(theme_colors.text_secondary)
                        .size(14),
                    search_field_list
                ]
                .spacing(12)
                .align_y(iced::Alignment::Center),
                
                row![
                    text("Sort by:")
                        .color(theme_colors.text_secondary)
                        .size(14),
                    sort_by_list,
                    sort_order_list
                ]
                .spacing(12)
                .align_y(iced::Alignment::Center),
                
                row![
                    text("Max results:")
                        .color(theme_colors.text_secondary)
                        .size(14),
                    max_results_input
                ]
                .spacing(12)
                .align_y(iced::Alignment::Center),
                
                row![
                    text("Date range:")
                        .color(theme_colors.text_secondary)
                        .size(14),
                    date_range_list
                ]
                .spacing(12)
                .align_y(iced::Alignment::Center),
            ]
            .spacing(16)
            .padding(16)
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

// 搜索视图

use iced::widget::{
    button, column, container, row, text, text_input, scrollable, 
    horizontal_rule, pick_list
};
use iced::{Element, Length, Color, Background, Border, Shadow};

use crate::core::app_state::ArxivManager;
use crate::core::models::{SearchField, SortBy, SortOrder};
use crate::core::messages::Message;
use crate::ui::style::{button_primary_style, button_secondary_style};
use crate::ui::components::PaperCard;
use crate::ui::theme::*;

pub struct SearchView;

impl SearchView {
    pub fn view(app: &ArxivManager) -> Element<'_, Message> {
        let search_input = text_input("Search arXiv papers...", &app.search_query)
            .on_input(Message::SearchQueryChanged)
            .on_submit(Message::SearchSubmitted)
            .style(|_theme, status| iced::widget::text_input::Style {
                background: Background::Color(GRUVBOX_SURFACE),
                border: Border {
                    color: match status {
                        iced::widget::text_input::Status::Focused => GRUVBOX_GREEN,
                        _ => GRUVBOX_BORDER,
                    },
                    width: 1.0,
                    radius: 4.0.into(),
                },
                icon: Color::TRANSPARENT,
                placeholder: GRUVBOX_TEXT_MUTED,
                value: GRUVBOX_TEXT,
                selection: GRUVBOX_GREEN,
            });

        let search_button = button(text("Search").color(Color::BLACK))
            .on_press(Message::SearchSubmitted)
            .style(button_primary_style);

        let advanced_toggle = button(
            text(if app.advanced_search_visible { "Hide Advanced" } else { "Advanced" })
                .color(GRUVBOX_TEXT)
        )
        .on_press(Message::AdvancedSearchToggled)
        .style(button_secondary_style);

        let search_bar = row![search_input, search_button, advanced_toggle]
            .spacing(10)
            .padding(10);

        let mut main_content = vec![search_bar.into()];

        // 高级搜索面板
        if app.advanced_search_visible {
            main_content.push(Self::advanced_search_panel(app));
        }

        main_content.push(horizontal_rule(1).into());

        let results_content = if app.is_searching {
            column![text("Searching...").color(GRUVBOX_TEXT)]
        } else if let Some(error) = &app.search_error {
            column![
                text("Error:").color(GRUVBOX_RED),
                text(error).color(GRUVBOX_TEXT)
            ]
        } else if app.search_results.is_empty() {
            column![text("No results").color(GRUVBOX_TEXT_MUTED)]
        } else {
            column(
                app.search_results.iter().map(|paper| {
                    PaperCard::view(app, paper, false)
                }).collect::<Vec<Element<Message>>>()
            ).spacing(10)
        };

        main_content.push(scrollable(results_content).height(Length::Fill).into());

        container(column(main_content))
        .padding(20)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_BG)),
            border: Border::default(),
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    fn advanced_search_panel(app: &ArxivManager) -> Element<'_, Message> {
        // 搜索字段选择
        let search_field_list = pick_list(
            SearchField::all_variants(),
            Some(app.search_config.search_in.clone()),
            Message::SearchFieldChanged,
        )
        .placeholder("Search in...")
        .style(|_theme, status| iced::widget::pick_list::Style {
            text_color: GRUVBOX_TEXT,
            background: Background::Color(GRUVBOX_SURFACE),
            border: Border {
                color: match status {
                    iced::widget::pick_list::Status::Active => GRUVBOX_BORDER,
                    iced::widget::pick_list::Status::Hovered => GRUVBOX_GREEN,
                    iced::widget::pick_list::Status::Opened => GRUVBOX_GREEN,
                },
                width: 1.0,
                radius: 4.0.into(),
            },
            handle_color: GRUVBOX_TEXT,
            placeholder_color: GRUVBOX_TEXT_MUTED,
        });

        // 排序选项
        let sort_by_list = pick_list(
            SortBy::all_variants(),
            Some(app.search_config.sort_by.clone()),
            Message::SortByChanged,
        )
        .placeholder("Sort by...")
        .style(|_theme, status| iced::widget::pick_list::Style {
            text_color: GRUVBOX_TEXT,
            background: Background::Color(GRUVBOX_SURFACE),
            border: Border {
                color: match status {
                    iced::widget::pick_list::Status::Active => GRUVBOX_BORDER,
                    iced::widget::pick_list::Status::Hovered => GRUVBOX_GREEN,
                    iced::widget::pick_list::Status::Opened => GRUVBOX_GREEN,
                },
                width: 1.0,
                radius: 4.0.into(),
            },
            handle_color: GRUVBOX_TEXT,
            placeholder_color: GRUVBOX_TEXT_MUTED,
        });

        let sort_order_list = pick_list(
            SortOrder::all_variants(),
            Some(app.search_config.sort_order.clone()),
            Message::SortOrderChanged,
        )
        .placeholder("Order...")
        .style(|_theme, status| iced::widget::pick_list::Style {
            text_color: GRUVBOX_TEXT,
            background: Background::Color(GRUVBOX_SURFACE),
            border: Border {
                color: match status {
                    iced::widget::pick_list::Status::Active => GRUVBOX_BORDER,
                    iced::widget::pick_list::Status::Hovered => GRUVBOX_GREEN,
                    iced::widget::pick_list::Status::Opened => GRUVBOX_GREEN,
                },
                width: 1.0,
                radius: 4.0.into(),
            },
            handle_color: GRUVBOX_TEXT,
            placeholder_color: GRUVBOX_TEXT_MUTED,
        });

        // 最大结果数
        let max_results_input = text_input(
            "Max results (1-100)", 
            &app.search_config.max_results.to_string()
        )
        .on_input(Message::MaxResultsChanged)
        .style(|_theme, status| iced::widget::text_input::Style {
            background: Background::Color(GRUVBOX_SURFACE),
            border: Border {
                color: match status {
                    iced::widget::text_input::Status::Focused => GRUVBOX_GREEN,
                    _ => GRUVBOX_BORDER,
                },
                width: 1.0,
                radius: 4.0.into(),
            },
            icon: Color::TRANSPARENT,
            placeholder: GRUVBOX_TEXT_MUTED,
            value: GRUVBOX_TEXT,
            selection: GRUVBOX_GREEN,
        });

        container(
            column![
                row![
                    column![
                        text("Search in:").color(GRUVBOX_TEXT).size(14),
                        search_field_list
                    ].spacing(4).width(Length::FillPortion(1)),
                    column![
                        text("Sort by:").color(GRUVBOX_TEXT).size(14),
                        sort_by_list
                    ].spacing(4).width(Length::FillPortion(1)),
                    column![
                        text("Order:").color(GRUVBOX_TEXT).size(14),
                        sort_order_list
                    ].spacing(4).width(Length::FillPortion(1)),
                    column![
                        text("Max results:").color(GRUVBOX_TEXT).size(14),
                        max_results_input
                    ].spacing(4).width(Length::FillPortion(1)),
                ].spacing(20),
            ].spacing(10)
        )
        .padding(15)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_SURFACE)),
            border: Border {
                color: GRUVBOX_BORDER,
                width: 1.0,
                radius: 6.0.into(),
            },
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }
}

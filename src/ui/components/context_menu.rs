// Tab context menu component with English-only text

use iced::widget::{button, column, container, text};
use iced::{Element, Length, Color, Background, Border};

use crate::core::app_state::ArxivManager;
use crate::core::models::ui::TabGroup;
use crate::core::messages::Message;
use crate::ui::style::button_secondary_style_dynamic;

pub struct ContextMenu;

#[derive(Debug, Clone)]
pub struct ContextMenuState {
    pub visible: bool,
    pub tab_index: usize,
    pub x: f32,
    pub y: f32,
}

impl Default for ContextMenuState {
    fn default() -> Self {
        Self {
            visible: false,
            tab_index: 0,
            x: 0.0,
            y: 0.0,
        }
    }
}

impl ContextMenu {
    pub fn view<'a>(state: &'a ContextMenuState, app: &'a ArxivManager) -> Element<'a, Message> {
        if !state.visible || state.tab_index >= app.tabs.len() {
            return container(text("")).into();
        }

        let tab = &app.tabs[state.tab_index];
        let theme_colors = app.theme_colors();
        let current_font = app.current_font();
        let base_font_size = app.current_font_size();
        let scale = app.current_scale();

        let mut menu_items = column![];

        // Duplicate Tab
        menu_items = menu_items.push(
            button(
                text("Duplicate")
                    .size(base_font_size * 0.9)
                    .font(current_font)
                    .color(theme_colors.text_primary)
            )
            .on_press(Message::TabDuplicate(state.tab_index))
            .style(button_secondary_style_dynamic(&app.settings.theme))
            .width(Length::Fill)
            .padding([4.0 * scale, 8.0 * scale])
        );

        // Group options
        let groups = vec![
            (TabGroup::Default, "Default"),
            (TabGroup::Research, "Research"),
            (TabGroup::Library, "Library"),
            (TabGroup::Downloads, "Downloads"),
        ];

        for (group, name) in groups {
            if group != tab.group {
                menu_items = menu_items.push(
                    button(
                        text(format!("  {}", name))
                            .size(base_font_size * 0.9)
                            .font(current_font)
                            .color(theme_colors.text_secondary)
                    )
                    .on_press(Message::TabMoveToGroup(state.tab_index, group))
                    .style(button_secondary_style_dynamic(&app.settings.theme))
                    .width(Length::Fill)
                    .padding([4.0 * scale, 8.0 * scale])
                );
            }
        }

        // Separator
        menu_items = menu_items.push(
            container(text("").height(1))
                .style(move |_theme: &iced::Theme| {
                    container::Style {
                        background: Some(Background::Color(theme_colors.border_color)),
                        ..Default::default()
                    }
                })
                .width(Length::Fill)
                .padding([2.0 * scale, 0.0])
        );

        // Duplicate tab
        menu_items = menu_items.push(
            button(
                text("Duplicate")
                    .size(base_font_size * 0.9)
                    .font(current_font)
                    .color(theme_colors.text_primary)
            )
            .on_press(Message::TabDuplicate(state.tab_index))
            .style(button_secondary_style_dynamic(&app.settings.theme))
            .width(Length::Fill)
            .padding([4.0 * scale, 8.0 * scale])
        );

        // Close operations
        if tab.closable {
            // Separator
            menu_items = menu_items.push(
                container(text("").height(1))
                    .style(move |_theme: &iced::Theme| {
                        container::Style {
                            background: Some(Background::Color(theme_colors.border_color)),
                            ..Default::default()
                        }
                    })
                    .width(Length::Fill)
                    .padding([2.0 * scale, 0.0])
            );

            menu_items = menu_items.push(
                button(
                    text("Close Tabs to Right")
                        .size(base_font_size * 0.9)
                        .font(current_font)
                        .color(theme_colors.text_primary)
                )
                .on_press(Message::CloseTabsToRight(state.tab_index))
                .style(button_secondary_style_dynamic(&app.settings.theme))
                .width(Length::Fill)
                .padding([4.0 * scale, 8.0 * scale])
            );

            menu_items = menu_items.push(
                button(
                    text("Close Other Tabs")
                        .size(base_font_size * 0.9)
                        .font(current_font)
                        .color(theme_colors.text_primary)
                )
                .on_press(Message::CloseOtherTabs(state.tab_index))
                .style(button_secondary_style_dynamic(&app.settings.theme))
                .width(Length::Fill)
                .padding([4.0 * scale, 8.0 * scale])
            );

            let group_name = match &tab.group {
                TabGroup::Default => "Default",
                TabGroup::Research => "Research", 
                TabGroup::Library => "Library",
                TabGroup::Downloads => "Downloads",
                TabGroup::Custom(_) => "Custom",
            };

            menu_items = menu_items.push(
                button(
                    text(format!("Close {} Tabs", group_name))
                        .size(base_font_size * 0.9)
                        .font(current_font)
                        .color(theme_colors.text_primary)
                )
                .on_press(Message::CloseTabsInGroup(tab.group.clone()))
                .style(button_secondary_style_dynamic(&app.settings.theme))
                .width(Length::Fill)
                .padding([4.0 * scale, 8.0 * scale])
            );
        }

        // Create context menu container
        let menu_container = container(menu_items)
            .padding([4.0 * scale, 4.0 * scale])
            .style(move |_theme: &iced::Theme| {
                container::Style {
                    background: Some(Background::Color(theme_colors.dark_bg)),
                    border: Border {
                        color: theme_colors.border_color,
                        width: 1.0,
                        radius: (4.0 * scale).into(),
                    },
                    shadow: iced::Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                        offset: iced::Vector::new(2.0, 2.0),
                        blur_radius: 8.0,
                    },
                    ..Default::default()
                }
            })
            .width(200.0 * scale);

        // Create transparent background overlay, hide menu when clicked
        button(
            container(menu_container)
                .style(|_theme: &iced::Theme| container::Style::default())
                .padding(iced::Padding::new(state.x).top(state.y))
        )
        .on_press(Message::HideContextMenu)
        .style(move |_theme: &iced::Theme, _status| {
            button::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                ..Default::default()
            }
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

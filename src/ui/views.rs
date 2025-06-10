// 用户界面视图组件

use iced::widget::{
    button, column, container, row, text, text_input, scrollable, progress_bar, 
    pane_grid, horizontal_rule, vertical_space, horizontal_space
};
use iced::{Element, Length, Color, Background, Border, Shadow};

use crate::core::app_state::ArxivManager;
use crate::core::models::{ArxivPaper, DownloadItem, DownloadStatus, PaneType};
use crate::core::messages::Message;
use crate::ui::style::{button_primary_style, button_secondary_style, button_danger_style};
use crate::ui::theme::*;

impl ArxivManager {
    pub fn view(&self) -> Element<Message> {
        let sidebar = if self.sidebar_visible {
            Some(self.sidebar())
        } else {
            None
        };

        let pane_grid = pane_grid::PaneGrid::new(&self.panes, |_pane, pane_data, _is_maximized| {
            let title_bar = pane_grid::TitleBar::new(text(&pane_data.title).color(GRUVBOX_TEXT))
                .controls(pane_grid::Controls::new(text("×")));
            
            let content = match pane_data.pane_type {
                PaneType::Search => self.search_view(),
                PaneType::Library => self.library_view(),
                PaneType::Downloads => self.downloads_view(),
                PaneType::Settings => self.settings_view(),
                PaneType::PaperView(index) => {
                    if let Some(paper) = self.saved_papers.get(index) {
                        self.paper_view(paper)
                    } else {
                        container(text("Paper not found").color(GRUVBOX_TEXT)).into()
                    }
                }
            };

            pane_grid::Content::new(content)
                .title_bar(title_bar)
        })
        .on_click(Message::PaneClicked)
        .on_resize(10, Message::PaneResized)
        .on_drag(Message::PaneDragged)
        .spacing(4);

        let main_content = container(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| iced::widget::container::Style {
                background: Some(Background::Color(GRUVBOX_BG)),
                border: Border::default(),
                text_color: Some(GRUVBOX_TEXT),
                shadow: Shadow::default(),
            });

        let base_layout = if let Some(sidebar) = sidebar {
            row![sidebar, main_content]
                .spacing(0)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        } else {
            main_content.into()
        };

        // 如果命令栏可见，添加覆盖层
        if self.command_palette_visible {
            iced::widget::stack![
                base_layout,
                self.command_palette_overlay()
            ].into()
        } else {
            base_layout
        }
    }

    pub fn sidebar(&self) -> Element<Message> {
        let toggle_button = button(text("☰").color(GRUVBOX_TEXT))
            .on_press(Message::SidebarToggled)
            .style(button_secondary_style);

        // 导航按钮
        let navigation_buttons = column![
            button(text("🔍 Search").color(GRUVBOX_TEXT))
                .on_press(Message::OpenSearchPane)
                .width(Length::Fill)
                .style(button_secondary_style),
            button(text("📚 Library").color(GRUVBOX_TEXT))
                .on_press(Message::OpenLibraryPane)
                .width(Length::Fill)
                .style(button_secondary_style),
            button(text("📥 Downloads").color(GRUVBOX_TEXT))
                .on_press(Message::OpenDownloadsPane)
                .width(Length::Fill)
                .style(button_secondary_style),
            button(text("⚙️ Settings").color(GRUVBOX_TEXT))
                .on_press(Message::OpenSettingsPane)
                .width(Length::Fill)
                .style(button_secondary_style),
        ]
        .spacing(8);

        let pane_controls = column![
            button(text("Split Horizontal").color(GRUVBOX_TEXT))
                .on_press(Message::SplitHorizontal)
                .style(button_secondary_style),
            button(text("Split Vertical").color(GRUVBOX_TEXT))
                .on_press(Message::SplitVertical)
                .style(button_secondary_style),
            button(text("Close Pane").color(Color::WHITE))
                .on_press(Message::CloseFocusedPane)
                .style(button_danger_style),
        ]
        .spacing(8);

        let saved_papers_list = scrollable(
            column(
                self.saved_papers.iter().map(|paper| {
                    button(text(&paper.title).color(GRUVBOX_TEXT))
                        .on_press(Message::OpenPaperPane(paper.clone()))
                        .width(Length::Fill)
                        .style(button_secondary_style)
                        .into()
                }).collect::<Vec<Element<Message>>>()
            ).spacing(4)
        );

        container(
            column![
                toggle_button,
                horizontal_rule(2),
                text("Pane Controls").color(GRUVBOX_TEXT).size(16),
                pane_controls,
                horizontal_rule(2),
                text("Saved Papers").color(GRUVBOX_TEXT).size(16),
                saved_papers_list,
                horizontal_rule(2),
                text("Navigation").color(GRUVBOX_TEXT).size(16),
                navigation_buttons,
            ]
            .spacing(16)
            .padding(16)
        )
        .width(280)
        .height(Length::Fill)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_SURFACE)),
            border: Border {
                color: GRUVBOX_BORDER,
                width: 1.0,
                radius: 0.0.into(),
            },
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    pub fn search_view(&self) -> Element<Message> {
        let search_input = text_input("Search arXiv papers...", &self.search_query)
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
            text(if self.advanced_search_visible { "Hide Advanced" } else { "Advanced" })
                .color(GRUVBOX_TEXT)
        )
        .on_press(Message::AdvancedSearchToggled)
        .style(button_secondary_style);

        let search_bar = row![search_input, search_button, advanced_toggle]
            .spacing(10)
            .padding(10);

        let mut main_content = vec![search_bar.into()];

        // 高级搜索面板
        if self.advanced_search_visible {
            main_content.push(self.advanced_search_panel());
        }

        main_content.push(horizontal_rule(1).into());

        let results_content = if self.is_searching {
            column![text("Searching...").color(GRUVBOX_TEXT)]
        } else if let Some(error) = &self.search_error {
            column![
                text("Error:").color(GRUVBOX_RED),
                text(error).color(GRUVBOX_TEXT)
            ]
        } else if self.search_results.is_empty() {
            column![text("No results").color(GRUVBOX_TEXT_MUTED)]
        } else {
            column(
                self.search_results.iter().map(|paper| {
                    self.paper_card(paper, false)
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

    fn advanced_search_panel(&self) -> Element<Message> {
        use iced::widget::{pick_list, checkbox};
        use crate::core::models::{SearchField, SortBy, SortOrder, ARXIV_CATEGORIES};

        // 搜索字段选择
        let search_field_list = pick_list(
            SearchField::all_variants(),
            Some(self.search_config.search_in.clone()),
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
            Some(self.search_config.sort_by.clone()),
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
            Some(self.search_config.sort_order.clone()),
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

        // 分类复选框 - 按学科分组
        let physics_categories = ARXIV_CATEGORIES.iter()
            .filter(|(code, _)| code.starts_with("hep-") || code.starts_with("gr-qc") || 
                    code.starts_with("quant-ph") || code.starts_with("nucl-") || 
                    code.starts_with("math-ph") || code.starts_with("nlin.") ||
                    code.starts_with("physics.") || code.starts_with("astro-ph.") ||
                    code.starts_with("cond-mat."))
            .map(|(code, name)| {
                let is_checked = self.search_config.categories.contains(&code.to_string());
                checkbox(format!("{} - {}", code, name), is_checked)
                    .on_toggle(|_| Message::CategoryToggled(code.to_string()))
                    .style(|_theme, _status| iced::widget::checkbox::Style {
                        background: Background::Color(GRUVBOX_SURFACE),
                        icon_color: GRUVBOX_GREEN,
                        border: Border {
                            color: GRUVBOX_BORDER,
                            width: 1.0,
                            radius: 2.0.into(),
                        },
                        text_color: Some(GRUVBOX_TEXT),
                    })
                    .into()
            }).collect::<Vec<Element<Message>>>();

        let math_categories = ARXIV_CATEGORIES.iter()
            .filter(|(code, _)| code.starts_with("math."))
            .map(|(code, name)| {
                let is_checked = self.search_config.categories.contains(&code.to_string());
                checkbox(format!("{} - {}", code, name), is_checked)
                    .on_toggle(|_| Message::CategoryToggled(code.to_string()))
                    .style(|_theme, _status| iced::widget::checkbox::Style {
                        background: Background::Color(GRUVBOX_SURFACE),
                        icon_color: GRUVBOX_BLUE,
                        border: Border {
                            color: GRUVBOX_BORDER,
                            width: 1.0,
                            radius: 2.0.into(),
                        },
                        text_color: Some(GRUVBOX_TEXT),
                    })
                    .into()
            }).collect::<Vec<Element<Message>>>();

        let other_categories = ARXIV_CATEGORIES.iter()
            .filter(|(code, _)| code.starts_with("cs.") || code.starts_with("stat.") || 
                    code.starts_with("q-bio.") || code.starts_with("q-fin.") || 
                    code.starts_with("econ."))
            .take(15) // 限制显示数量
            .map(|(code, name)| {
                let is_checked = self.search_config.categories.contains(&code.to_string());
                checkbox(format!("{} - {}", code, name), is_checked)
                    .on_toggle(|_| Message::CategoryToggled(code.to_string()))
                    .style(|_theme, _status| iced::widget::checkbox::Style {
                        background: Background::Color(GRUVBOX_SURFACE),
                        icon_color: GRUVBOX_ORANGE,
                        border: Border {
                            color: GRUVBOX_BORDER,
                            width: 1.0,
                            radius: 2.0.into(),
                        },
                        text_color: Some(GRUVBOX_TEXT),
                    })
                    .into()
            }).collect::<Vec<Element<Message>>>();

        let categories_section = column![
            text("Physics & Astronomy").color(GRUVBOX_GREEN).size(16).width(Length::Fill),
            scrollable(column(physics_categories).spacing(4)).height(200),
            vertical_space().height(10),
            text("Mathematics").color(GRUVBOX_BLUE).size(16).width(Length::Fill),
            scrollable(column(math_categories).spacing(4)).height(150),
            vertical_space().height(10),
            text("Other Fields").color(GRUVBOX_ORANGE).size(16).width(Length::Fill),
            scrollable(column(other_categories).spacing(4)).height(100),
        ].spacing(8);

        // 最大结果数
        let max_results_input = text_input(
            "Max results (1-100)", 
            &self.search_config.max_results.to_string()
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
                vertical_space().height(10),
                text("Categories:").color(GRUVBOX_TEXT).size(14),
                categories_section
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

    pub fn library_view(&self) -> Element<Message> {
        let content = if self.saved_papers.is_empty() {
            column![text("No saved papers").color(GRUVBOX_TEXT_MUTED)]
        } else {
            column(
                self.saved_papers.iter().map(|paper| {
                    self.paper_card(paper, true)
                }).collect::<Vec<Element<Message>>>()
            ).spacing(10)
        };

        container(
            scrollable(content).height(Length::Fill)
        )
        .padding(20)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_BG)),
            border: Border::default(),
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    pub fn downloads_view(&self) -> Element<Message> {
        let content = if self.downloads.is_empty() {
            column![text("No downloads").color(GRUVBOX_TEXT_MUTED)]
        } else {
            column(
                self.downloads.iter().map(|download| {
                    self.download_card(download)
                }).collect::<Vec<Element<Message>>>()
            ).spacing(10)
        };

        container(
            scrollable(content).height(Length::Fill)
        )
        .padding(20)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_BG)),
            border: Border::default(),
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    pub fn settings_view(&self) -> Element<Message> {
        use iced::widget::{pick_list, checkbox, text_input};
        use crate::core::models::{Theme, Language, SearchField, SortBy, SortOrder};

        let title = text("Settings")
            .color(GRUVBOX_TEXT)
            .size(28);

        // 外观设置
        let appearance_section = self.create_settings_section(
            "Appearance",
            GRUVBOX_BLUE,
            vec![
                self.create_setting_row(
                    "Theme:",
                    pick_list(
                        Theme::all_variants(),
                        Some(self.settings.theme.clone()),
                        Message::ThemeChanged,
                    )
                    .placeholder("Select theme...")
                    .style(self.pick_list_style())
                    .into()
                ),
                self.create_setting_row(
                    "Language:",
                    pick_list(
                        Language::all_variants(),
                        Some(self.settings.language.clone()),
                        Message::LanguageChanged,
                    )
                    .placeholder("Select language...")
                    .style(self.pick_list_style())
                    .into()
                ),
            ]
        );

        // 下载设置
        let download_section = self.create_settings_section(
            "Downloads",
            GRUVBOX_GREEN,
            vec![
                self.create_setting_row(
                    "Download Directory:",
                    text_input("Path to download directory", &self.settings.download_directory)
                        .on_input(Message::DownloadDirectoryChanged)
                        .style(self.text_input_style())
                        .into()
                ),
                self.create_setting_row(
                    "Auto Download:",
                    checkbox("Automatically download papers when saved", self.settings.auto_download)
                        .on_toggle(|_| Message::AutoDownloadToggled)
                        .style(self.checkbox_style())
                        .into()
                ),
                self.create_setting_row(
                    "Max Concurrent Downloads:",
                    text_input("1-10", &self.settings.max_concurrent_downloads.to_string())
                        .on_input(Message::MaxConcurrentDownloadsChanged)
                        .style(self.text_input_style())
                        .into()
                ),
            ]
        );

        // 搜索设置
        let search_section = self.create_settings_section(
            "Search",
            GRUVBOX_ORANGE,
            vec![
                self.create_setting_row(
                    "Show Abstracts in Results:",
                    checkbox("Display paper abstracts in search results", self.settings.show_abstracts_in_search)
                        .on_toggle(|_| Message::ShowAbstractsToggled)
                        .style(self.checkbox_style())
                        .into()
                ),
                self.create_setting_row(
                    "Default Search Field:",
                    pick_list(
                        SearchField::all_variants(),
                        Some(self.settings.default_search_field.clone()),
                        Message::DefaultSearchFieldChanged,
                    )
                    .placeholder("Select search field...")
                    .style(self.pick_list_style())
                    .into()
                ),
                self.create_setting_row(
                    "Default Sort By:",
                    pick_list(
                        SortBy::all_variants(),
                        Some(self.settings.default_sort_by.clone()),
                        Message::DefaultSortByChanged,
                    )
                    .placeholder("Select sort by...")
                    .style(self.pick_list_style())
                    .into()
                ),
                self.create_setting_row(
                    "Default Sort Order:",
                    pick_list(
                        SortOrder::all_variants(),
                        Some(self.settings.default_sort_order.clone()),
                        Message::DefaultSortOrderChanged,
                    )
                    .placeholder("Select sort order...")
                    .style(self.pick_list_style())
                    .into()
                ),
                self.create_setting_row(
                    "Default Max Results:",
                    text_input("1-100", &self.settings.default_max_results.to_string())
                        .on_input(Message::DefaultMaxResultsChanged)
                        .style(self.text_input_style())
                        .into()
                ),
                self.create_setting_row(
                    "Auto Save Searches:",
                    checkbox("Automatically save search queries", self.settings.auto_save_searches)
                        .on_toggle(|_| Message::AutoSaveSearchesToggled)
                        .style(self.checkbox_style())
                        .into()
                ),
            ]
        );

        // 通知设置
        let notification_section = self.create_settings_section(
            "Notifications",
            GRUVBOX_PURPLE,
            vec![
                self.create_setting_row(
                    "Enable Notifications:",
                    checkbox("Show notifications for downloads and updates", self.settings.notification_enabled)
                        .on_toggle(|_| Message::NotificationToggled)
                        .style(self.checkbox_style())
                        .into()
                ),
                self.create_setting_row(
                    "Check for Updates:",
                    checkbox("Automatically check for application updates", self.settings.check_updates)
                        .on_toggle(|_| Message::CheckUpdatesToggled)
                        .style(self.checkbox_style())
                        .into()
                ),
            ]
        );

        // 快捷键设置
        let shortcuts_section = self.create_settings_section(
            "Keyboard Shortcuts",
            GRUVBOX_ORANGE,
            vec![
                self.create_shortcut_setting("Toggle Command Palette:", "toggle_command_palette", &self.settings.shortcuts.toggle_command_palette.display),
                self.create_shortcut_setting("Focus Search:", "focus_search", &self.settings.shortcuts.focus_search.display),
                self.create_shortcut_setting("Quick Save Paper:", "quick_save_paper", &self.settings.shortcuts.quick_save_paper.display),
                self.create_shortcut_setting("Quick Download Paper:", "quick_download_paper", &self.settings.shortcuts.quick_download_paper.display),
                self.create_shortcut_setting("Toggle Sidebar:", "toggle_sidebar", &self.settings.shortcuts.toggle_sidebar.display),
                self.create_shortcut_setting("Next Pane:", "next_pane", &self.settings.shortcuts.next_pane.display),
                self.create_shortcut_setting("Previous Pane:", "previous_pane", &self.settings.shortcuts.previous_pane.display),
                self.create_shortcut_setting("Close Pane:", "close_pane", &self.settings.shortcuts.close_pane.display),
                self.create_shortcut_setting("Split Horizontal:", "split_horizontal", &self.settings.shortcuts.split_horizontal.display),
                self.create_shortcut_setting("Split Vertical:", "split_vertical", &self.settings.shortcuts.split_vertical.display),
                self.create_shortcut_setting("Go to Search:", "go_to_search", &self.settings.shortcuts.go_to_search.display),
                self.create_shortcut_setting("Go to Library:", "go_to_library", &self.settings.shortcuts.go_to_library.display),
                self.create_shortcut_setting("Go to Downloads:", "go_to_downloads", &self.settings.shortcuts.go_to_downloads.display),
                self.create_shortcut_setting("Go to Settings:", "go_to_settings", &self.settings.shortcuts.go_to_settings.display),
                row![
                    button(text("Reset All Shortcuts").color(Color::WHITE))
                        .on_press(Message::ResetShortcuts)
                        .style(button_danger_style),
                    horizontal_space(),
                    text("Reset all shortcuts to default values")
                        .color(GRUVBOX_TEXT_MUTED)
                        .size(12),
                ].spacing(10).into()
            ]
        );

        // 操作按钮
        let action_buttons = row![
            button(text("Reset to Defaults").color(Color::WHITE))
                .on_press(Message::ResetSettings)
                .style(button_danger_style),
            horizontal_space().width(10),
            button(text("Export Settings").color(GRUVBOX_TEXT))
                .on_press(Message::ExportSettings)
                .style(button_secondary_style),
            horizontal_space().width(10),
            button(text("Import Settings").color(GRUVBOX_TEXT))
                .on_press(Message::ImportSettings)
                .style(button_secondary_style),
        ].spacing(10);

        container(
            scrollable(
                column![
                    title,
                    vertical_space().height(20),
                    appearance_section,
                    vertical_space().height(15),
                    download_section,
                    vertical_space().height(15),
                    search_section,
                    vertical_space().height(15),
                    notification_section,
                    vertical_space().height(15),
                    shortcuts_section,
                    vertical_space().height(25),
                    action_buttons,
                ].spacing(10)
            )
        )
        .padding(20)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_BG)),
            border: Border::default(),
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    fn create_settings_section<'a>(&self, title: &'a str, color: Color, items: Vec<Element<'a, Message>>) -> Element<'a, Message> {
        container(
            column![
                text(title).color(color).size(20),
                vertical_space().height(10),
                column(items).spacing(15)
            ].spacing(5)
        )
        .padding(15)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_SURFACE)),
            border: Border {
                color: color,
                width: 1.0,
                radius: 8.0.into(),
            },
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    fn create_setting_row<'a>(&self, label: &'a str, control: Element<'a, Message>) -> Element<'a, Message> {
        row![
            text(label)
                .color(GRUVBOX_TEXT)
                .size(14)
                .width(Length::FillPortion(2)),
            container(control).width(Length::FillPortion(3))
        ]
        .spacing(15)
        .align_y(iced::Alignment::Center)
        .into()
    }

    fn pick_list_style(&self) -> impl Fn(&iced::Theme, iced::widget::pick_list::Status) -> iced::widget::pick_list::Style {
        |_theme, status| iced::widget::pick_list::Style {
            text_color: GRUVBOX_TEXT,
            background: Background::Color(GRUVBOX_BG),
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
        }
    }

    fn text_input_style(&self) -> impl Fn(&iced::Theme, iced::widget::text_input::Status) -> iced::widget::text_input::Style {
        |_theme, status| iced::widget::text_input::Style {
            background: Background::Color(GRUVBOX_BG),
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
        }
    }

    fn checkbox_style(&self) -> impl Fn(&iced::Theme, iced::widget::checkbox::Status) -> iced::widget::checkbox::Style {
        |_theme, _status| iced::widget::checkbox::Style {
            background: Background::Color(GRUVBOX_BG),
            icon_color: GRUVBOX_GREEN,
            border: Border {
                color: GRUVBOX_BORDER,
                width: 1.0,
                radius: 2.0.into(),
            },
            text_color: Some(GRUVBOX_TEXT),
        }
    }

    fn create_shortcut_setting<'a>(&self, label: &'a str, action: &'a str, current_shortcut: &'a str) -> Element<'a, Message> {
        use iced::widget::{text_input, button, text};
        
        let shortcut_input = text_input("Enter shortcut...", current_shortcut)
            .on_input(move |new_shortcut| Message::ShortcutChanged { 
                action: action.to_string(), 
                shortcut: new_shortcut 
            })
            .size(14)
            .width(Length::Fixed(150.0))
            .style(self.text_input_style());

        let reset_button = button(text("Reset").size(12))
            .on_press(Message::ShortcutChanged { 
                action: action.to_string(), 
                shortcut: self.get_default_shortcut(action) 
            })
            .style(button_secondary_style);

        self.create_setting_row(
            label,
            row![
                shortcut_input,
                horizontal_space().width(8),
                reset_button,
            ]
            .align_y(iced::Alignment::Center)
            .into()
        )
    }

    fn get_default_shortcut(&self, action: &str) -> String {
        use crate::core::models::KeyboardShortcuts;
        let defaults = KeyboardShortcuts::default();
        
        match action {
            "toggle_command_palette" => defaults.toggle_command_palette.display,
            "focus_search" => defaults.focus_search.display,
            "quick_save_paper" => defaults.quick_save_paper.display,
            "quick_download_paper" => defaults.quick_download_paper.display,
            "toggle_sidebar" => defaults.toggle_sidebar.display,
            "next_pane" => defaults.next_pane.display,
            "previous_pane" => defaults.previous_pane.display,
            "close_pane" => defaults.close_pane.display,
            "split_horizontal" => defaults.split_horizontal.display,
            "split_vertical" => defaults.split_vertical.display,
            "go_to_search" => defaults.go_to_search.display,
            "go_to_library" => defaults.go_to_library.display,
            "go_to_downloads" => defaults.go_to_downloads.display,
            "go_to_settings" => defaults.go_to_settings.display,
            _ => "".to_string(),
        }
    }

    pub fn paper_view<'a>(&'a self, paper: &'a ArxivPaper) -> Element<'a, Message> {
        let title = text(&paper.title)
            .color(GRUVBOX_TEXT)
            .size(24);

        let authors = text(paper.authors.join(", "))
            .color(GRUVBOX_TEXT_MUTED)
            .size(14);

        let published = text(format!("Published: {}", paper.published))
            .color(GRUVBOX_TEXT_MUTED)
            .size(12);

        let categories = text(format!("Categories: {}", paper.categories.join(", ")))
            .color(GRUVBOX_TEXT_MUTED)
            .size(12);

        let abstract_text = text(&paper.abstract_text)
            .color(GRUVBOX_TEXT)
            .size(14);

        container(
            scrollable(
                column![
                    title,
                    authors,
                    published,
                    categories,
                    horizontal_rule(1),
                    text("Abstract").color(GRUVBOX_TEXT).size(18),
                    abstract_text,
                ]
                .spacing(10)
            )
        )
        .padding(20)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_BG)),
            border: Border::default(),
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    pub fn paper_card<'a>(&'a self, paper: &'a ArxivPaper, is_saved: bool) -> Element<'a, Message> {
        let title = text(&paper.title)
            .color(GRUVBOX_TEXT)
            .size(16);

        let authors = text(paper.authors.join(", "))
            .color(GRUVBOX_TEXT_MUTED)
            .size(12);

        let buttons = if is_saved {
            row![
                button(text("Remove").color(Color::WHITE))
                    .on_press(Message::RemovePaper(paper.id.clone()))
                    .style(button_danger_style),
                button(text("Download").color(Color::BLACK))
                    .on_press(Message::DownloadPaper(paper.clone()))
                    .style(button_primary_style),
                button(text("View").color(GRUVBOX_TEXT))
                    .on_press(Message::OpenPaperPane(paper.clone()))
                    .style(button_secondary_style),
            ]
        } else {
            row![
                button(text("Save").color(Color::BLACK))
                    .on_press(Message::SavePaper(paper.clone()))
                    .style(button_primary_style),
                button(text("Download").color(GRUVBOX_TEXT))
                    .on_press(Message::DownloadPaper(paper.clone()))
                    .style(button_secondary_style),
            ]
        }
        .spacing(8);

        container(
            column![
                title,
                authors,
                vertical_space().height(8),
                buttons,
            ]
            .spacing(4)
        )
        .padding(12)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_SURFACE)),
            border: Border {
                color: GRUVBOX_BORDER,
                width: 1.0,
                radius: 8.0.into(),
            },
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow::default(),
        })
        .into()
    }

    pub fn download_card<'a>(&'a self, download: &'a DownloadItem) -> Element<'a, Message> {
        let title = text(&download.title)
            .color(GRUVBOX_TEXT)
            .size(14);

        let status_text = match &download.status {
            DownloadStatus::Pending => "Pending",
            DownloadStatus::Downloading => "Downloading",
            DownloadStatus::Completed => "Completed",
            DownloadStatus::Failed(_) => "Failed",
        };

        let status = text(status_text)
            .color(match download.status {
                DownloadStatus::Failed(_) => GRUVBOX_RED,
                DownloadStatus::Completed => GRUVBOX_GREEN,
                _ => GRUVBOX_TEXT_MUTED,
            })
            .size(12);

        let progress = if matches!(download.status, DownloadStatus::Downloading) {
            Some(progress_bar(0.0..=100.0, download.progress))
        } else {
            None
        };

        let mut content = column![title, status].spacing(4);
        
        if let Some(progress_bar) = progress {
            content = content.push(progress_bar);
        }

        container(content)
            .padding(12)
            .style(|_theme| iced::widget::container::Style {
                background: Some(Background::Color(GRUVBOX_SURFACE)),
                border: Border {
                    color: GRUVBOX_BORDER,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                text_color: Some(GRUVBOX_TEXT),
                shadow: Shadow::default(),
            })
            .into()
    }

    // 命令栏覆盖层
    pub fn command_palette_overlay(&self) -> Element<Message> {
        use iced::widget::{container, column, row, text, text_input, scrollable};
        
        // 命令栏主体
        let command_input = text_input("Type a command...", &self.command_palette_input)
            .on_input(Message::CommandPaletteInputChanged)
            .on_submit(if let Some(index) = self.selected_command_index {
                if let Some(command) = self.command_suggestions.get(index) {
                    Message::ExecuteCommand(command.clone())
                } else {
                    Message::ClearCommandPalette
                }
            } else {
                Message::ClearCommandPalette
            })
            .size(18)
            .style(|_theme, status| iced::widget::text_input::Style {
                background: Background::Color(GRUVBOX_SURFACE),
                border: Border {
                    color: match status {
                        iced::widget::text_input::Status::Focused => GRUVBOX_GREEN,
                        _ => GRUVBOX_BORDER,
                    },
                    width: 2.0,
                    radius: 6.0.into(),
                },
                icon: Color::TRANSPARENT,
                placeholder: GRUVBOX_TEXT_MUTED,
                value: GRUVBOX_TEXT,
                selection: GRUVBOX_GREEN,
            });

        // 命令建议列表
        let suggestions_list = if !self.command_suggestions.is_empty() {
            let suggestions = column(
                self.command_suggestions.iter().enumerate().map(|(index, command)| {
                    let is_selected = self.selected_command_index == Some(index);
                    
                    let command_button = button(
                        row![
                            text(command.display_name())
                                .color(if is_selected { Color::BLACK } else { GRUVBOX_TEXT })
                                .size(14),
                            horizontal_space(),
                            text(command.keywords().join(" "))
                                .color(if is_selected { Color::from_rgb(0.3, 0.3, 0.3) } else { GRUVBOX_TEXT_MUTED })
                                .size(12),
                        ]
                        .padding([8, 12])
                        .width(Length::Fill)
                    )
                    .on_press(Message::ExecuteCommand(command.clone()))
                    .width(Length::Fill)
                    .style(move |_theme, status| {
                        let base_color = if is_selected {
                            GRUVBOX_GREEN
                        } else {
                            GRUVBOX_SURFACE
                        };
                        
                        iced::widget::button::Style {
                            background: Some(Background::Color(match status {
                                iced::widget::button::Status::Hovered => {
                                    if is_selected {
                                        Color::from_rgb(
                                            GRUVBOX_GREEN.r * 0.9,
                                            GRUVBOX_GREEN.g * 0.9,
                                            GRUVBOX_GREEN.b * 0.9,
                                        )
                                    } else {
                                        Color::from_rgb(
                                            GRUVBOX_SURFACE.r * 1.1,
                                            GRUVBOX_SURFACE.g * 1.1,
                                            GRUVBOX_SURFACE.b * 1.1,
                                        )
                                    }
                                }
                                _ => base_color,
                            })),
                            text_color: if is_selected { Color::BLACK } else { GRUVBOX_TEXT },
                            border: Border {
                                color: if is_selected { GRUVBOX_GREEN } else { Color::TRANSPARENT },
                                width: if is_selected { 1.0 } else { 0.0 },
                                radius: 4.0.into(),
                            },
                            shadow: Shadow::default(),
                        }
                    });

                    command_button.into()
                }).collect::<Vec<Element<Message>>>()
            )
            .spacing(2);

            scrollable(suggestions)
                .height(Length::Fixed(300.0))
        } else {
            scrollable(
                container(
                    text("No commands found")
                        .color(GRUVBOX_TEXT_MUTED)
                        .size(14)
                )
                .padding(20)
                .center_x(Length::Fill)
            )
            .height(Length::Fixed(60.0))
        };

        let command_palette = container(
            column![
                command_input,
                vertical_space().height(8),
                suggestions_list
            ]
            .spacing(0)
        )
        .padding(20)
        .width(Length::Fixed(600.0))
        .max_height(400.0)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(GRUVBOX_BG)),
            border: Border {
                color: GRUVBOX_BORDER,
                width: 2.0,
                radius: 12.0.into(),
            },
            text_color: Some(GRUVBOX_TEXT),
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 20.0,
            },
        });

        // 将命令栏居中显示，添加半透明背景
        container(
            container(command_palette)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .width(Length::Fill)
                .height(Length::Fill)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.5))),
            border: Border::default(),
            text_color: None,
            shadow: Shadow::default(),
        })
        .into()
    }
}

use iced::{Background, Color, border, theme};

pub mod container;
pub mod button;
pub mod text;
pub mod pane_grid;
pub mod scrollable;

pub const TEXT_SIZE: f32 = 13.0;
pub const ICON_SIZE: f32 = 12.0;

#[derive(Debug, Clone)]
pub struct Theme {
    pub colors: Colors,
}

#[derive(Debug, Clone)]
pub struct Colors {
    pub general: General,
    pub text: Text,
    pub buttons: Buttons,
}

#[derive(Debug, Clone)]
pub struct General {
    pub background: Color,
    pub surface: Color,
    pub border: Color,
    pub horizontal_rule: Color,
    pub unread_indicator: Color,
}

#[derive(Debug, Clone)]
pub struct Text {
    pub primary: Color,
    pub secondary: Color,
    pub success: Color,
    pub error: Color,
}

#[derive(Debug, Clone)]
pub struct Buttons {
    pub primary: Button,
    pub secondary: Button,
}

#[derive(Debug, Clone)]
pub struct Button {
    pub background: Color,
    pub background_hover: Color,
    pub text: Color,
    pub border: Color,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            colors: Colors::gruvbox_dark(),
        }
    }

    // 添加所有需要的样式方法
    pub fn container(&self) -> impl Fn(&iced::Theme) -> iced::widget::container::Style {
        let colors = self.colors.clone();
        move |_theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(colors.general.background)),
            border: iced::Border::default(),
            text_color: Some(colors.text.primary),
            shadow: iced::Shadow::default(),
        }
    }

    pub fn sidebar(&self) -> impl Fn(&iced::Theme) -> iced::widget::container::Style {
        let colors = self.colors.clone();
        move |_theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(colors.general.surface)),
            border: iced::Border {
                color: colors.general.border,
                width: 1.0,
                radius: 0.0.into(),
            },
            text_color: Some(colors.text.primary),
            shadow: iced::Shadow::default(),
        }
    }

    pub fn card(&self) -> impl Fn(&iced::Theme) -> iced::widget::container::Style {
        let colors = self.colors.clone();
        move |_theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(colors.general.surface)),
            border: iced::Border {
                color: colors.general.border,
                width: 1.0,
                radius: 8.0.into(),
            },
            text_color: Some(colors.text.primary),
            shadow: iced::Shadow::default(),
        }
    }

    pub fn button_primary(&self) -> impl Fn(&iced::Theme, iced::widget::button::Status) -> iced::widget::button::Style {
        let colors = self.colors.clone();
        move |_theme, status| {
            let (background, text_color) = match status {
                iced::widget::button::Status::Hovered => (colors.buttons.primary.background_hover, colors.buttons.primary.text),
                _ => (colors.buttons.primary.background, colors.buttons.primary.text),
            };
            
            iced::widget::button::Style {
                background: Some(iced::Background::Color(background)),
                text_color,
                border: iced::Border {
                    color: colors.buttons.primary.border,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                shadow: iced::Shadow::default(),
            }
        }
    }

    pub fn button_secondary(&self) -> impl Fn(&iced::Theme, iced::widget::button::Status) -> iced::widget::button::Style {
        let colors = self.colors.clone();
        move |_theme, status| {
            let (background, text_color) = match status {
                iced::widget::button::Status::Hovered => (colors.buttons.secondary.background_hover, colors.buttons.secondary.text),
                _ => (colors.buttons.secondary.background, colors.buttons.secondary.text),
            };
            
            iced::widget::button::Style {
                background: Some(iced::Background::Color(background)),
                text_color,
                border: iced::Border {
                    color: colors.buttons.secondary.border,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                shadow: iced::Shadow::default(),
            }
        }
    }

    pub fn button_danger(&self) -> impl Fn(&iced::Theme, iced::widget::button::Status) -> iced::widget::button::Style {
        let colors = self.colors.clone();
        move |_theme, status| {
            let background = match status {
                iced::widget::button::Status::Hovered => Color::from_rgb(0.85, 0.3, 0.3),
                _ => colors.text.error,
            };
            
            iced::widget::button::Style {
                background: Some(iced::Background::Color(background)),
                text_color: Color::WHITE,
                border: iced::Border {
                    color: colors.text.error,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                shadow: iced::Shadow::default(),
            }
        }
    }

    pub fn text_input(&self) -> impl Fn(&iced::Theme, iced::widget::text_input::Status) -> iced::widget::text_input::Style {
        let colors = self.colors.clone();
        move |_theme, status| iced::widget::text_input::Style {
            background: iced::Background::Color(colors.general.surface),
            border: iced::Border {
                color: match status {
                    iced::widget::text_input::Status::Focused => colors.text.success,
                    _ => colors.general.border,
                },
                width: 1.0,
                radius: 4.0.into(),
            },
            icon: Color::TRANSPARENT,
            placeholder: colors.text.secondary,
            value: colors.text.primary,
            selection: colors.text.success,
        }
    }

    pub fn pane_grid(&self) -> impl Fn(&iced::Theme, iced::widget::pane_grid::Status) -> iced::widget::pane_grid::Style {
        let colors = self.colors.clone();
        move |_theme, _status| iced::widget::pane_grid::Style {
            hovered_region: iced::Background::Color(colors.general.surface),
            picked_split: colors.text.success,
            hovered_split: colors.text.error,
        }
    }

    pub fn pane_grid_title_bar(&self) -> impl Fn(&iced::Theme) -> iced::widget::pane_grid::TitleBarStyle {
        let colors = self.colors.clone();
        move |_theme| iced::widget::pane_grid::TitleBarStyle {
            background: iced::Background::Color(colors.general.surface),
            border: iced::Border {
                color: colors.general.border,
                width: 1.0,
                radius: 0.0.into(),
            },
        }
    }

    pub fn text_heading(&self) -> Color {
        self.colors.text.primary
    }

    pub fn text_body(&self) -> Color {
        self.colors.text.primary
    }

    pub fn text_muted(&self) -> Color {
        self.colors.text.secondary
    }

    pub fn text_danger(&self) -> Color {
        self.colors.text.error
    }

    pub fn text_success(&self) -> Color {
        self.colors.text.success
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new()
    }
}

impl Colors {
    pub fn gruvbox_dark() -> Self {
        Self {
            general: General {
                background: Color::from_rgb(0.16, 0.16, 0.16), // #282828
                surface: Color::from_rgb(0.20, 0.19, 0.17),     // #32302f
                border: Color::from_rgb(0.35, 0.33, 0.29),      // #595959
                horizontal_rule: Color::from_rgb(0.35, 0.33, 0.29),
                unread_indicator: Color::from_rgb(0.80, 0.41, 0.13), // #cc6d19
            },
            text: Text {
                primary: Color::from_rgb(0.92, 0.86, 0.70),     // #ebdbb2
                secondary: Color::from_rgb(0.66, 0.61, 0.52),   // #a89984
                success: Color::from_rgb(0.72, 0.73, 0.15),     // #b8bb26
                error: Color::from_rgb(0.98, 0.38, 0.37),       // #fb4934
            },
            buttons: Buttons {
                primary: Button {
                    background: Color::from_rgb(0.45, 0.62, 0.22),     // #73a322
                    background_hover: Color::from_rgb(0.72, 0.73, 0.15), // #b8bb26
                    text: Color::from_rgb(0.16, 0.16, 0.16),            // #282828
                    border: Color::from_rgb(0.72, 0.73, 0.15),          // #b8bb26
                },
                secondary: Button {
                    background: Color::from_rgb(0.35, 0.33, 0.29),     // #595959
                    background_hover: Color::from_rgb(0.45, 0.41, 0.35), // #7b6e4a
                    text: Color::from_rgb(0.92, 0.86, 0.70),            // #ebdbb2
                    border: Color::from_rgb(0.35, 0.33, 0.29),          // #595959
                },
            },
        }
    }
}

impl theme::Base for Theme {
    fn base(&self) -> theme::Style {
        theme::Style {
            background_color: self.colors.general.background,
            text_color: self.colors.text.primary,
        }
    }

    fn palette(&self) -> Option<theme::Palette> {
        None
    }
}

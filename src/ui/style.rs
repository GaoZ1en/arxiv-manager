use iced::{
    widget::{
        container,
        button as iced_button,
        text_input as iced_text_input,
        scrollable as iced_scrollable,
        progress_bar as iced_progress_bar,
    },
    Background, Color, Theme,
    border,
};

/// Gruvbox 配色方案
pub mod colors {
    use iced::Color;

    // 深色主题
    pub mod dark {
        use super::Color;
        
        pub const BG0_H: Color = Color::from_rgb(0.11, 0.11, 0.11);      // #1d2021
        pub const BG0: Color = Color::from_rgb(0.16, 0.16, 0.16);        // #282828
        pub const BG1: Color = Color::from_rgb(0.20, 0.19, 0.17);        // #3c3836
        pub const BG2: Color = Color::from_rgb(0.25, 0.24, 0.22);        // #504945
        pub const BG3: Color = Color::from_rgb(0.30, 0.28, 0.26);        // #665c54
        pub const BG4: Color = Color::from_rgb(0.35, 0.33, 0.31);        // #7c6f64
        
        pub const FG0: Color = Color::from_rgb(0.98, 0.94, 0.84);        // #fbf1c7
        pub const FG1: Color = Color::from_rgb(0.92, 0.86, 0.70);        // #ebdbb2
        pub const FG2: Color = Color::from_rgb(0.85, 0.77, 0.58);        // #d5c4a1
        pub const FG3: Color = Color::from_rgb(0.67, 0.60, 0.49);        // #bdae93
        pub const FG4: Color = Color::from_rgb(0.49, 0.44, 0.37);        // #a89984
        
        pub const RED: Color = Color::from_rgb(0.80, 0.14, 0.11);        // #cc241d
        pub const GREEN: Color = Color::from_rgb(0.60, 0.59, 0.10);      // #98971a
        pub const YELLOW: Color = Color::from_rgb(0.84, 0.60, 0.13);     // #d79921
        pub const BLUE: Color = Color::from_rgb(0.27, 0.52, 0.53);       // #458588
        pub const PURPLE: Color = Color::from_rgb(0.69, 0.38, 0.56);     // #b16286
        pub const AQUA: Color = Color::from_rgb(0.41, 0.62, 0.42);       // #689d6a
        pub const ORANGE: Color = Color::from_rgb(0.84, 0.41, 0.08);     // #d65d0e
        
        pub const LIGHT1: Color = Color::from_rgb(0.96, 0.91, 0.81);     // #f2e5bc
        pub const LIGHT2: Color = Color::from_rgb(0.93, 0.87, 0.75);     // #f0e0b6
        pub const LIGHT3: Color = Color::from_rgb(0.90, 0.83, 0.69);     // #e6dbb0
        pub const LIGHT4: Color = Color::from_rgb(0.87, 0.79, 0.63);     // #ddd6a3
    }
}

/// 公共颜色枚举
#[derive(Debug, Clone, Copy)]
pub enum GruvboxColors {
    Dark,
    Light1,
    Light2,
    Light3,
    Light4,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Aqua,
    Orange,
}

impl From<GruvboxColors> for Color {
    fn from(color: GruvboxColors) -> Self {
        match color {
            GruvboxColors::Dark => colors::dark::BG0,
            GruvboxColors::Light1 => colors::dark::LIGHT1,
            GruvboxColors::Light2 => colors::dark::LIGHT2,
            GruvboxColors::Light3 => colors::dark::LIGHT3,
            GruvboxColors::Light4 => colors::dark::LIGHT4,
            GruvboxColors::Red => colors::dark::RED,
            GruvboxColors::Green => colors::dark::GREEN,
            GruvboxColors::Yellow => colors::dark::YELLOW,
            GruvboxColors::Blue => colors::dark::BLUE,
            GruvboxColors::Purple => colors::dark::PURPLE,
            GruvboxColors::Aqua => colors::dark::AQUA,
            GruvboxColors::Orange => colors::dark::ORANGE,
        }
    }
}

/// 样式主题结构
pub struct GruvboxStyle;

impl GruvboxStyle {
    pub fn container() -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(colors::dark::BG0)),
            text_color: Some(colors::dark::FG1),
            border_radius: border::Radius::from(0.0),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }

    pub fn content() -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(colors::dark::BG0)),
            text_color: Some(colors::dark::FG1),
            border_radius: border::Radius::from(0.0),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }

    pub fn button() -> iced_button::Appearance {
        iced_button::Appearance {
            background: Some(Background::Color(colors::dark::BG1)),
            text_color: colors::dark::FG1,
            border_radius: border::Radius::from(5.0),
            border_width: 1.0,
            border_color: colors::dark::BG3,
            shadow_offset: iced::Vector::default(),
        }
    }
}

/// 容器样式
pub mod container {
    use super::*;

    pub fn main() -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(colors::dark::BG0)),
            text_color: Some(colors::dark::FG1),
            border_radius: border::Radius::from(0.0),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }

    pub fn sidebar() -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(colors::dark::BG1)),
            text_color: Some(colors::dark::FG1),
            border_radius: border::Radius::from(0.0),
            border_width: 1.0,
            border_color: colors::dark::BG3,
        }
    }

    pub fn content() -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(colors::dark::BG0)),
            text_color: Some(colors::dark::FG1),
            border_radius: border::Radius::from(0.0),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }

    pub fn card() -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(colors::dark::BG1)),
            text_color: Some(colors::dark::FG1),
            border_radius: border::Radius::from(8.0),
            border_width: 1.0,
            border_color: colors::dark::BG2,
        }
    }
}

/// 按钮样式
pub mod button {
    use super::*;

    pub fn primary() -> iced_button::Appearance {
        iced_button::Appearance {
            background: Some(Background::Color(colors::dark::BLUE)),
            text_color: colors::dark::FG0,
            border_radius: border::Radius::from(5.0),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            shadow_offset: iced::Vector::default(),
        }
    }

    pub fn secondary() -> iced_button::Appearance {
        iced_button::Appearance {
            background: Some(Background::Color(colors::dark::BG2)),
            text_color: colors::dark::FG1,
            border_radius: border::Radius::from(5.0),
            border_width: 1.0,
            border_color: colors::dark::BG3,
            shadow_offset: iced::Vector::default(),
        }
    }

    pub fn text() -> iced_button::Appearance {
        iced_button::Appearance {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: colors::dark::FG2,
            border_radius: border::Radius::from(3.0),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            shadow_offset: iced::Vector::default(),
        }
    }

    pub fn sidebar_item() -> iced_button::Appearance {
        iced_button::Appearance {
            background: None,
            text_color: colors::dark::FG2,
            border_radius: border::Radius::from(3.0),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            shadow_offset: iced::Vector::default(),
        }
    }

    pub fn sidebar_item_active() -> iced_button::Appearance {
        iced_button::Appearance {
            background: Some(Background::Color(colors::dark::BG1)),
            text_color: colors::dark::FG0,
            border_radius: border::Radius::from(3.0),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            shadow_offset: iced::Vector::default(),
        }
    }
}

/// 文本样式
pub mod text {
    use super::*;

    pub fn title() -> Color {
        colors::dark::FG0
    }

    pub fn subtitle() -> Color {
        colors::dark::FG1
    }

    pub fn body() -> Color {
        colors::dark::FG2
    }

    pub fn muted() -> Color {
        colors::dark::FG3
    }

    pub fn button() -> Color {
        colors::dark::FG0
    }
}

/// 文本输入样式
pub mod text_input {
    use super::*;

    pub fn default() -> iced_text_input::Appearance {
        iced_text_input::Appearance {
            background: Background::Color(colors::dark::BG1),
            border_radius: border::Radius::from(5.0),
            border_width: 1.0,
            border_color: colors::dark::BG3,
            icon_color: colors::dark::FG2,
        }
    }
}

/// 滚动条样式
pub mod scrollable {
    use super::*;

    pub fn default() -> iced_scrollable::Appearance {
        iced_scrollable::Appearance {
            container: container::Appearance {
                background: None,
                text_color: None,
                border_radius: border::Radius::from(0.0),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            vertical_rail: iced_scrollable::Rail {
                background: Some(Background::Color(colors::dark::BG1)),
                border_radius: border::Radius::from(2.0),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                scroller: iced_scrollable::Scroller {
                    color: colors::dark::BG3,
                    border_radius: border::Radius::from(2.0),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
            },
            horizontal_rail: iced_scrollable::Rail {
                background: Some(Background::Color(colors::dark::BG1)),
                border_radius: border::Radius::from(2.0),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                scroller: iced_scrollable::Scroller {
                    color: colors::dark::BG3,
                    border_radius: border::Radius::from(2.0),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
            },
            gap: None,
        }
    }
}

/// 进度条样式
pub mod progress_bar {
    use super::*;

    pub fn default() -> iced_progress_bar::Appearance {
        iced_progress_bar::Appearance {
            background: Background::Color(colors::dark::BG2),
            bar: Background::Color(colors::dark::BLUE),
            border_radius: border::Radius::from(3.0),
        }
    }
}

// Public style exports for easier access
pub use colors as GruvboxColors;

/// Text styles module
pub mod text {
    use iced::Color;
    use crate::ui::style::colors::dark;

    pub fn title() -> Color {
        dark::FG0
    }

    pub fn subtitle() -> Color {
        dark::FG1
    }

    pub fn body() -> Color {
        dark::FG2
    }

    pub fn muted() -> Color {
        dark::FG3
    }

    pub fn button() -> Color {
        dark::FG0
    }
}

/// Button styles module
pub mod button {
    use iced::widget::button;
    use iced::{Background, Color, border};
    use crate::ui::style::colors::dark;

    pub fn primary() -> button::Style {
        button::Style {
            background: Some(Background::Color(dark::BLUE)),
            text_color: dark::FG0,
            border: border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: border::Radius::from(5.0),
            },
            shadow: iced::Shadow::default(),
        }
    }

    pub fn secondary() -> button::Style {
        button::Style {
            background: Some(Background::Color(dark::BG2)),
            text_color: dark::FG1,
            border: border::Border {
                color: dark::BG3,
                width: 1.0,
                radius: border::Radius::from(5.0),
            },
            shadow: iced::Shadow::default(),
        }
    }

    pub fn text() -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            text_color: dark::FG2,
            border: border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: border::Radius::from(3.0),
            },
            shadow: iced::Shadow::default(),
        }
    }

    pub fn sidebar_item() -> button::Style {
        button::Style {
            background: None,
            text_color: dark::FG2,
            border: border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: border::Radius::from(3.0),
            },
            shadow: iced::Shadow::default(),
        }
    }

    pub fn sidebar_item_active() -> button::Style {
        button::Style {
            background: Some(Background::Color(dark::BG1)),
            text_color: dark::FG0,
            border: border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: border::Radius::from(3.0),
            },
            shadow: iced::Shadow::default(),
        }
    }
}

/// Container styles module
pub mod container {
    use iced::widget::container;
    use iced::{Background, Color, border};
    use crate::ui::style::colors::dark;

    pub fn card() -> container::Style {
        container::Style {
            background: Some(Background::Color(dark::BG1)),
            text_color: Some(dark::FG1),
            border: border::Border {
                color: dark::BG2,
                width: 1.0,
                radius: border::Radius::from(8.0),
            },
            shadow: iced::Shadow::default(),
        }
    }
}

/// Scrollable styles module
pub mod scrollable {
    use iced::widget::scrollable;
    use iced::{Background, Color, border};
    use crate::ui::style::colors::dark;

    pub fn default() -> scrollable::Style {
        scrollable::Style {
            container: iced::widget::container::Style {
                background: None,
                text_color: None,
                border: border::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: border::Radius::from(0.0),
                },
                shadow: iced::Shadow::default(),
            },
            vertical_rail: scrollable::Rail {
                background: Some(Background::Color(dark::BG1)),
                border: border::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: border::Radius::from(2.0),
                },
                scroller: scrollable::Scroller {
                    color: dark::BG3,
                    border: border::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: border::Radius::from(2.0),
                    },
                },
            },
            horizontal_rail: scrollable::Rail {
                background: Some(Background::Color(dark::BG1)),
                border: border::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: border::Radius::from(2.0),
                },
                scroller: scrollable::Scroller {
                    color: dark::BG3,
                    border: border::Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: border::Radius::from(2.0),
                    },
                },
            },
            gap: None,
        }
    }
}

/// Text input styles module
pub mod text_input {
    use iced::widget::text_input;
    use iced::{Background, Color, border};
    use crate::ui::style::colors::dark;

    pub fn default() -> text_input::Style {
        text_input::Style {
            background: Background::Color(dark::BG1),
            border: border::Border {
                color: dark::BG3,
                width: 1.0,
                radius: border::Radius::from(5.0),
            },
            icon: dark::FG2,
            placeholder: dark::FG3,
            value: dark::FG1,
            selection: dark::BLUE,
        }
    }
}

pub struct GruvboxStyle;

impl GruvboxStyle {
    pub fn container() -> container::Style {
        container::Style {
            background: Some(Background::Color(colors::dark::BG0)),
            text_color: Some(colors::dark::FG1),
            border: border::Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: border::Radius::from(0.0),
            },
            shadow: iced::Shadow::default(),
        }
    }
    
    pub fn content() -> container::Style {
        container::card()
    }
    
    pub fn button() -> button::Style {
        button::primary()
    }
}

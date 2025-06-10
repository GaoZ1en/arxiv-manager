use iced::widget::scrollable::{Catalog, Status, Style, StyleFn};
use iced::{Background, Border};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> StyleFn<'a, Self> {
        Box::new(primary)
    }

    fn style(&self, class: &StyleFn<'_, Self>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn primary(theme: &Theme, _status: Status) -> Style {
    Style {
        container: iced::widget::container::Style {
            background: Some(Background::Color(theme.colors.general.background)),
            border: Border::default(),
            ..Default::default()
        },
        vertical_rail: iced::widget::scrollable::Rail {
            background: Some(Background::Color(theme.colors.general.surface)),
            border: Border {
                width: 1.0,
                color: theme.colors.general.border,
                radius: 2.0.into(),
            },
            scroller: iced::widget::scrollable::Scroller {
                color: theme.colors.general.border,
                border: Border {
                    width: 1.0,
                    color: theme.colors.general.border,
                    radius: 2.0.into(),
                },
            },
        },
        horizontal_rail: iced::widget::scrollable::Rail {
            background: Some(Background::Color(theme.colors.general.surface)),
            border: Border {
                width: 1.0,
                color: theme.colors.general.border,
                radius: 2.0.into(),
            },
            scroller: iced::widget::scrollable::Scroller {
                color: theme.colors.general.border,
                border: Border {
                    width: 1.0,
                    color: theme.colors.general.border,
                    radius: 2.0.into(),
                },
            },
        },
        gap: None,
    }
}

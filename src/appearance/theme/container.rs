use iced::widget::container::{Catalog, Style, StyleFn};
use iced::{Background, Border};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> StyleFn<'a, Self> {
        Box::new(primary)
    }

    fn style(&self, class: &StyleFn<'_, Self>) -> Style {
        class(self)
    }
}

pub fn primary(theme: &Theme) -> Style {
    Style {
        background: Some(Background::Color(theme.colors.general.background)),
        border: Border::default(),
        ..Default::default()
    }
}

pub fn surface(theme: &Theme) -> Style {
    Style {
        background: Some(Background::Color(theme.colors.general.surface)),
        border: Border {
            width: 1.0,
            color: theme.colors.general.border,
            radius: 6.0.into(),
        },
        ..Default::default()
    }
}

pub fn sidebar(theme: &Theme) -> Style {
    Style {
        background: Some(Background::Color(theme.colors.general.surface)),
        border: Border {
            width: 1.0,
            color: theme.colors.general.border,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

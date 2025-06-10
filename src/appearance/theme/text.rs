use iced::widget::text::{Catalog, Style, StyleFn};

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
        color: Some(theme.colors.text.primary),
    }
}

pub fn secondary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors.text.secondary),
    }
}

pub fn success(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors.text.success),
    }
}

pub fn error(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors.text.error),
    }
}

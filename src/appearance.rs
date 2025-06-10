// Appearance and theming system inspired by halloy
pub mod theme;

pub use theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Light,
    Dark,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Dark
    }
}

pub fn theme() -> Theme {
    Theme::default()
}

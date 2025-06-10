pub mod theme;
pub mod search;
pub mod library;
pub mod downloads;
pub mod settings;
pub mod app;
pub mod simple;

pub use theme::{gruvbox_dark, gruvbox_light};
pub use search::search_view;
pub use library::library_view;
pub use downloads::downloads_view;
pub use settings::settings_view;
pub use app::run;

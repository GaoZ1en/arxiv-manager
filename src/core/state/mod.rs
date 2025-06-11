// 状态管理模块 - 将应用状态分解为专门的子状态

pub mod ui_state;
pub mod search_state;
pub mod download_state;

// 重新导出所有状态类型
pub use ui_state::UiState;
pub use search_state::SearchState;
pub use download_state::DownloadState;

/// 应用主状态 - 组合所有子状态
#[derive(Debug)]
pub struct AppState {
    pub ui: UiState,
    pub search: SearchState,
    pub download: DownloadState,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            ui: UiState::new(),
            search: SearchState::new(),
            download: DownloadState::new(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

use self::state::AppState;

pub mod state;
pub mod ui;

/// The main application, containing the state
pub struct App {
    /// State
    state: AppState,
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let state = AppState::default();

        Self { state }
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }
}

use crate::prelude::*;

/// State of the application.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect, States)]
pub enum AppState {
    /// Application is loading the localization.
    #[default]
    LoadingLocalization,

    /// Application is in the main menu.
    MainMenu,

    /// Application is in the settings menu.
    SettingsMenu,

    /// Application is in the game more selection screen.
    GameModeSelectionScreen,

    /// Application is in the player selection screen.
    PlayerSelectionScreen,

    /// Application is in the enemy selection screen.
    EnemySelectionScreen,

    /// Application is in the game.
    Game,
}

impl AppState {
    /// Gets the previous application state for when the user wants to go back.
    pub fn previous(self) -> Option<AppState> {
        match self {
            AppState::LoadingLocalization | AppState::MainMenu => None,
            AppState::SettingsMenu => Some(AppState::MainMenu),
            AppState::GameModeSelectionScreen => Some(AppState::MainMenu),
            AppState::PlayerSelectionScreen => Some(AppState::GameModeSelectionScreen),
            AppState::EnemySelectionScreen => Some(AppState::PlayerSelectionScreen),
            AppState::Game => None,
        }
    }
}

impl Display for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppState::LoadingLocalization => write!(f, "the localization loading"),
            AppState::MainMenu => write!(f, "the main menu"),
            AppState::SettingsMenu => write!(f, "the settings menu"),
            AppState::GameModeSelectionScreen => write!(f, "the game mode selection screen"),
            AppState::PlayerSelectionScreen => write!(f, "the player selection screen"),
            AppState::EnemySelectionScreen => write!(f, "the enemy selection screen"),
            AppState::Game => write!(f, "the game"),
        }
    }
}

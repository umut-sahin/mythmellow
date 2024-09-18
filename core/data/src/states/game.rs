use crate::prelude::*;

/// State of the game.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect, SubStates)]
#[source(AppState = AppState::Game)]
pub enum GameState {
    /// Game will transition to another state.
    #[default]
    Transition,

    /// Game is being initialized, which happens only once at the beginning of the game.
    Initialization,

    /// Chapter is being loaded, which happens at each the beginning of each chapter.
    Loading,

    /// Game is being played.
    Playing,

    /// Game is paused.
    Paused,

    /// Game is in the settings menu.
    SettingsMenu,

    /// Game is in the level up screen.
    LevelUpScreen,

    /// Game is in the market.
    Market,

    /// Game is being restarted.
    Restart,

    /// Chapter is won.
    Won,

    /// Game is over.
    Over,
}

impl Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameState::Transition => {
                write!(f, "transitioning")
            },
            GameState::Initialization => {
                write!(f, "initializing the game")
            },
            GameState::Loading => {
                write!(f, "loading the chapter")
            },
            GameState::Playing => {
                write!(f, "starting the gameplay")
            },
            GameState::Paused => {
                write!(f, "transitioning to the pause menu")
            },
            GameState::SettingsMenu => {
                write!(f, "transitioning to the settings menu")
            },
            GameState::LevelUpScreen => {
                write!(f, "transitioning to the level up screen")
            },
            GameState::Market => {
                write!(f, "transitioning to the market")
            },
            GameState::Restart => {
                write!(f, "restarting the game")
            },
            GameState::Won => {
                write!(f, "chapter is won")
            },
            GameState::Over => {
                write!(f, "transitioning to the game over screen")
            },
        }
    }
}

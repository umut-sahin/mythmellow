use {
    super::*,
    mythmallow_core_dependencies::*,
};

/// States in which the player is in a menu.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect)]
pub struct InMenu;

impl ComputedStates for InMenu {
    type SourceStates = (AppState, Option<GameState>);

    fn compute((app_state, game_state): (AppState, Option<GameState>)) -> Option<InMenu> {
        match app_state {
            AppState::LoadingLocalization => None,

            AppState::MainMenu
            | AppState::SettingsMenu
            | AppState::GameModeSelectionScreen
            | AppState::PlayerSelectionScreen
            | AppState::EnemySelectionScreen => Some(InMenu),

            AppState::Game => {
                match game_state.unwrap_or_default() {
                    GameState::Transition
                    | GameState::Initialization
                    | GameState::Loading
                    | GameState::Playing
                    | GameState::Restart
                    | GameState::Won => None,

                    GameState::Paused
                    | GameState::SettingsMenu
                    | GameState::LevelUpScreen
                    | GameState::Market
                    | GameState::Over => Some(InMenu),
                }
            },
        }
    }
}

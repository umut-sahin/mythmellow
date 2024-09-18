use crate::prelude::*;

/// States in which the application is in the settings menu.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect)]
pub struct InSettingsMenu;

impl ComputedStates for InSettingsMenu {
    type SourceStates = (AppState, Option<GameState>);

    fn compute((app_state, game_state): (AppState, Option<GameState>)) -> Option<InSettingsMenu> {
        match app_state {
            AppState::SettingsMenu => Some(InSettingsMenu),
            AppState::Game if game_state == Some(GameState::SettingsMenu) => Some(InSettingsMenu),
            _ => None,
        }
    }
}

use crate::prelude::*;

/// States in which the game is going on.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect)]
pub struct InGame;

impl ComputedStates for InGame {
    type SourceStates = GameState;

    fn compute(game_state: GameState) -> Option<InGame> {
        match game_state {
            GameState::Transition
            | GameState::Initialization
            | GameState::Loading
            | GameState::Playing
            | GameState::Paused
            | GameState::SettingsMenu
            | GameState::LevelUpScreen
            | GameState::Market
            | GameState::Won => Some(InGame),

            GameState::Over | GameState::Restart => None,
        }
    }
}

use crate::prelude::*;

/// States in which the chapter is going on.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Reflect)]
pub struct InChapter;

impl ComputedStates for InChapter {
    type SourceStates = GameState;

    fn compute(game_state: GameState) -> Option<InChapter> {
        match game_state {
            GameState::Transition
            | GameState::Initialization
            | GameState::Loading
            | GameState::Playing
            | GameState::Paused
            | GameState::SettingsMenu
            | GameState::LevelUpScreen
            | GameState::Market => Some(InChapter),

            GameState::Won | GameState::Over | GameState::Restart => None,
        }
    }
}

//! State resources.

use crate::prelude::*;


/// Resource for the state stack of the game state.
///
/// Game state stack is used for managing the flow of the game.
/// For example, if it's, `[Play, Paused, Settings]`, it means
/// that the game is currently on the settings menu, and once the
/// back action is triggered, it'll transition to the pause menu.
/// If the back action is triggered again, it'll transition
/// to the game.
///
/// One state can appear multiple times in the game state stack.
/// This enables complex game state flows, such as showing
/// the level up screen `n` times back to back, if the player
/// levelled up `n` times in the past chapter. For example, if it's
/// `[Loading, Market, LevelUp, LevelUp, LevelUp, Paused]`, it means:
/// - the game is paused
/// - once it's resumed, level up screen would be shown 3 times
/// - than the market would be shown
/// - lastly the next chapter would start loading
#[derive(Debug, Default, Deref, DerefMut, Reflect, Resource)]
pub struct GameStateStack(pub Vec<GameState>);

impl GameStateStack {
    /// Transitions to a new game state.
    pub fn transition(&mut self, state: GameState) {
        self.0.pop();
        self.0.push(state);
    }
}


/// Resource for the result of the game.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect, Resource)]
pub enum GameResult {
    /// Game is won.
    Won,
    /// Game is lost.
    Lost,
}

//! Game mode selection screen components.

use crate::prelude::*;


/// Component for the game mode buttons in the game mode selection screen.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct GameModeSelectionScreenGameModeButton {
    /// Index of the game mode in the game mode registry the button represents.
    pub game_mode_index: usize,
}

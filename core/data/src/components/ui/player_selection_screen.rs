//! Player selection screen components.

use crate::prelude::*;


/// Component for the player buttons in the player selection screen.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct PlayerSelectionScreenPlayerButton {
    /// Index of the mythology in the player registry the button represents.
    pub mythology_index: usize,

    /// Index of the player in the mythology the button represents.
    pub player_index: usize,
}

//! Enemy selection screen components.

use crate::prelude::*;


/// Component for the enemy buttons in the enemy selection screen.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct EnemySelectionScreenEnemyButton {
    /// Index of the enemy pack in the enemy registry the button represents.
    pub enemy_pack_index: usize,
}

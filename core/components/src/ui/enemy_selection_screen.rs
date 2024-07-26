//! Enemy selection screen components.

use mythmallow_core_dependencies::*;


/// Component for the enemy selection screen.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct EnemySelectionScreen;


/// Component for the enemy buttons in the enemy selection screen.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct EnemySelectionScreenEnemyButton {
    /// Index of the enemy pack in the enemy registry the button represents.
    pub enemy_pack_index: usize,
}


/// Component for the back button in the enemy selection screen.
#[derive(Clone, Component, Debug, Default, Reflect)]
pub struct EnemySelectionScreenBackButton;

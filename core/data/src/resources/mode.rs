//! Game mode resources.

use crate::prelude::*;


/// Resource for the index of the selected game mode in the game mode registry.
#[derive(Debug, Default, Deref, Resource, Reflect)]
pub struct GameModeIndex(pub usize);


/// Resource for the current game mode.
#[derive(Debug, Default, Deref, Resource, Reflect)]
pub struct GameMode<M: IGameMode>(pub M);


/// Resource for the level structure of the player for the selected game mode.
#[derive(Clone, Resource)]
pub struct PlayerLevelStructure {
    /// Max level that can be reached in the game mode.
    pub max_level: Option<Level>,
    /// Function to calculate the required experience for each level.
    pub required_experience_calculator: fn(&World, Level) -> Experience,
}

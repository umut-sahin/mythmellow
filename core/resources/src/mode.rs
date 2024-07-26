//! Game mode resources.

use {
    mythmallow_core_dependencies::*,
    mythmallow_core_interfaces::*,
};


/// Resource for the index of the selected game mode in the game mode registry.
#[derive(Debug, Default, Deref, Resource, Reflect)]
pub struct GameModeIndex(pub usize);


/// Resource for the current game mode.
#[derive(Debug, Default, Deref, Resource, Reflect)]
pub struct GameMode<M: IGameMode>(pub M);

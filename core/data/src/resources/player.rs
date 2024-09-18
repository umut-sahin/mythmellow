//! Player resources.

use crate::prelude::*;


/// Resource for the index of the selected mythology in the player registry.
#[derive(Debug, Deref, Resource, Reflect)]
pub struct MythologyIndex(pub usize);


/// Resource for the index of the selected player in the selected mythology.
#[derive(Debug, Deref, Resource, Reflect)]
pub struct PlayerIndex(pub usize);


/// Resource for the status of the god mode.
#[derive(Clone, Copy, Debug, Reflect, Resource)]
pub struct GodMode {
    /// Whether the god mode is enabled.
    pub is_enabled: bool,
}

impl Default for GodMode {
    fn default() -> GodMode {
        GodMode { is_enabled: false }
    }
}

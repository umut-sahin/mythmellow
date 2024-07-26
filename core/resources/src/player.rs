//! Player resources.

use mythmallow_core_dependencies::*;


/// Resource for the index of the selected mythology in the player registry.
#[derive(Debug, Deref, Resource, Reflect)]
pub struct MythologyIndex(pub usize);


/// Resource for the index of the selected player in the selected mythology.
#[derive(Debug, Deref, Resource, Reflect)]
pub struct PlayerIndex(pub usize);

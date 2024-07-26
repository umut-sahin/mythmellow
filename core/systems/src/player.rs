//! Player systems.

use {
    mythmallow_core_dependencies::*,
    mythmallow_core_registries::all::*,
    mythmallow_core_resources::all::*,
};


/// Spawns the player.
pub fn spawn_player(world: &mut World) {
    let player_registry = world.resource::<PlayerRegistry>();
    let mythology_index = world.resource::<MythologyIndex>();
    let player_index = world.resource::<PlayerIndex>();
    let player = player_registry[mythology_index.0].players[player_index.0].clone();

    log::info!("spawning the player {:?}", player.id());
    player.spawn(world);
}

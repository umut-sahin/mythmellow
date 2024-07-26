//! Game mode systems.

use {
    mythmallow_core_dependencies::*,
    mythmallow_core_registries::all::*,
    mythmallow_core_resources::all::*,
};


/// Initializes the selected game mode.
pub fn initialize_game_mode(world: &mut World) {
    let game_mode_registry = world.resource::<GameModeRegistry>();
    let game_mode_index = world.resource::<GameModeIndex>();
    let game_mode = game_mode_registry[game_mode_index.0].clone();

    log::info!("initializing the {:?} mode", game_mode.id());
    game_mode.initialize(world);
}

/// Deinitializes the selected game mode.
pub fn deinitialize_game_mode(world: &mut World) {
    let game_mode_registry = world.resource::<GameModeRegistry>();
    let game_mode_index = world.resource::<GameModeIndex>();
    let game_mode = game_mode_registry[game_mode_index.0].clone();

    log::info!("deinitializing the {:?} mode", game_mode.id());
    game_mode.deinitialize(world);
}

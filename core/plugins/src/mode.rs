use {
    mythmallow_core_dependencies::*,
    mythmallow_core_registries::all::*,
    mythmallow_core_resources::all::*,
    mythmallow_core_sets::*,
    mythmallow_core_states::*,
    mythmallow_core_systems::{
        mode::*,
        utility::*,
    },
};

/// Plugin for managing the game modes.
pub struct ModePlugin;

impl Plugin for ModePlugin {
    fn build(&self, app: &mut App) {
        // Register resources.
        app.register_type::<GameModeIndex>();

        // Initialize registry.
        app.init_resource::<GameModeRegistry>();

        // Add systems.
        app.add_systems(
            OnEnter(GameState::Initialization),
            initialize_game_mode.in_set(InitializationSystems::First),
        );
        app.add_systems(
            OnEnter(GameState::Restart),
            deinitialize_game_mode.in_set(RestartSystems::Last),
        );
        app.add_systems(
            OnExit(AppState::Game),
            (deinitialize_game_mode, remove_resource::<GameModeIndex>).chain(),
        );
    }
}

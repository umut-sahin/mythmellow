use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_registries::all::*,
    mythmallow_core_resources::all::*,
    mythmallow_core_sets::*,
    mythmallow_core_states::*,
    mythmallow_core_systems::{
        player::*,
        utility::*,
    },
};

/// Plugin for managing the players of the game.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Register resources.
        app.register_type::<PlayerIndex>();

        // Register components.
        app.register_type::<Player>();
        app.register_type::<PlayerHitBox>();

        // Initialize registry.
        app.init_resource::<PlayerRegistry>();

        // Add systems.
        app.add_systems(
            OnEnter(GameState::Initialization),
            spawn_player.in_set(InitializationSystems::Player),
        );
        app.add_systems(
            OnExit(AppState::Game),
            (remove_resource::<MythologyIndex>, remove_resource::<PlayerIndex>),
        );
    }
}

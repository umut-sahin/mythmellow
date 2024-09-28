use crate::{
    prelude::*,
    systems::{
        player::*,
        utility::*,
    },
};

/// Plugin for managing the players of the game.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // Register resources.
        app.register_type::<MythologyIndex>();
        app.register_type::<PlayerIndex>();
        app.register_type::<GodMode>();

        // Register components.
        app.register_type::<Player>();
        app.register_type::<PlayerHitBox>();

        // Initialize registry.
        app.init_resource::<PlayerRegistry>();

        // Insert resources.
        let arguments = app.world().resource::<Arguments>();
        app.insert_resource(GodMode { is_enabled: arguments.enable_god_mode });

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
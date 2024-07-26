use {
    mythmallow_core_components::all::*,
    mythmallow_core_dependencies::*,
    mythmallow_core_interfaces::*,
    mythmallow_core_registries::all::*,
    mythmallow_core_resources::all::*,
    mythmallow_core_sets::*,
    mythmallow_core_states::*,
    mythmallow_core_systems::{
        enemy::*,
        utility::*,
    },
};

/// Plugin for managing the enemies of the game.
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // Register resources.
        app.register_type::<EnemyPackIndex>();

        // Register components.
        app.register_type::<Enemy>();
        app.register_type::<EnemyHitBox>();

        // Initialize registry.
        app.init_resource::<EnemyRegistry>();

        // Add systems.
        app.add_systems(
            OnEnter(GameState::Loading),
            (initialize_enemy_counter, initialize_enemy_spawn_pattern)
                .in_set(LoadingSystems::Enemy),
        );
        app.add_systems(Update, spawn_enemies.in_set(GameplaySystems::Enemy));
        app.add_systems(
            OnExit(InChapter),
            (remove_resource::<EnemyCounter>, remove_resource::<EnemySpawnPattern>),
        );
        app.add_systems(OnExit(AppState::Game), remove_resource::<EnemyPackIndex>);
    }
}

use crate::{
    prelude::*,
    systems::{
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
